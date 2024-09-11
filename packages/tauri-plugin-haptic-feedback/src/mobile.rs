use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_haptic_feedback);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<HapticFeedback<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "HapticFeedbackPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_haptic_feedback)?;
  Ok(HapticFeedback(handle))
}

/// Access to the context-menu APIs.
pub struct HapticFeedback<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> HapticFeedback<R> {
  pub fn vibrate(&self, payload: HapticVibrateRequest) -> crate::Result<HapticResponse> {
    self
      .0
      .run_mobile_plugin("vibrate", payload)
      .map_err(Into::into)
  }

  pub fn impact_feedback(&self, payload: ImpactFeedbackRequest) -> crate::Result<HapticResponse> {
    self
      .0
      .run_mobile_plugin("impactFeedback", payload)
      .map_err(Into::into)
  }

  pub fn selection_feedback(&self) -> crate::Result<HapticResponse> {
    self
      .0
      .run_mobile_plugin("selectionFeedback", ())
      .map_err(Into::into)
  }
}
