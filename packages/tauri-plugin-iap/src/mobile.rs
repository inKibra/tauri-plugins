use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_iap);

pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<IAP<R>> {
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_iap)?;
  Ok(IAP(handle))
}

pub struct IAP<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> IAP<R> {
  pub fn fetch_products(&self, payload: FetchProductsArgs) -> crate::Result<Vec<Product>> {
    self
      .0
      .run_mobile_plugin("fetchProducts", payload)
      .map_err(Into::into)
  }

  pub fn purchase_product(&self, payload: PurchaseProductArgs) -> crate::Result<PurchaseResult> {
    self
      .0
      .run_mobile_plugin("purchaseProduct", payload)
      .map_err(Into::into)
  }

  pub fn restore_purchases(&self) -> crate::Result<RestoreResult> {
    self
      .0
      .run_mobile_plugin("restorePurchases", ())
      .map_err(Into::into)
  }
}