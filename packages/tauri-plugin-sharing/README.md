# Tauri Plugin Sharing

This plugin provides APIs for sharing content from your Tauri application on iOS platforms.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-sharing = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-sharing@VERSION", package="tauri-plugin-sharing" }
```

### Install JavaScript Guest Bindings

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
npm add @inkibra/tauri-plugin-sharing
# or
yarn add @inkibra/tauri-plugin-sharing
# or
pnpm add @inkibra/tauri-plugin-sharing
```

Note: If you're using a monorepo with Lerna, make sure to set the registry in your root `.npmrc` file or use Lerna's scoped registry configuration.

## Usage

First, you need to register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sharing::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterward, you can use the plugin in your JavaScript code:

### Available Functions

1. `share(text: string, url: string): Promise<void>`
   - Shares the provided text and URL using the native iOS sharing sheet.

## iOS Setup

No additional setup is required for iOS. The plugin uses the built-in `UIActivityViewController` for sharing.

## Example

Here's a basic example of how to use the plugin:

```javascript
import { share } from '@inkibra/tauri-plugin-sharing';

// Share some text and a URL
try {
  await share("Check out this cool app!", "https://example.com");
  console.log("Content shared successfully");
} catch (error) {
  console.error("Failed to share content:", error);
}
```

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
