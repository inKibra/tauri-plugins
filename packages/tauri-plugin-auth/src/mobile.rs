use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_auth);

pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Auth<R>> {
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_auth)?;
  Ok(Auth(handle))
}

pub struct Auth<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Auth<R> {
  pub fn authenticate(&self, payload: AuthenticateArgs) -> crate::Result<AuthResult> {
    self
      .0
      .run_mobile_plugin("authenticate", payload)
      .map_err(Into::into)
  }
}