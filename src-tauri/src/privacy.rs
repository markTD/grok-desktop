//! Local privacy / safety snapshot for transparency UI.
//! Does not read token secrets — only config flags and path facts.

use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacySnapshot {
    pub grok_home: Option<String>,
    pub auth_cache_present: bool,
    pub sessions_dir_present: bool,
    /// From config if present: [features] telemetry
    pub telemetry_enabled: Option<bool>,
    /// From config if present: [telemetry] trace_upload
    pub trace_upload: Option<bool>,
    /// From config if present: [ui] yolo or permission_mode
    pub config_yolo: Option<bool>,
    pub config_permission_mode: Option<String>,
    pub config_path: Option<String>,
    /// Human summary lines for the UI
    pub summary_lines: Vec<String>,
}

pub fn snapshot() -> PrivacySnapshot {
    let home = dirs_grok_home();
    let config_path = home.as_ref().map(|h| h.join("config.toml"));
    let (telemetry, trace_upload, yolo, perm_mode) = config_path
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|t| parse_config_flags(&t))
        .unwrap_or((None, None, None, None));

    let auth_cache_present = home
        .as_ref()
        .map(|h| h.join("auth.json").is_file())
        .unwrap_or(false);
    let sessions_dir_present = home
        .as_ref()
        .map(|h| h.join("sessions").is_dir())
        .unwrap_or(false);

    let mut summary_lines = vec![
        "Grok Desktop is a local UI. It does not run its own cloud backend."
            .into(),
        "Model calls and tool results go through the official Grok Build CLI → xAI (and any MCP you enabled)."
            .into(),
        "Sessions are stored under ~/.grok/sessions/ on this machine."
            .into(),
        "Exported notes (if you use Export) are written only under <project>/.grok-desktop/notes/."
            .into(),
    ];

    if let Some(t) = telemetry {
        summary_lines.push(format!(
            "Local config [features] telemetry = {t} (Grok Build anonymous telemetry switch)."
        ));
    } else {
        summary_lines.push(
            "Could not read [features] telemetry from config — open ~/.grok/config.toml or run /privacy in the TUI."
                .into(),
        );
    }

    if let Some(ref mode) = perm_mode {
        summary_lines.push(format!(
            "Grok Build UI permission_mode in config: {mode}."
        ));
    }

    summary_lines.push(
        "Account data-retention options: run `grok` TUI and use /privacy (plan-dependent)."
            .into(),
    );

    PrivacySnapshot {
        grok_home: home.map(|p| p.display().to_string()),
        auth_cache_present,
        sessions_dir_present,
        telemetry_enabled: telemetry,
        trace_upload,
        config_yolo: yolo,
        config_permission_mode: perm_mode,
        config_path: config_path.map(|p| p.display().to_string()),
        summary_lines,
    }
}

fn parse_config_flags(toml: &str) -> (Option<bool>, Option<bool>, Option<bool>, Option<String>) {
    // Lightweight line scan — avoids pulling a TOML crate for a few flags.
    let mut section = String::new();
    let mut telemetry = None;
    let mut trace_upload = None;
    let mut yolo = None;
    let mut permission_mode = None;

    for raw in toml.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(|c| c == '[' || c == ']').to_string();
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let k = k.trim();
        let v = v.trim().trim_matches('"');

        match (section.as_str(), k) {
            ("features", "telemetry") => telemetry = parse_bool(v),
            ("telemetry", "trace_upload") => trace_upload = parse_bool(v),
            ("ui", "yolo") => yolo = parse_bool(v),
            ("ui", "permission_mode") => permission_mode = Some(v.to_string()),
            _ => {}
        }
    }

    (telemetry, trace_upload, yolo, permission_mode)
}

fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn dirs_grok_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".grok"))
}
