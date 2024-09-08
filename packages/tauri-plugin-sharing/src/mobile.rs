use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_sharing);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Sharing<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.inkibra.tauri.plugin.sharing", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_sharing)?;
  Ok(Sharing(handle))
}

/// Access to the sharing APIs.
pub struct Sharing<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Sharing<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
  pub fn share(&self, payload: ShareRequest) -> crate::Result<ShareResponse> {
    self
      .0
      .run_mobile_plugin("share", payload)
      .map_err(Into::into)
  }
}

