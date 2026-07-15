//! Monitor Grok Build CLI version, updates, models, and local changelog.

use crate::grok;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub context_window: Option<u64>,
    pub supports_reasoning_effort: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheck {
    pub current_version: Option<String>,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub channel: Option<String>,
    pub auto_update: Option<bool>,
    pub error: Option<String>,
    pub raw: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildMonitor {
    pub binary_path: Option<String>,
    pub version: Option<String>,
    pub stable_version: Option<String>,
    pub checked_at: Option<String>,
    pub ready: bool,
    pub authenticated: bool,
    pub message: String,
    pub update: UpdateCheck,
    pub models: Vec<ModelInfo>,
    pub changelog_preview: Vec<String>,
    pub grok_home: Option<String>,
}

pub fn monitor() -> BuildMonitor {
    let status = grok::probe();
    let home = dirs_grok_home();

    let (version_file_version, stable_version, checked_at) = read_version_json(home.as_ref());
    let version = status
        .version
        .clone()
        .or(version_file_version)
        .or_else(|| status.binary_path.as_ref().map(|_| "unknown".into()));

    let update = check_update(status.binary_path.as_deref());
    let models = read_models_cache(home.as_ref());
    let changelog_preview = read_changelog_preview(home.as_ref(), 8);

    BuildMonitor {
        binary_path: status.binary_path,
        version,
        stable_version,
        checked_at,
        ready: status.ready,
        authenticated: status.authenticated,
        message: status.message,
        update,
        models,
        changelog_preview,
        grok_home: home.map(|p| p.display().to_string()),
    }
}

/// Run `grok update` (installs latest). Long-running; call from spawn_blocking.
pub fn run_update() -> Result<String, String> {
    let bin = grok::resolve_binary().ok_or_else(|| "Grok CLI not found".to_string())?;
    let output = Command::new(&bin)
        .args(["update"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run grok update: {e}"))?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    let err = String::from_utf8_lossy(&output.stderr);
    if !err.trim().is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&err);
    }
    if !output.status.success() {
        return Err(if text.trim().is_empty() {
            format!("grok update failed ({})", output.status)
        } else {
            text
        });
    }
    Ok(text)
}

fn check_update(bin: Option<&str>) -> UpdateCheck {
    let Some(bin) = bin else {
        return UpdateCheck {
            current_version: None,
            latest_version: None,
            update_available: false,
            channel: None,
            auto_update: None,
            error: Some("no binary".into()),
            raw: None,
        };
    };

    // `grok update --check --json` with soft timeout
    let mut child = match Command::new(bin)
        .args(["update", "--check", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return UpdateCheck {
                current_version: None,
                latest_version: None,
                update_available: false,
                channel: None,
                auto_update: None,
                error: Some(e.to_string()),
                raw: None,
            };
        }
    };

    let start = Instant::now();
    let timeout = Duration::from_secs(12);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return UpdateCheck {
                    current_version: None,
                    latest_version: None,
                    update_available: false,
                    channel: None,
                    auto_update: None,
                    error: Some("update check timed out".into()),
                    raw: None,
                };
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(40)),
            Err(e) => {
                return UpdateCheck {
                    current_version: None,
                    latest_version: None,
                    update_available: false,
                    channel: None,
                    auto_update: None,
                    error: Some(e.to_string()),
                    raw: None,
                };
            }
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            return UpdateCheck {
                current_version: None,
                latest_version: None,
                update_available: false,
                channel: None,
                auto_update: None,
                error: Some(e.to_string()),
                raw: None,
            };
        }
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let raw: Value = match serde_json::from_str(text.trim()) {
        Ok(v) => v,
        Err(_) => {
            return UpdateCheck {
                current_version: None,
                latest_version: None,
                update_available: false,
                channel: None,
                auto_update: None,
                error: Some(format!(
                    "bad update JSON: {}",
                    text.chars().take(200).collect::<String>()
                )),
                raw: None,
            };
        }
    };

    UpdateCheck {
        current_version: raw
            .get("currentVersion")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        latest_version: raw
            .get("latestVersion")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        update_available: raw
            .get("updateAvailable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        channel: raw
            .get("channel")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        auto_update: raw.get("autoUpdate").and_then(|v| v.as_bool()),
        error: raw
            .get("error")
            .and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(v.to_string())
                }
            }),
        raw: Some(raw),
    }
}

fn read_version_json(home: Option<&PathBuf>) -> (Option<String>, Option<String>, Option<String>) {
    let Some(home) = home else {
        return (None, None, None);
    };
    let path = home.join("version.json");
    let Ok(text) = fs::read_to_string(path) else {
        return (None, None, None);
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return (None, None, None);
    };
    (
        v.get("version")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string()),
        v.get("stable_version")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string()),
        v.get("checked_at")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string()),
    )
}

fn read_models_cache(home: Option<&PathBuf>) -> Vec<ModelInfo> {
    let Some(home) = home else {
        return vec![];
    };
    let path = home.join("models_cache.json");
    let Ok(text) = fs::read_to_string(path) else {
        return vec![];
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return vec![];
    };
    let Some(models) = v.get("models").and_then(|m| m.as_object()) else {
        return vec![];
    };

    let mut out = Vec::new();
    for (id, entry) in models {
        let info = entry.get("info").unwrap_or(entry);
        let hidden = info
            .get("hidden")
            .and_then(|h| h.as_bool())
            .unwrap_or(false);
        if hidden {
            continue;
        }
        out.push(ModelInfo {
            id: id.clone(),
            name: info
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(id)
                .to_string(),
            description: info
                .get("description")
                .and_then(|d| d.as_str())
                .map(|s| s.to_string()),
            context_window: info.get("context_window").and_then(|c| c.as_u64()),
            supports_reasoning_effort: info
                .get("supports_reasoning_effort")
                .and_then(|b| b.as_bool())
                .unwrap_or(false),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn read_changelog_preview(home: Option<&PathBuf>, max: usize) -> Vec<String> {
    let Some(home) = home else {
        return vec![];
    };
    let path = home.join("CHANGELOG.md");
    let Ok(text) = fs::read_to_string(path) else {
        return vec![];
    };
    text.lines()
        .filter(|l| l.starts_with("- ") || l.starts_with("# "))
        .take(max)
        .map(|s| s.to_string())
        .collect()
}

fn dirs_grok_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".grok"))
}
