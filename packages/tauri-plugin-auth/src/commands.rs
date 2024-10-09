use tauri::{AppHandle, command, Runtime};

use crate::{models::*, Result, AuthExt};

#[command]
pub(crate) async fn authenticate<R: Runtime>(
    app_handle: AppHandle<R>,
    payload: AuthenticateArgs,
) -> Result<AuthResult> {
    app_handle.auth().authenticate(payload)
}
