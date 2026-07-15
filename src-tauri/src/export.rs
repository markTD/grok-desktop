//! Write session notes into the project folder for learning / portfolio.

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub path: String,
}

/// Save markdown notes under `<cwd>/.grok-desktop/notes/`.
pub fn export_session_notes(
    cwd: String,
    markdown: String,
    suggested_name: Option<String>,
) -> Result<ExportResult, String> {
    let cwd = cwd.trim();
    if cwd.is_empty() {
        return Err("Project folder is empty".into());
    }
    let root = PathBuf::from(cwd);
    if !root.is_dir() {
        return Err(format!("Not a directory: {cwd}"));
    }

    let notes_dir = root.join(".grok-desktop").join("notes");
    fs::create_dir_all(&notes_dir).map_err(|e| format!("Could not create notes dir: {e}"))?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let base = suggested_name
        .as_ref()
        .map(|s| sanitize_filename(s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("session-{stamp}"));

    let filename = if base.ends_with(".md") {
        base
    } else {
        format!("{base}.md")
    };

    let path = notes_dir.join(&filename);
    // Never escape the notes dir
    if !path.starts_with(&notes_dir) {
        return Err("Invalid notes path".into());
    }

    let header = format!(
        "<!-- Exported by Grok Desktop · {} -->\n\n",
        chrono_ish(stamp)
    );
    fs::write(&path, format!("{header}{markdown}"))
        .map_err(|e| format!("Write failed: {e}"))?;

    Ok(ExportResult {
        path: path.display().to_string(),
    })
}

fn sanitize_filename(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '-'
            }
        })
        .collect();
    s.trim_matches('-').chars().take(80).collect()
}

fn chrono_ish(secs: u64) -> String {
    // Lightweight UTC-ish stamp without chrono dependency
    format!("unix-{secs}")
}

#[allow(dead_code)]
fn ensure_under(parent: &Path, child: &Path) -> bool {
    child.starts_with(parent)
}
