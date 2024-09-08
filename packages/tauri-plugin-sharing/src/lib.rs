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
use desktop::Sharing;
#[cfg(mobile)]
use mobile::Sharing;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the sharing APIs.
pub trait SharingExt<R: Runtime> {
  fn sharing(&self) -> &Sharing<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SharingExt<R> for T {
  fn sharing(&self) -> &Sharing<R> {
    self.state::<Sharing<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("sharing")
    .invoke_handler(tauri::generate_handler![commands::ping, commands::share])
    .setup(|app, api| {
      #[cfg(mobile)]
      let sharing = mobile::init(app, api)?;
      #[cfg(desktop)]
      let sharing = desktop::init(app, api)?;
      app.manage(sharing);
      Ok(())
    })
    .build()
}
