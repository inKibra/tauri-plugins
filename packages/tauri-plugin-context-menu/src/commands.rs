use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ContextMenuExt;

#[command]
pub(crate) async fn show_context_menu<R: Runtime>(
    app: AppHandle<R>,
    payload: ShowContextMenuRequest,
) -> Result<ShowContextMenuResponse> {
    app.context_menu().show_context_menu(payload)
}
