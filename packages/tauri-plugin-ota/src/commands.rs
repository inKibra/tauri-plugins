use tauri::{AppHandle, command, Runtime};

use crate::{models::*, Result, OTAExt};

#[command]
pub(crate) async fn prepare<R: Runtime>(
    app_handle: AppHandle<R>,
    payload: PrepareArgs,
) -> Result<UpdateInfo> {
    app_handle.ota_manager().prepare(payload.manifest_url).await
}
