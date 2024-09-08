use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SharingExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.sharing().ping(payload)
}

#[command]
pub(crate) async fn share<R: Runtime>(
    app: AppHandle<R>,
    payload: ShareRequest,
) -> Result<ShareResponse> {
    app.sharing().share(payload)
}