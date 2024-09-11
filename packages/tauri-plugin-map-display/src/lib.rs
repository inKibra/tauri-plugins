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
use desktop::MapDisplay;
#[cfg(mobile)]
use mobile::MapDisplay;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the map-display APIs.
pub trait MapDisplayExt<R: Runtime> {
  fn map_display(&self) -> &MapDisplay<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MapDisplayExt<R> for T {
  fn map_display(&self) -> &MapDisplay<R> {
    self.state::<MapDisplay<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("map-display")
    .invoke_handler(tauri::generate_handler![
      commands::show_map,
      commands::hide_map,
      commands::set_region,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let map_display = mobile::init(app, api)?;
      #[cfg(desktop)]
      let map_display = desktop::init(app, api)?;
      app.manage(map_display);
      Ok(())
    })
    .build()
}
