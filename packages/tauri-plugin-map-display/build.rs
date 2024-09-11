const COMMANDS: &[&str] = &["show_map", "hide_map", "set_region"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
