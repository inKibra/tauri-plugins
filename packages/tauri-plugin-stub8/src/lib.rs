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
use desktop::Example;
#[cfg(mobile)]
use mobile::Example;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the example APIs.
pub trait ExampleExt<R: Runtime> {
  fn example(&self) -> &Example<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ExampleExt<R> for T {
  fn example(&self) -> &Example<R> {
    self.state::<Example<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("example")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let example = mobile::init(app, api)?;
      #[cfg(desktop)]
      let example = desktop::init(app, api)?;
      app.manage(example);
      Ok(())
    })
    .build()
}
