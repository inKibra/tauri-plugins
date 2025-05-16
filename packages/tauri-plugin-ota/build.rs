const COMMANDS: &[&str] = &["prepare"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
