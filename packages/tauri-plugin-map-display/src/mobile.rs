use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_map_display);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<MapDisplay<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "MapDisplayPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_map_display)?;
  Ok(MapDisplay(handle))
}

/// Access to the context-menu APIs.
pub struct MapDisplay<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> MapDisplay<R> {
  pub fn show_map(&self, payload: ShowMapRequest) -> crate::Result<ShowMapResponse> {
    self
      .0
      .run_mobile_plugin("showMap", payload)
      .map_err(Into::into)
  }

  pub fn hide_map(&self) -> crate::Result<HideMapResponse> {
    self
      .0
      .run_mobile_plugin("hideMap", ())
      .map_err(Into::into)
  }

  pub fn set_region(&self, payload: SetRegionRequest) -> crate::Result<SetRegionResponse> {
    self
      .0
      .run_mobile_plugin("setRegion", payload)
      .map_err(Into::into)
  }

}
