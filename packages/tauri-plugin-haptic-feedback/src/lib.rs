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
use desktop::HapticFeedback;
#[cfg(mobile)]
use mobile::HapticFeedback;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the map-display APIs.
pub trait HapticFeedbackExt<R: Runtime> {
  fn haptic_feedback(&self) -> &HapticFeedback<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HapticFeedbackExt<R> for T {
  fn haptic_feedback(&self) -> &HapticFeedback<R> {
    self.state::<HapticFeedback<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("haptic-feedback")
    .invoke_handler(tauri::generate_handler![
      commands::vibrate,
      commands::impact_feedback,
      commands::selection_feedback,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let haptic_feedback = mobile::init(app, api)?;
      #[cfg(desktop)]
      let haptic_feedback = desktop::init(app, api)?;
      app.manage(haptic_feedback);
      Ok(())
    })
    .build()
}
