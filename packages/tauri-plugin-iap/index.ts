import { invoke } from '@tauri-apps/api/core';

export type ProductInfo = {
  id: string;
  title: string;
  description: string;
  price: string;
  priceLocale: string;
}

export type PurchasedProduct = {
  productId: string;
  transactionId?: string;
  originalTransactionId?: string;
}

export type PurchaseResult = {
  success: boolean;
  product?: PurchasedProduct;
  error?: string;
}

export type RestoreResult = {
  success: boolean;
  restoredProducts: PurchasedProduct[];
  error?: string;
}

export async function fetchProducts(productIds: string[]): Promise<ProductInfo[]> {
  return await invoke('plugin:iap|fetch_products', { payload: { productIds } });
}

export async function purchaseProduct(productId: string): Promise<PurchaseResult> {
  return await invoke('plugin:iap|purchase_product', { payload: { productId } });
}

export async function restorePurchases(): Promise<RestoreResult> {
  return await invoke('plugin:iap|restore_purchases');
}