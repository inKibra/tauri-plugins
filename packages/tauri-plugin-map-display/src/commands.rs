use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::MapDisplayExt;

#[command]
pub(crate) async fn show_map<R: Runtime>(
    app: AppHandle<R>,
    payload: ShowMapRequest,
) -> Result<ShowMapResponse> {
    app.map_display().show_map(payload)
}

#[command]
pub(crate) async fn hide_map<R: Runtime>(app: AppHandle<R>) -> Result<HideMapResponse> {
    app.map_display().hide_map()
}

#[command]
pub(crate) async fn set_region<R: Runtime>(app: AppHandle<R>, payload: SetRegionRequest) -> Result<SetRegionResponse> {
    app.map_display().set_region(payload)
}