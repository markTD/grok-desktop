//! Project and Grok session path helpers.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub cwd: String,
    pub is_git: bool,
    pub git_root: Option<String>,
    pub branch: Option<String>,
    /// URL-encoded group dir under ~/.grok/sessions
    pub sessions_group: Option<String>,
    pub sessions_group_path: Option<String>,
    pub session_count: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionPaths {
    pub session_id: String,
    pub session_dir: Option<String>,
    pub plan_md: Option<String>,
    pub summary_json: Option<String>,
    pub updates_jsonl: Option<String>,
}

pub fn project_info(cwd: String) -> Result<ProjectInfo, String> {
    let cwd = cwd.trim().to_string();
    if cwd.is_empty() {
        return Err("cwd empty".into());
    }
    let root = PathBuf::from(&cwd);
    if !root.is_dir() {
        return Err(format!("Not a directory: {cwd}"));
    }

    let is_git = root.join(".git").exists() || git_rev_parse_ok(&root);
    let git_root = if is_git {
        git_output(&root, &["rev-parse", "--show-toplevel"])
    } else {
        None
    };
    let branch = if is_git {
        git_output(&root, &["rev-parse", "--abbrev-ref", "HEAD"])
    } else {
        None
    };

    let encoded = percent_encode_path(&cwd);
    let group = dirs_home().map(|h| h.join("sessions").join(&encoded));
    let sessions_group_path = group
        .as_ref()
        .filter(|p| p.is_dir())
        .map(|p| p.display().to_string());
    let session_count = group
        .as_ref()
        .filter(|p| p.is_dir())
        .map(|p| count_sessions(p))
        .unwrap_or(0);

    Ok(ProjectInfo {
        cwd,
        is_git,
        git_root,
        branch,
        sessions_group: sessions_group_path.as_ref().map(|_| encoded),
        sessions_group_path,
        session_count,
    })
}

pub fn session_paths(cwd: String, session_id: String) -> Result<SessionPaths, String> {
    let info = project_info(cwd)?;
    let Some(group) = info.sessions_group_path else {
        return Ok(SessionPaths {
            session_id,
            session_dir: None,
            plan_md: None,
            summary_json: None,
            updates_jsonl: None,
        });
    };
    let dir = PathBuf::from(&group).join(&session_id);
    if !dir.is_dir() {
        return Ok(SessionPaths {
            session_id,
            session_dir: None,
            plan_md: None,
            summary_json: None,
            updates_jsonl: None,
        });
    }

    let pick = |name: &str| {
        let p = dir.join(name);
        if p.is_file() {
            Some(p.display().to_string())
        } else {
            None
        }
    };

    // plan.md (plan mode) or plan.json (todo list)
    let plan_md = pick("plan.md").or_else(|| pick("plan.json"));

    Ok(SessionPaths {
        session_id,
        session_dir: Some(dir.display().to_string()),
        plan_md,
        summary_json: pick("summary.json"),
        updates_jsonl: pick("updates.jsonl"),
    })
}

fn count_sessions(group: &Path) -> u32 {
    let Ok(rd) = std::fs::read_dir(group) else {
        return 0;
    };
    rd.flatten()
        .filter(|e| e.path().is_dir())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("019") // UUIDv7 sessions from grok
                || e.path().join("summary.json").is_file()
        })
        .count() as u32
}

fn percent_encode_path(path: &str) -> String {
    // Match Grok's URL-encoding of the full cwd for session grouping.
    let mut out = String::with_capacity(path.len() * 3);
    for b in path.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char);
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{b:02X}"));
            }
        }
    }
    out
}

fn git_rev_parse_ok(dir: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn git_output(dir: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".grok"))
}
