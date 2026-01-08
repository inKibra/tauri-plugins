use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{
    ipc::Channel,
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_notifications);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Notifications<R>> {
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_notifications)?;
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.notifications", "NotificationsPlugin")?;
    #[cfg(target_os = "android")]
    handle.run_mobile_plugin("initFirebase", ())?;
    Ok(Notifications(handle))
}

pub struct Notifications<R: Runtime>(PluginHandle<R>);

#[derive(Serialize)]
struct WatchNotificationsArgs {
    channel: Channel,
}

impl<R: Runtime> Notifications<R> {
    pub fn check_permissions(&self) -> crate::Result<NotificationPermissionStatus> {
        self.0
            .run_mobile_plugin("checkPermissions", ())
            .map_err(Into::into)
    }

    pub fn request_permissions(&self) -> crate::Result<NotificationPermissionStatus> {
        self.0
            .run_mobile_plugin("requestPermissions", ())
            .map_err(Into::into)
    }

    pub fn check_registration_status(&self) -> crate::Result<NotificationRegistrationStatus> {
        self.0
            .run_mobile_plugin("checkRegistrationStatus", ())
            .map_err(Into::into)
    }

    pub fn register_for_remote_notifications(
        &self,
    ) -> crate::Result<NotificationRegistrationResult> {
        self.0
            .run_mobile_plugin("registerForRemoteNotifications", ())
            .map_err(Into::into)
    }

    pub fn watch_notifications(&self, channel: Channel) -> crate::Result<WatchNotificationResult> {
        self.0
            .run_mobile_plugin("watchNotifications", WatchNotificationsArgs { channel })
            .map_err(Into::into)
    }
}
