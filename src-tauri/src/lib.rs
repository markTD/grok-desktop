mod acp;
mod build_monitor;
mod export;
mod grok;
mod privacy;
mod project;

use acp::{
    AcpState, ConnectOptions, ConnectResult, ConnectionInfo, PermissionReply, PromptResult,
};
use build_monitor::BuildMonitor;
use export::ExportResult;
use grok::GrokStatus;
use privacy::PrivacySnapshot;
use project::{ProjectInfo, SessionPaths};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
fn grok_status() -> GrokStatus {
    grok::probe()
}

#[tauri::command]
async fn build_monitor_report() -> BuildMonitor {
    tauri::async_runtime::spawn_blocking(build_monitor::monitor)
        .await
        .unwrap_or_else(|_| build_monitor::monitor())
}

#[tauri::command]
async fn grok_run_update() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(build_monitor::run_update)
        .await
        .map_err(|e| format!("update task failed: {e}"))?
}

#[tauri::command]
fn project_info(cwd: String) -> Result<ProjectInfo, String> {
    project::project_info(cwd)
}

#[tauri::command]
fn session_paths(cwd: String, session_id: String) -> Result<SessionPaths, String> {
    project::session_paths(cwd, session_id)
}

#[tauri::command]
async fn acp_connect(
    app: tauri::AppHandle,
    state: State<'_, Arc<AcpState>>,
    options: ConnectOptions,
) -> Result<ConnectResult, String> {
    let state = Arc::clone(&state);
    tauri::async_runtime::spawn_blocking(move || acp::connect(app, &state, options))
        .await
        .map_err(|e| format!("connect task failed: {e}"))?
}

#[tauri::command]
async fn acp_prompt(
    app: tauri::AppHandle,
    state: State<'_, Arc<AcpState>>,
    text: String,
) -> Result<PromptResult, String> {
    let state = Arc::clone(&state);
    tauri::async_runtime::spawn_blocking(move || acp::prompt(app, &state, text))
        .await
        .map_err(|e| format!("prompt task failed: {e}"))?
}

#[tauri::command]
fn acp_cancel(app: tauri::AppHandle, state: State<'_, Arc<AcpState>>) -> Result<(), String> {
    acp::cancel_turn(app, &state)
}

#[tauri::command]
fn acp_respond_permission(
    state: State<'_, Arc<AcpState>>,
    reply: PermissionReply,
) -> Result<(), String> {
    acp::respond_permission(&state, reply)
}

#[tauri::command]
fn acp_disconnect(state: State<'_, Arc<AcpState>>) -> Result<(), String> {
    acp::disconnect(&state)
}

#[tauri::command]
fn acp_connection(state: State<'_, Arc<AcpState>>) -> Result<ConnectionInfo, String> {
    acp::connection_info(&state)
}

#[tauri::command]
fn export_session_notes(
    cwd: String,
    markdown: String,
    suggested_name: Option<String>,
) -> Result<ExportResult, String> {
    export::export_session_notes(cwd, markdown, suggested_name)
}

#[tauri::command]
fn privacy_snapshot() -> PrivacySnapshot {
    privacy::snapshot()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Arc::new(AcpState::new()))
        .invoke_handler(tauri::generate_handler![
            grok_status,
            build_monitor_report,
            grok_run_update,
            project_info,
            session_paths,
            privacy_snapshot,
            acp_connect,
            acp_prompt,
            acp_cancel,
            acp_respond_permission,
            acp_disconnect,
            acp_connection,
            export_session_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
