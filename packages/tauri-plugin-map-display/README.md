# Tauri Plugin Map Display

This plugin provides APIs for displaying and interacting with maps in Tauri applications, supporting iOS platforms.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-map-display = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-map-display@VERSION", package="tauri-plugin-map-display" }
```

### Install JavaScript Guest Bindings

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
npm add @inkibra/tauri-plugin-map-display
# or
yarn add @inkibra/tauri-plugin-map-display
# or
pnpm add @inkibra/tauri-plugin-map-display
```

Note: If you're using a monorepo with Lerna, make sure to set the registry in your root `.npmrc` file or use Lerna's scoped registry configuration.

## Usage

First, you need to register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_map_display::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterward, you can use the plugin in your JavaScript code to interact with maps.

### Available Functions

Based on the provided Swift code, the plugin likely supports the following functions:

1. `showMap(options: ShowMapRequest): Promise<ShowMapResponse>`
   - Displays a map with the specified options.

2. `hideMap(): Promise<HideMapResponse>`
   - Hides the currently displayed map.

3. `setRegion(options: SetRegionRequest): Promise<SetRegionResponse>`
   - Sets the region of the map to display.

## iOS Setup

1. Ensure you have set up your Xcode project correctly.
2. The plugin uses `MapKit`, so make sure your app has the necessary permissions and capabilities enabled in Xcode.

## Example

Here's a basic example of how to use the plugin:

```javascript
import { showMap, setRegion } from '@inkibra/tauri-plugin-map-display';

// Show a map
const showMapResult = await showMap({
  frame: { x: 0, y: 0, width: 300, height: 300 },
  mapType: 'standard',
  showsUserLocation: true
});

if (showMapResult.success) {
  console.log('Map displayed successfully');

  // Set the map region
  const setRegionResult = await setRegion({
    region: {
      center: { latitude: 37.7749, longitude: -122.4194 },
      span: { latitudeDelta: 0.1, longitudeDelta: 0.1 }
    }
  });

  if (setRegionResult.success) {
    console.log('Map region set successfully');
  }
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
