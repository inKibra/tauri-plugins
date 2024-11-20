use tauri::{ipc::Channel, AppHandle, command, Runtime};

use crate::{models::*, Result, NotificationsExt};

#[command]
pub(crate) async fn check_permissions<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<NotificationPermissionStatus> {
    app_handle.notifications().check_permissions()
}

#[command]
pub(crate) async fn request_permissions<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<NotificationPermissionStatus> {
    app_handle.notifications().request_permissions()
}

#[command]
pub(crate) async fn check_registration_status<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<NotificationRegistrationStatus> {
    app_handle.notifications().check_registration_status()
}

#[command]
pub(crate) async fn register_for_remote_notifications<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<NotificationRegistrationResult> {
    app_handle.notifications().register_for_remote_notifications()
}

#[command]
pub(crate) async fn watch_notifications<R: Runtime>(
    app_handle: AppHandle<R>,
    channel: Channel,
) -> Result<WatchNotificationResult> {
    app_handle.notifications().watch_notifications(channel)
}
