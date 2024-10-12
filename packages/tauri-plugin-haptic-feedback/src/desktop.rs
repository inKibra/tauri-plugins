use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<HapticFeedback<R>> {
  Ok(HapticFeedback(app.clone()))
}

/// Access to the haptic feedback APIs.
pub struct HapticFeedback<R: Runtime>(AppHandle<R>);

impl<R: Runtime> HapticFeedback<R> {
  pub fn vibrate(&self, _payload: HapticVibrateRequest) -> crate::Result<HapticResponse> {
    Ok(HapticResponse { success: true })
  }

  pub fn impact_feedback(&self, _payload: ImpactFeedbackRequest) -> crate::Result<HapticResponse> {
    Ok(HapticResponse { success: true })
  }

  pub fn selection_feedback(&self) -> crate::Result<HapticResponse> {
    Ok(HapticResponse { success: true })
  }
}
