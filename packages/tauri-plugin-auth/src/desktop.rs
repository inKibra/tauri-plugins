use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Auth<R>> {
  Ok(Auth(app.clone()))
}

/// Access to the Auth APIs.
pub struct Auth<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Auth<R> {
  pub fn authenticate(&self, _: AuthenticateArgs) -> crate::Result<AuthResult> {
    // For desktop, we'll return a mock result indicating that authentication is not supported
    Ok(AuthResult {
      success: false,
      token: None,
      error: Some("Authentication is not supported on desktop platforms".into()),
    })
  }
}