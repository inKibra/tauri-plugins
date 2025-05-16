// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_sharing::init())
        .plugin(tauri_plugin_context_menu::init())
        .plugin(tauri_plugin_map_display::init())
        .plugin(tauri_plugin_haptic_feedback::init())
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_iap::init())
        .plugin(tauri_plugin_auth::init())
        .plugin(tauri_plugin_notifications::init())
        .plugin(tauri_plugin_ota::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
