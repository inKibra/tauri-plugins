use tauri::{AppHandle, command, Runtime};

use crate::{models::*, Result, IAPExt};

#[command]
pub(crate) async fn fetch_products<R: Runtime>(
    app_handle: AppHandle<R>,
    payload: FetchProductsArgs,
) -> Result<Vec<Product>> {
    app_handle.iap().fetch_products(payload)
}

#[command]
pub(crate) async fn purchase_product<R: Runtime>(
    app_handle: AppHandle<R>,
    payload: PurchaseProductArgs,
) -> Result<PurchaseResult> {
    app_handle.iap().purchase_product(payload)
}

#[command]
pub(crate) async fn restore_purchases<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<RestoreResult> {
    app_handle.iap().restore_purchases()
}