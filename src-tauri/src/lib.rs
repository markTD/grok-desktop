mod acp;
mod grok;

use grok::GrokStatus;

#[tauri::command]
fn grok_status() -> GrokStatus {
    grok::probe()
}

#[tauri::command]
fn acp_planned_command() -> Vec<String> {
    acp::planned_stdio_args()
        .iter()
        .map(|s| s.to_string())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![grok_status, acp_planned_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
