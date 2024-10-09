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
use desktop::Auth;
#[cfg(mobile)]
use mobile::Auth;

pub trait AuthExt<R: Runtime> {
  fn auth(&self) -> &Auth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AuthExt<R> for T {
  fn auth(&self) -> &Auth<R> {
    self.state::<Auth<R>>().inner()
  }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("auth")
    .invoke_handler(tauri::generate_handler![
      commands::authenticate,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let auth = mobile::init(app, api)?;
      #[cfg(desktop)]
      let auth = desktop::init(app, api)?;
      app.manage(auth);
      Ok(())
    })
    .build()
}