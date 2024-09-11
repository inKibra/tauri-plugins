use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_context_menu);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<ContextMenu<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ContextMenuPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_context_menu)?;
  Ok(ContextMenu(handle))
}

/// Access to the context-menu APIs.
pub struct ContextMenu<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> ContextMenu<R> {
  pub fn show_context_menu(&self, payload: ShowContextMenuRequest) -> crate::Result<ShowContextMenuResponse> {
    self
      .0
      .run_mobile_plugin("showContextMenu", payload)
      .map_err(Into::into)
  }
}
