use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Sharing<R>> {
  Ok(Sharing(app.clone()))
}

/// Access to the sharing APIs.
pub struct Sharing<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Sharing<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn share(&self, payload: ShareRequest) -> crate::Result<ShareResponse> {
    Ok(ShareResponse {
      success: true,
    })
  }
}