use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::IAP;
#[cfg(mobile)]
use mobile::IAP;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the IAP APIs.
pub trait IAPExt<R: Runtime> {
  fn iap(&self) -> &IAP<R>;
}

impl<R: Runtime, T: Manager<R>> crate::IAPExt<R> for T {
  fn iap(&self) -> &IAP<R> {
    self.state::<IAP<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("iap")
    .invoke_handler(tauri::generate_handler![
      commands::fetch_products,
      commands::purchase_product,
      commands::restore_purchases,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let iap = mobile::init(app, api)?;
      #[cfg(desktop)]
      let iap = desktop::init(app, api)?;
      app.manage(iap);
      Ok(())
    })
    .build()
}