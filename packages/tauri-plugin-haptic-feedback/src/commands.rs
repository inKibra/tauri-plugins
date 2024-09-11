use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::HapticFeedbackExt;

#[command]
pub(crate) async fn vibrate<R: Runtime>(
    app: AppHandle<R>,
    payload: HapticVibrateRequest,
) -> Result<HapticResponse> {
    app.haptic_feedback().vibrate(payload)
}

#[command]
pub(crate) async fn impact_feedback<R: Runtime>(app: AppHandle<R>, payload: ImpactFeedbackRequest) -> Result<HapticResponse> {
    app.haptic_feedback().impact_feedback(payload)
}

#[command]
pub(crate) async fn selection_feedback<R: Runtime>(app: AppHandle<R>) -> Result<HapticResponse> {
    app.haptic_feedback().selection_feedback()
}