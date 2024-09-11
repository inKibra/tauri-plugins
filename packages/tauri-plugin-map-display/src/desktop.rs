use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<MapDisplay<R>> {
  Ok(MapDisplay(app.clone()))
}

/// Access to the map-display APIs.
pub struct MapDisplay<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MapDisplay<R> {
  pub fn show_map(&self, payload: ShowMapRequest) -> crate::Result<ShowMapResponse> {
    Ok(ShowMapResponse {
      success: true,
    })
  }

  pub fn hide_map(&self) -> crate::Result<HideMapResponse> {
    Ok(HideMapResponse {
      success: true,
    })
  }

  pub fn set_region(&self, payload: SetRegionRequest) -> crate::Result<SetRegionResponse> {
    Ok(SetRegionResponse {
      success: true,
    })
  }
}
