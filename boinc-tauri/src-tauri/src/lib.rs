// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod boinc_rpc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_boinc_tasks(active_only: Option<bool>) -> Result<Vec<boinc_rpc::BoincTask>, String> {
    boinc_rpc::get_results(active_only.unwrap_or(false)).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_boinc_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
