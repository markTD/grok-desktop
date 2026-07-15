//! ACP client for `grok agent stdio`.
//!
//! Lifecycle: connect (initialize + authenticate + session/new|load) → prompt
//! turns with streamed `session/update` events → optional permission replies →
//! disconnect.

use crate::grok;
use serde::{Deserialize, Serialize};
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
const EVENT_PERMISSION: &str = "acp://permission";

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
    /// JSON-RPC id of the in-flight `session/prompt`, if any.
    active_prompt_id: Arc<Mutex<Option<u64>>>,
    reader: Option<thread::JoinHandle<()>>,
    always_approve: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOptions {
    pub cwd: String,
    /// When true, pass `--always-approve` to the agent.
    #[serde(default)]
    pub always_approve: bool,
    /// Resume an existing session via `session/load`.
    pub resume_session_id: Option<String>,
    /// Extra rules appended to the system prompt (`session/new` `_meta.rules`).
    pub rules: Option<String>,
    /// Model id for `grok agent -m`.
    pub model: Option<String>,
    /// Reasoning effort for `grok agent --effort`.
    pub effort: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResult {
    pub session_id: String,
    pub cwd: String,
    pub model_id: Option<String>,
    pub binary_path: String,
    pub resumed: bool,
    pub always_approve: bool,
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
    pub always_approve: bool,
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
    kind: String,
    update: Value,
    session_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PermissionPayload {
    request_id: u64,
    tool_call: Option<Value>,
    options: Vec<Value>,
    raw: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionReply {
    pub request_id: u64,
    /// `selected` | `cancelled`
    pub outcome: String,
    pub option_id: Option<String>,
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

/// Connect: spawn agent, initialize, authenticate, create or load session.
pub fn connect(app: AppHandle, state: &AcpState, opts: ConnectOptions) -> Result<ConnectResult, String> {
    disconnect(state)?;

    let cwd = opts.cwd.trim().to_string();
    let cwd_path = PathBuf::from(&cwd);
    if !cwd_path.is_dir() {
        return Err(format!("Working directory is not a folder: {cwd}"));
    }

    let bin = grok::resolve_binary().ok_or_else(|| {
        "Grok CLI not found. Install from https://x.ai/cli or set GROK_BINARY.".to_string()
    })?;
    let binary_path = bin.display().to_string();

    let mut args = vec!["agent".to_string()];
    if let Some(model) = opts
        .model
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        args.push("-m".into());
        args.push(model.to_string());
    }
    if let Some(effort) = opts
        .effort
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        args.push("--effort".into());
        args.push(effort.to_string());
    }
    if opts.always_approve {
        args.push("--always-approve".into());
    }
    args.push("stdio".into());

    emit_log(
        &app,
        format!("Spawning `{binary_path} {}`", args.join(" ")),
    );

    let mut child = Command::new(&bin)
        .args(&args)
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
    let active_prompt_id = Arc::new(Mutex::new(None));
    let stdin_for_reader = Arc::clone(&stdin);

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
                    handle_incoming(&app_r, &pending_r, &session_id_r, &stdin_for_reader, msg);
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

    let bootstrap = Bootstrap {
        stdin: Arc::clone(&stdin),
        next_id: Arc::clone(&next_id),
        pending: Arc::clone(&pending),
        running: Arc::clone(&running),
        active_prompt_id: None,
    };

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

    let _auth = bootstrap.request(
        "authenticate",
        json!({ "methodId": default_auth }),
        Duration::from_secs(30),
    )?;

    let resumed = opts.resume_session_id.is_some();
    let (session_id, model_id) = if let Some(resume_id) = opts.resume_session_id.clone() {
        let session = bootstrap.request(
            "session/load",
            json!({
                "sessionId": resume_id,
                "cwd": cwd_path.to_string_lossy(),
                "mcpServers": []
            }),
            Duration::from_secs(60),
        )?;
        let sid = session
            .get("sessionId")
            .and_then(|v| v.as_str())
            .unwrap_or(&resume_id)
            .to_string();
        let model_id = session
            .pointer("/models/currentModelId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        (sid, model_id)
    } else {
        let mut params = json!({
            "cwd": cwd_path.to_string_lossy(),
            "mcpServers": []
        });
        if let Some(rules) = opts.rules.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            params["_meta"] = json!({ "rules": rules });
        }
        let session = bootstrap.request("session/new", params, Duration::from_secs(60))?;
        let sid = session
            .get("sessionId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("session/new missing sessionId: {session}"))?
            .to_string();
        let model_id = session
            .pointer("/models/currentModelId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        (sid, model_id)
    };

    *session_id_holder.lock().map_err(|e| e.to_string())? = Some(session_id.clone());

    let live = LiveSession {
        child,
        stdin,
        session_id: session_id.clone(),
        cwd: cwd.clone(),
        next_id,
        pending,
        running,
        active_prompt_id,
        reader: Some(reader),
        always_approve: opts.always_approve,
    };

    *state.inner.lock().map_err(|e| e.to_string())? = Some(live);

    let mode = if opts.always_approve {
        "auto-approve"
    } else {
        "ask on tools"
    };
    emit_status(
        &app,
        StatusPayload {
            connected: true,
            session_id: Some(session_id.clone()),
            cwd: Some(cwd.clone()),
            message: format!(
                "{} · {}{}",
                if resumed { "Resumed" } else { "Connected" },
                &session_id[..8.min(session_id.len())],
                model_id
                    .as_ref()
                    .map(|m| format!(" · {m} · {mode}"))
                    .unwrap_or_else(|| format!(" · {mode}"))
            ),
        },
    );

    Ok(ConnectResult {
        session_id,
        cwd,
        model_id,
        binary_path,
        resumed,
        always_approve: opts.always_approve,
    })
}

pub fn prompt(app: AppHandle, state: &AcpState, text: String) -> Result<PromptResult, String> {
    let text = text.trim().to_string();
    if text.is_empty() {
        return Err("Prompt is empty".into());
    }

    let (session_id, stdin, next_id, pending, running, active_prompt_id) = {
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
            Arc::clone(&live.active_prompt_id),
        )
    };

    emit_log(&app, format!("session/prompt ({session_id})"));

    let bootstrap = Bootstrap {
        stdin,
        next_id,
        pending,
        running,
        active_prompt_id: Some(active_prompt_id),
    };

    let result = bootstrap.request(
        "session/prompt",
        json!({
            "sessionId": session_id,
            "prompt": [{ "type": "text", "text": text }]
        }),
        Duration::from_secs(600),
    );

    // Always clear active prompt id when the wait ends.
    // (request() also clears on success path via Drop-ish logic below)
    if let Ok(guard) = state.inner.lock() {
        if let Some(live) = guard.as_ref() {
            if let Ok(mut ap) = live.active_prompt_id.lock() {
                *ap = None;
            }
        }
    }

    let result = result?;

    let stop_reason = result
        .get("stopReason")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(PromptResult {
        stop_reason,
        meta: result.get("_meta").cloned(),
    })
}

/// Cancel the in-flight prompt turn (`session/cancel` notification).
pub fn cancel_turn(app: AppHandle, state: &AcpState) -> Result<(), String> {
    let guard = state.inner.lock().map_err(|e| e.to_string())?;
    let live = guard
        .as_ref()
        .ok_or_else(|| "Not connected".to_string())?;

    let session_id = live.session_id.clone();
    let msg = json!({
        "jsonrpc": "2.0",
        "method": "session/cancel",
        "params": { "sessionId": session_id }
    });

    {
        let mut stdin = live.stdin.lock().map_err(|e| e.to_string())?;
        writeln!(stdin, "{msg}").map_err(|e| format!("Write cancel failed: {e}"))?;
        stdin
            .flush()
            .map_err(|e| format!("Flush cancel failed: {e}"))?;
    }

    // Unblock the waiting prompt request if still pending.
    if let Ok(mut ap) = live.active_prompt_id.lock() {
        if let Some(id) = ap.take() {
            if let Ok(mut map) = live.pending.lock() {
                if let Some(tx) = map.remove(&id) {
                    let _ = tx.send(Err("Turn cancelled".into()));
                }
            }
        }
    }

    emit_log(&app, format!("session/cancel ({session_id})"));
    Ok(())
}

/// Reply to a `session/request_permission` from the agent.
pub fn respond_permission(state: &AcpState, reply: PermissionReply) -> Result<(), String> {
    let guard = state.inner.lock().map_err(|e| e.to_string())?;
    let live = guard
        .as_ref()
        .ok_or_else(|| "Not connected".to_string())?;

    let result = match reply.outcome.as_str() {
        "cancelled" => json!({ "outcome": { "outcome": "cancelled" } }),
        _ => {
            let option_id = reply
                .option_id
                .ok_or_else(|| "optionId required for selected outcome".to_string())?;
            json!({
                "outcome": {
                    "outcome": "selected",
                    "optionId": option_id
                }
            })
        }
    };

    let msg = json!({
        "jsonrpc": "2.0",
        "id": reply.request_id,
        "result": result
    });

    let mut stdin = live.stdin.lock().map_err(|e| e.to_string())?;
    writeln!(stdin, "{msg}").map_err(|e| format!("Write permission reply failed: {e}"))?;
    stdin
        .flush()
        .map_err(|e| format!("Flush permission reply failed: {e}"))?;
    Ok(())
}

pub fn disconnect(state: &AcpState) -> Result<(), String> {
    let mut guard = state.inner.lock().map_err(|e| e.to_string())?;
    if let Some(mut live) = guard.take() {
        live.running.store(false, Ordering::SeqCst);
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
            always_approve: live.always_approve,
        }),
        _ => Ok(ConnectionInfo {
            connected: false,
            session_id: None,
            cwd: None,
            always_approve: false,
        }),
    }
}

// ─── Internals ───────────────────────────────────────────────────────────────

struct Bootstrap {
    stdin: Arc<Mutex<ChildStdin>>,
    next_id: Arc<AtomicU64>,
    pending: Arc<Mutex<HashMap<u64, Sender<Result<Value, String>>>>>,
    running: Arc<AtomicBool>,
    active_prompt_id: Option<Arc<Mutex<Option<u64>>>>,
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

        if method == "session/prompt" {
            if let Some(ap) = &self.active_prompt_id {
                if let Ok(mut g) = ap.lock() {
                    *g = Some(id);
                }
            }
        }

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

        let out = match rx.recv_timeout(timeout) {
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
        };

        if method == "session/prompt" {
            if let Some(ap) = &self.active_prompt_id {
                if let Ok(mut g) = ap.lock() {
                    if g.as_ref() == Some(&id) {
                        *g = None;
                    }
                }
            }
        }

        out
    }
}

fn parse_id(msg: &Value) -> Option<u64> {
    msg.get("id").and_then(|v| {
        v.as_u64()
            .or_else(|| v.as_i64().map(|i| i as u64))
            .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
    })
}

fn handle_incoming(
    app: &AppHandle,
    pending: &Mutex<HashMap<u64, Sender<Result<Value, String>>>>,
    session_id: &Mutex<Option<String>>,
    stdin: &Mutex<ChildStdin>,
    msg: Value,
) {
    if let Some(id) = parse_id(&msg) {
        // Agent → client request (permissions, etc.)
        if let Some(method) = msg.get("method").and_then(|v| v.as_str()) {
            if method == "session/request_permission" || method.ends_with("request_permission") {
                let params = msg.get("params").cloned().unwrap_or(json!({}));
                let options = params
                    .get("options")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let tool_call = params.get("toolCall").cloned();
                let _ = app.emit(
                    EVENT_PERMISSION,
                    PermissionPayload {
                        request_id: id,
                        tool_call,
                        options,
                        raw: msg.clone(),
                    },
                );
                return;
            }

            // Unknown agent request — cancel/error so the agent does not hang.
            emit_log(app, format!("Unhandled agent request `{method}` id={id}"));
            let err = json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("Method not supported by Grok Desktop: {method}") }
            });
            if let Ok(mut s) = stdin.lock() {
                let _ = writeln!(s, "{err}");
                let _ = s.flush();
            }
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
                if !(other.starts_with("_x.ai/") || other.starts_with("x.ai/")) {
                    emit_log(app, format!("notif {other}"));
                }
            }
        }
    }
}
