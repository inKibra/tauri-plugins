const COMMANDS: &[&str] = &["fetch_products", "purchase_product", "restore_purchases"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
