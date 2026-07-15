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
    /// Raw: [features] telemetry
    pub telemetry_enabled: Option<bool>,
    /// Raw: [telemetry] trace_upload
    pub trace_upload: Option<bool>,
    /// Plain status for laypeople: off | on | unknown
    pub analytics_status: String,
    /// e.g. "OFF", "ON", "NOT SURE"
    pub analytics_label: String,
    /// One short sentence for the UI card
    pub analytics_detail: String,
    pub config_yolo: Option<bool>,
    pub config_permission_mode: Option<String>,
    pub config_path: Option<String>,
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

    let (analytics_status, analytics_label, analytics_detail) =
        analytics_plain(telemetry, trace_upload);

    let mut summary_lines = vec![
        "Grok Desktop has no cloud of its own.".into(),
        "AI chat still uses Grok Build → xAI (prompts + files the agent opens)."
            .into(),
        format!("Optional product analytics (usage stats): {analytics_label}. {analytics_detail}"),
        "Session files stay under ~/.grok/sessions/ on this computer.".into(),
    ];

    if let Some(ref mode) = perm_mode {
        summary_lines.push(format!("Grok permission_mode in config: {mode}."));
    }

    summary_lines.push(
        "How long xAI keeps coding data: check SuperGrok/xAI settings (TUI: /privacy)."
            .into(),
    );

    PrivacySnapshot {
        grok_home: home.map(|p| p.display().to_string()),
        auth_cache_present,
        sessions_dir_present,
        telemetry_enabled: telemetry,
        trace_upload,
        analytics_status,
        analytics_label,
        analytics_detail,
        config_yolo: yolo,
        config_permission_mode: perm_mode,
        config_path: config_path.map(|p| p.display().to_string()),
        summary_lines,
    }
}

/// Collapse telemetry + trace_upload into one layperson status.
fn analytics_plain(
    telemetry: Option<bool>,
    trace_upload: Option<bool>,
) -> (String, String, String) {
    // Docs: trace_upload inherits the telemetry toggle when unset.
    let effective_traces = match (telemetry, trace_upload) {
        (_, Some(t)) => Some(t),
        (Some(t), None) => Some(t),
        (None, None) => None,
    };

    // effective_traces already folds “unset → follow telemetry”
    match effective_traces {
        Some(false) => (
            "off".into(),
            "OFF".into(),
            "Your Grok config turns optional product analytics off. This is separate from AI chat (which still needs the network)."
                .into(),
        ),
        Some(true) => (
            "on".into(),
            "ON".into(),
            "Optional product analytics look enabled in config (usage stats for the product). This is not the same as “sending your whole repo,” but it is extra data. Turn off with telemetry = false under [features] in ~/.grok/config.toml, then Refresh."
                .into(),
        ),
        None => (
            "unknown".into(),
            "NOT SURE".into(),
            "We could not find a clear analytics setting in ~/.grok/config.toml. AI chat still works over the network. To force analytics off, add under [features]: telemetry = false — then Refresh. Account retention is separate: run grok and type /privacy."
                .into(),
        ),
    }
}

fn parse_config_flags(toml: &str) -> (Option<bool>, Option<bool>, Option<bool>, Option<String>) {
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
