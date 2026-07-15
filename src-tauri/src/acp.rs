//! ACP client for `grok agent --always-approve stdio`.
//!
//! Lifecycle: connect (initialize + authenticate + session/new) → prompt turns
//! with streamed `session/update` events → disconnect.

use crate::grok;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const EVENT_UPDATE: &str = "acp://update";
const EVENT_STATUS: &str = "acp://status";
const EVENT_ERROR: &str = "acp://error";
const EVENT_LOG: &str = "acp://log";

/// Shared app-level ACP handle.
pub struct AcpState {
    inner: Mutex<Option<LiveSession>>,
}

impl AcpState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }
}

struct LiveSession {
    child: Child,
    stdin: Arc<Mutex<ChildStdin>>,
    session_id: String,
    cwd: String,
    next_id: Arc<AtomicU64>,
    pending: Arc<Mutex<HashMap<u64, Sender<Result<Value, String>>>>>,
    running: Arc<AtomicBool>,
    reader: Option<thread::JoinHandle<()>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResult {
    pub session_id: String,
    pub cwd: String,
    pub model_id: Option<String>,
    pub binary_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptResult {
    pub stop_reason: Option<String>,
    pub meta: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInfo {
    pub connected: bool,
    pub session_id: Option<String>,
    pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusPayload {
    connected: bool,
    session_id: Option<String>,
    cwd: Option<String>,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePayload {
    /// Raw sessionUpdate kind (e.g. agent_message_chunk).
    kind: String,
    /// Full update object from the agent.
    update: Value,
    session_id: String,
}

fn emit_log(app: &AppHandle, msg: impl Into<String>) {
    let _ = app.emit(EVENT_LOG, msg.into());
}

fn emit_error(app: &AppHandle, msg: impl Into<String>) {
    let _ = app.emit(EVENT_ERROR, msg.into());
}

fn emit_status(app: &AppHandle, payload: StatusPayload) {
    let _ = app.emit(EVENT_STATUS, payload);
}

/// Connect: spawn agent, initialize, authenticate, create session.
pub fn connect(app: AppHandle, state: &AcpState, cwd: String) -> Result<ConnectResult, String> {
    disconnect(state)?;

    let cwd_path = PathBuf::from(&cwd);
    if !cwd_path.is_dir() {
        return Err(format!("Working directory is not a folder: {cwd}"));
    }

    let bin = grok::resolve_binary().ok_or_else(|| {
        "Grok CLI not found. Install from https://x.ai/cli or set GROK_BINARY.".to_string()
    })?;
    let binary_path = bin.display().to_string();

    emit_log(&app, format!("Spawning `{binary_path} agent --always-approve stdio`"));

    let mut child = Command::new(&bin)
        .args(["agent", "--always-approve", "stdio"])
        .current_dir(&cwd_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn grok agent: {e}"))?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Agent stdin missing".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Agent stdout missing".to_string())?;

    // Drain stderr so the process never blocks on a full pipe.
    if let Some(stderr) = child.stderr.take() {
        let app_err = app.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                if !line.trim().is_empty() {
                    emit_log(&app_err, format!("[stderr] {line}"));
                }
            }
        });
    }

    let stdin = Arc::new(Mutex::new(stdin));
    let next_id = Arc::new(AtomicU64::new(1));
    let pending: Arc<Mutex<HashMap<u64, Sender<Result<Value, String>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let running = Arc::new(AtomicBool::new(true));

    // Reader thread: demux responses / notifications.
    let pending_r = Arc::clone(&pending);
    let running_r = Arc::clone(&running);
    let app_r = app.clone();
    let session_id_holder: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let session_id_r = Arc::clone(&session_id_holder);

    let reader = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        while running_r.load(Ordering::SeqCst) {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    emit_error(&app_r, "Agent process closed stdout (disconnected)");
                    break;
                }
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let msg: Value = match serde_json::from_str(trimmed) {
                        Ok(v) => v,
                        Err(e) => {
                            emit_log(&app_r, format!("Bad JSON from agent: {e}: {trimmed}"));
                            continue;
                        }
                    };
                    handle_incoming(&app_r, &pending_r, &session_id_r, msg);
                }
                Err(e) => {
                    if running_r.load(Ordering::SeqCst) {
                        emit_error(&app_r, format!("Agent stdout read error: {e}"));
                    }
                    break;
                }
            }
        }
        running_r.store(false, Ordering::SeqCst);
    });

    // Temporary session shell for request() before we know session_id.
    let bootstrap = Bootstrap {
        stdin: Arc::clone(&stdin),
        next_id: Arc::clone(&next_id),
        pending: Arc::clone(&pending),
        running: Arc::clone(&running),
    };

    // initialize
    let init = bootstrap.request(
        "initialize",
        json!({
            "protocolVersion": 1,
            "clientCapabilities": {},
            "clientInfo": {
                "name": "grok-desktop",
                "version": env!("CARGO_PKG_VERSION")
            }
        }),
        Duration::from_secs(20),
    )?;

    let default_auth = init
        .pointer("/_meta/defaultAuthMethodId")
        .and_then(|v| v.as_str())
        .unwrap_or("cached_token")
        .to_string();

    // authenticate
    let _auth = bootstrap.request(
        "authenticate",
        json!({ "methodId": default_auth }),
        Duration::from_secs(30),
    )?;

    // session/new
    let session = bootstrap.request(
        "session/new",
        json!({
            "cwd": cwd_path.to_string_lossy(),
            "mcpServers": []
        }),
        Duration::from_secs(60),
    )?;

    let session_id = session
        .get("sessionId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("session/new missing sessionId: {session}"))?
        .to_string();

    let model_id = session
        .pointer("/models/currentModelId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    *session_id_holder.lock().map_err(|e| e.to_string())? = Some(session_id.clone());

    let live = LiveSession {
        child,
        stdin,
        session_id: session_id.clone(),
        cwd: cwd.clone(),
        next_id,
        pending,
        running,
        reader: Some(reader),
    };

    *state.inner.lock().map_err(|e| e.to_string())? = Some(live);

    emit_status(
        &app,
        StatusPayload {
            connected: true,
            session_id: Some(session_id.clone()),
            cwd: Some(cwd.clone()),
            message: format!(
                "Connected — session {}{}",
                &session_id[..8.min(session_id.len())],
                model_id
                    .as_ref()
                    .map(|m| format!(" · {m}"))
                    .unwrap_or_default()
            ),
        },
    );

    Ok(ConnectResult {
        session_id,
        cwd,
        model_id,
        binary_path,
    })
}

/// Send a user prompt; streams session/update events until the turn completes.
pub fn prompt(app: AppHandle, state: &AcpState, text: String) -> Result<PromptResult, String> {
    let text = text.trim().to_string();
    if text.is_empty() {
        return Err("Prompt is empty".into());
    }

    let (session_id, stdin, next_id, pending, running) = {
        let guard = state.inner.lock().map_err(|e| e.to_string())?;
        let live = guard
            .as_ref()
            .ok_or_else(|| "Not connected. Connect to a project folder first.".to_string())?;
        if !live.running.load(Ordering::SeqCst) {
            return Err("Agent process is no longer running. Disconnect and reconnect.".into());
        }
        (
            live.session_id.clone(),
            Arc::clone(&live.stdin),
            Arc::clone(&live.next_id),
            Arc::clone(&live.pending),
            Arc::clone(&live.running),
        )
    };

    emit_log(&app, format!("session/prompt ({session_id})"));

    let bootstrap = Bootstrap {
        stdin,
        next_id,
        pending,
        running,
    };

    let result = bootstrap.request(
        "session/prompt",
        json!({
            "sessionId": session_id,
            "prompt": [{ "type": "text", "text": text }]
        }),
        Duration::from_secs(600),
    )?;

    let stop_reason = result
        .get("stopReason")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(PromptResult {
        stop_reason,
        meta: result.get("_meta").cloned(),
    })
}

pub fn disconnect(state: &AcpState) -> Result<(), String> {
    let mut guard = state.inner.lock().map_err(|e| e.to_string())?;
    if let Some(mut live) = guard.take() {
        live.running.store(false, Ordering::SeqCst);
        // Best-effort close stdin so the agent exits.
        drop(live.stdin.lock().ok());
        let _ = live.child.kill();
        let _ = live.child.wait();
        if let Some(handle) = live.reader.take() {
            let _ = handle.join();
        }
    }
    Ok(())
}

pub fn connection_info(state: &AcpState) -> Result<ConnectionInfo, String> {
    let guard = state.inner.lock().map_err(|e| e.to_string())?;
    match guard.as_ref() {
        Some(live) if live.running.load(Ordering::SeqCst) => Ok(ConnectionInfo {
            connected: true,
            session_id: Some(live.session_id.clone()),
            cwd: Some(live.cwd.clone()),
        }),
        _ => Ok(ConnectionInfo {
            connected: false,
            session_id: None,
            cwd: None,
        }),
    }
}

// ─── Internals ───────────────────────────────────────────────────────────────

struct Bootstrap {
    stdin: Arc<Mutex<ChildStdin>>,
    next_id: Arc<AtomicU64>,
    pending: Arc<Mutex<HashMap<u64, Sender<Result<Value, String>>>>>,
    running: Arc<AtomicBool>,
}

impl Bootstrap {
    fn request(&self, method: &str, params: Value, timeout: Duration) -> Result<Value, String> {
        if !self.running.load(Ordering::SeqCst) {
            return Err("Agent is not running".into());
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx): (
            Sender<Result<Value, String>>,
            Receiver<Result<Value, String>>,
        ) = mpsc::channel();

        self.pending
            .lock()
            .map_err(|e| e.to_string())?
            .insert(id, tx);

        let msg = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        {
            let mut stdin = self.stdin.lock().map_err(|e| e.to_string())?;
            writeln!(stdin, "{msg}").map_err(|e| format!("Write to agent failed: {e}"))?;
            stdin
                .flush()
                .map_err(|e| format!("Flush to agent failed: {e}"))?;
        }

        match rx.recv_timeout(timeout) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(e),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.pending.lock().ok().map(|mut p| p.remove(&id));
                Err(format!(
                    "Timeout waiting for `{method}` after {}s",
                    timeout.as_secs()
                ))
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(format!("Agent channel closed while waiting for `{method}`"))
            }
        }
    }
}

fn handle_incoming(
    app: &AppHandle,
    pending: &Mutex<HashMap<u64, Sender<Result<Value, String>>>>,
    session_id: &Mutex<Option<String>>,
    msg: Value,
) {
    // Response to our request
    if let Some(id) = msg.get("id").and_then(|v| v.as_u64()).or_else(|| {
        msg.get("id")
            .and_then(|v| v.as_i64())
            .map(|i| i as u64)
    }) {
        // Agent may send requests TO us with id+method (permissions, fs, …).
        if let Some(method) = msg.get("method").and_then(|v| v.as_str()) {
            handle_agent_request(app, method, &msg);
            return;
        }

        let result = if let Some(err) = msg.get("error") {
            Err(err.to_string())
        } else if let Some(result) = msg.get("result") {
            Ok(result.clone())
        } else {
            Err(format!("Response id={id} missing result/error"))
        };

        if let Ok(mut map) = pending.lock() {
            if let Some(tx) = map.remove(&id) {
                let _ = tx.send(result);
            }
        }
        return;
    }

    // Notification
    if let Some(method) = msg.get("method").and_then(|v| v.as_str()) {
        match method {
            "session/update" => {
                let params = msg.get("params").cloned().unwrap_or(json!({}));
                let update = params
                    .get("update")
                    .cloned()
                    .unwrap_or_else(|| params.clone());
                let kind = update
                    .get("sessionUpdate")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let sid = session_id
                    .lock()
                    .ok()
                    .and_then(|g| g.clone())
                    .unwrap_or_default();
                let _ = app.emit(
                    EVENT_UPDATE,
                    UpdatePayload {
                        kind,
                        update,
                        session_id: sid,
                    },
                );
            }
            other => {
                // x.ai/* noise → light log only for interesting ones
                if other.starts_with("_x.ai/") || other.starts_with("x.ai/") {
                    // suppress high-volume chatter in the UI log
                } else {
                    emit_log(app, format!("notif {other}"));
                }
            }
        }
    }
}

fn handle_agent_request(app: &AppHandle, method: &str, msg: &Value) {
    // For the MVP we start with --always-approve, so permission requests should
    // be rare. Still log anything unexpected that carries an id.
    emit_log(
        app,
        format!(
            "Agent request `{method}` (id={:?}) — not handled by client yet",
            msg.get("id")
        ),
    );
}
