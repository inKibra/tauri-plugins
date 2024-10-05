use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<IAP<R>> {
  Ok(IAP(app.clone()))
}

/// Access to the IAP APIs.
pub struct IAP<R: Runtime>(AppHandle<R>);

impl<R: Runtime> IAP<R> {
  pub fn fetch_products(&self, _: FetchProductsArgs) -> crate::Result<Vec<Product>> {
    Ok(vec![])
  }

  pub fn purchase_product(&self, _: PurchaseProductArgs) -> crate::Result<PurchaseResult> {
    Ok(PurchaseResult {
      success: false,
      transaction_id: None,
      error: Some("In-app purchases are not supported on desktop".into()),
    })
  }

  pub fn restore_purchases(&self) -> crate::Result<RestoreResult> {
    Ok(RestoreResult {
      success: false,
      restored_product_ids: vec![],
      error: Some("In-app purchases are not supported on desktop".into()),
    })
  }
}