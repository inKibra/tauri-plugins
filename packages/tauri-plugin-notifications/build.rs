const COMMANDS: &[&str] = &["check_permissions", "request_permissions", "check_registration_status", "register_for_remote_notifications", "watch_notifications"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
