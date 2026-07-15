//! Locate and probe the official Grok Build CLI (`grok`).
//!
//! Auth and agent runtime stay with the CLI; this crate only discovers the
//! binary and reports readiness for the desktop shell.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GrokStatus {
    /// Absolute path to the binary if found.
    pub binary_path: Option<String>,
    /// Output of `grok --version` (first line), if available.
    pub version: Option<String>,
    /// True when `grok models` exits 0 (soft auth check used by the CC bridge).
    pub authenticated: bool,
    /// Human-readable status for the UI.
    pub message: String,
    /// True when we can spawn ACP sessions (binary found + authenticated).
    pub ready: bool,
}

impl GrokStatus {
    fn not_found(hint: impl Into<String>) -> Self {
        Self {
            binary_path: None,
            version: None,
            authenticated: false,
            message: hint.into(),
            ready: false,
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

    let auth = run_status(&bin, &["models"]);
    let authenticated = auth.map(|code| code == 0).unwrap_or(false);

    let (ready, message) = match (version.as_ref(), authenticated) {
        (Some(v), true) => (
            true,
            format!("Ready — {v} (authenticated)"),
        ),
        (Some(v), false) => (
            false,
            format!(
                "Found {v} at {bin_str}, but auth failed. Run `grok` in a terminal to sign in."
            ),
        ),
        (None, true) => (true, format!("Ready — {bin_str}")),
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
    }
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

fn run_status(bin: &Path, args: &[&str]) -> Result<i32, String> {
    let status = Command::new(bin)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("failed to spawn {}: {e}", bin.display()))?;
    Ok(status.code().unwrap_or(-1))
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

