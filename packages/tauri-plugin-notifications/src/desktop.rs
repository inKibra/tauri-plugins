use serde::de::DeserializeOwned;
use tauri::{ipc::Channel, plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Notifications<R>> {
  Ok(Notifications(app.clone()))
}

/// Access to the Notifications APIs.
pub struct Notifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Notifications<R> {
  pub fn check_permissions(&self) -> crate::Result<NotificationPermissionStatus> {
    Ok(NotificationPermissionStatus {
      status: "denied".into(),
    })
  }

  pub fn request_permissions(&self) -> crate::Result<NotificationPermissionStatus> {
    Ok(NotificationPermissionStatus {
      status: "denied".into(),
    })
  }

  pub fn check_registration_status(&self) -> crate::Result<NotificationRegistrationStatus> {
    Ok(NotificationRegistrationStatus {
      is_registered: false,
      token: None,
    })
  }

  pub fn register_for_remote_notifications(&self) -> crate::Result<NotificationRegistrationResult> {
    Ok(NotificationRegistrationResult {
      success: false,
      token: None,
      error: Some("Remote notifications are not supported on desktop".into()),
    })
  }

  pub fn watch_notifications(&self, _channel: Channel) -> crate::Result<WatchNotificationResult> {
    Ok(WatchNotificationResult { success: true })
  }
}