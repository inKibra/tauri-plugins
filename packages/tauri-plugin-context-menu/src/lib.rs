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
use desktop::ContextMenu;
#[cfg(mobile)]
use mobile::ContextMenu;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the context-menu APIs.
pub trait ContextMenuExt<R: Runtime> {
  fn context_menu(&self) -> &ContextMenu<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ContextMenuExt<R> for T {
  fn context_menu(&self) -> &ContextMenu<R> {
    self.state::<ContextMenu<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("context-menu")
    .invoke_handler(tauri::generate_handler![commands::show_context_menu])
    .setup(|app, api| {
      #[cfg(mobile)]
      let context_menu = mobile::init(app, api)?;
      #[cfg(desktop)]
      let context_menu = desktop::init(app, api)?;
      app.manage(context_menu);
      Ok(())
    })
    .build()
}
