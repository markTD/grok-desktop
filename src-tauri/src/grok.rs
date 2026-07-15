//! Locate and probe the official Grok Build CLI (`grok`).
//!
//! Auth and agent runtime stay with the CLI; this crate only discovers the
//! binary and reports readiness for the desktop shell.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Soft timeout for `grok models` so the UI never freezes mid-session.
const MODELS_TIMEOUT: Duration = Duration::from_secs(8);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GrokStatus {
    /// Absolute path to the binary if found.
    pub binary_path: Option<String>,
    /// Output of `grok --version` (first line), if available.
    pub version: Option<String>,
    /// True when auth looks good (`grok models` or local auth cache).
    pub authenticated: bool,
    /// Human-readable status for the UI.
    pub message: String,
    /// True when we can spawn ACP sessions (binary found + auth ok-ish).
    pub ready: bool,
    /// How auth was determined (for the UI chip tooltip).
    pub auth_source: Option<String>,
}

impl GrokStatus {
    fn not_found(hint: impl Into<String>) -> Self {
        Self {
            binary_path: None,
            version: None,
            authenticated: false,
            message: hint.into(),
            ready: false,
            auth_source: None,
        }
    }
}

/// Resolve the `grok` binary: `GROK_BINARY` env, then `PATH`, then `~/.grok/bin/grok`.
pub fn resolve_binary() -> Option<PathBuf> {
    if let Ok(custom) = std::env::var("GROK_BINARY") {
        let p = PathBuf::from(custom);
        if is_executable(&p) {
            return Some(p);
        }
    }

    if let Some(path) = which("grok") {
        return Some(path);
    }

    if let Some(home) = dirs_home() {
        let candidate = home.join(".grok").join("bin").join("grok");
        if is_executable(&candidate) {
            return Some(candidate);
        }
    }

    None
}

/// Full readiness probe for the UI.
pub fn probe() -> GrokStatus {
    let Some(bin) = resolve_binary() else {
        return GrokStatus::not_found(
            "Grok Build CLI not found. Install from https://x.ai/cli or set GROK_BINARY.",
        );
    };

    let bin_str = bin.display().to_string();
    let version = run_capture(&bin, &["--version"]).ok().and_then(|out| {
        out.lines()
            .next()
            .map(|l| l.trim().to_string())
            .filter(|s| !s.is_empty())
    });

    // Prefer live `grok models`, but never block the desktop UI forever.
    // While an ACP session is running, `models` can be slow or flaky — fall back
    // to the local auth cache so the chip stays "ready".
    let (authenticated, auth_source) = match run_status_timeout(&bin, &["models"], MODELS_TIMEOUT) {
        Some(0) => (true, Some("models".into())),
        Some(_) => {
            if auth_cache_present() {
                (true, Some("auth-cache".into()))
            } else {
                (false, Some("models-failed".into()))
            }
        }
        None => {
            if auth_cache_present() {
                (true, Some("auth-cache".into()))
            } else if version.is_some() {
                // Binary works; let connect attempt OAuth path rather than hard-block.
                (true, Some("version-only".into()))
            } else {
                (false, Some("timeout".into()))
            }
        }
    };

    let (ready, message) = match (version.as_ref(), authenticated) {
        (Some(v), true) => {
            let src = auth_source.as_deref().unwrap_or("ok");
            (
                true,
                format!("CLI ready — {v} ({src})"),
            )
        }
        (Some(v), false) => (
            false,
            format!(
                "Found {v} at {bin_str}, but auth failed. Run `grok` in a terminal to sign in."
            ),
        ),
        (None, true) => (true, format!("CLI ready — {bin_str}")),
        (None, false) => (
            false,
            format!(
                "Found binary at {bin_str}, but could not verify version/auth. Try `grok models` in a terminal."
            ),
        ),
    };

    GrokStatus {
        binary_path: Some(bin_str),
        version,
        authenticated,
        message,
        ready,
        auth_source,
    }
}

fn auth_cache_present() -> bool {
    dirs_home()
        .map(|h| h.join(".grok").join("auth.json").is_file())
        .unwrap_or(false)
}

fn run_capture(bin: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| format!("failed to spawn {}: {e}", bin.display()))?;

    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }
    if !output.status.success() && text.trim().is_empty() {
        return Err(format!("exit {}", output.status));
    }
    Ok(text)
}

/// Run a short command; `None` means timeout/spawn failure.
fn run_status_timeout(bin: &Path, args: &[&str], timeout: Duration) -> Option<i32> {
    let mut child = Command::new(bin)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status.code().unwrap_or(-1)),
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => {
                let _ = child.kill();
                return None;
            }
        }
    }
}

fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true
    }
}
