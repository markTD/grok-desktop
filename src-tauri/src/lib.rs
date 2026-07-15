mod acp;
mod grok;

use acp::{
    AcpState, ConnectOptions, ConnectResult, ConnectionInfo, PermissionReply, PromptResult,
};
use grok::GrokStatus;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
fn grok_status() -> GrokStatus {
    grok::probe()
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Arc::new(AcpState::new()))
        .invoke_handler(tauri::generate_handler![
            grok_status,
            acp_connect,
            acp_prompt,
            acp_respond_permission,
            acp_disconnect,
            acp_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
