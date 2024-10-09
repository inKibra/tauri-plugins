# Tauri Plugin IAP (In-App Purchases)

This plugin provides APIs for handling in-app purchases in Tauri applications, supporting iOS platforms.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-iap = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-iap@VERSION", package="tauri-plugin-iap" }
```

### Install JavaScript Guest Bindings

Before installing the JavaScript bindings, you need to configure npm to use the GitHub Packages registry for the `@inkibra` scope. Run the following command:

```sh
npm config set @inkibra:registry https://npm.pkg.github.com
```

For yarn users:

```sh
yarn config set @inkibra:registry https://npm.pkg.github.com
```

For pnpm users:

```sh
pnpm config set @inkibra:registry https://npm.pkg.github.com
```

After setting the registry, you can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
npm add @inkibra/tauri-plugin-iap
# or
yarn add @inkibra/tauri-plugin-iap
# or
pnpm add @inkibra/tauri-plugin-iap
```

Note: If you're using a monorepo with Lerna, make sure to set the registry in your root `.npmrc` file or use Lerna's scoped registry configuration.

## Usage

First, you need to register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_iap::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterward, you can use the plugin in your JavaScript code:

```typescript
import { fetchProducts, purchaseProduct, restorePurchases } from '@inkibra/tauri-plugin-iap';

// Fetch available products
const products = await fetchProducts(['com.example.product1', 'com.example.product2']);

// Purchase a product
const purchaseResult = await purchaseProduct('com.example.product1');

// Restore purchases
const restoreResult = await restorePurchases();
```

### Available Functions

1. `fetchProducts(productIds: string[]): Promise<ProductInfo[]>`
   - Fetches information about products from the App Store.

2. `purchaseProduct(productId: string): Promise<PurchaseResult>`
   - Initiates a purchase for a specific product.

3. `restorePurchases(): Promise<RestoreResult>`
   - Restores previously purchased products.

### Types

```typescript
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
```

## iOS Setup

1. Ensure you have set up your App Store Connect account and configured your in-app purchases.
2. In your Xcode project, enable the "In-App Purchase" capability.
3. If you're using StoreKit configuration files for testing, make sure they are properly set up in your Xcode project.

## Example

Here's a basic example of how to use the plugin:

```typescript
import { fetchProducts, purchaseProduct, restorePurchases } from '@inkibra/tauri-plugin-iap';

// Fetch available products
const products = await fetchProducts(['com.example.product1', 'com.example.product2']);
console.log('Available products:', products);

// Purchase a product
const purchaseResult = await purchaseProduct('com.example.product1');
if (purchaseResult.success) {
  console.log('Purchase successful:', purchaseResult.product);
} else {
  console.error('Purchase failed:', purchaseResult.error);
}

// Restore purchases
const restoreResult = await restorePurchases();
if (restoreResult.success) {
  console.log('Restored products:', restoreResult.restoredProducts);
} else {
  console.error('Restore failed:', restoreResult.error);
}
```

## Server-Side Subscription Validation

For secure and up-to-date validation of subscription statuses, it's crucial to implement server-side verification. Apple provides the App Store Server API for this purpose.

### Checking Subscription Status

To verify the validity of a subscription on your server, you should use the "Get All Subscription Statuses" endpoint from the App Store Server API. This allows you to retrieve the current status of all of a customer's auto-renewable subscriptions in your app.

For detailed information and implementation guidelines, please refer to Apple's official documentation:

[Get All Subscription Statuses](https://developer.apple.com/documentation/appstoreserverapi/get_all_subscription_statuses/)

Key points to consider:

- This API requires server-to-server communication and cannot be called directly from your app.
- You'll need to set up and configure App Store Connect API keys for authentication.
- The API provides comprehensive information about subscription statuses, including expiration dates, renewal information, and grace periods.

Implementing server-side validation ensures that your app has the most accurate and secure information about a user's subscription status, protecting against potential client-side tampering or outdated information.

## Security

This plugin follows Tauri's security recommendations and implements the following features:

- Permission-based access to plugin functionality
- Sandboxed execution on supported platforms

For more details, check the `permissions` folder in the plugin's repository.

## Contributing

We welcome contributions to this plugin! Please feel free to submit issues and pull requests.

## License

This plugin is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

This plugin is maintained by the Inkibra team and the Tauri community.
