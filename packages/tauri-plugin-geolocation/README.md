![geolocation](https://github.com/tauri-apps/plugins-workspace/raw/v2/plugins/geolocation/banner.png)

This plugin provides APIs for getting and tracking the device's current position, including information about altitude, heading, and speed (if available).

## Install

_This plugin requires a Rust version of at least **1.75**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-iap = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-geolocation@VERSION", package="tauri-plugin-geolocation" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:


```sh
npm add @inkibra/tauri-plugin-geolocation
# or
yarn add @inkibra/tauri-plugin-geolocation
# or
pnpm add @inkibra/tauri-plugin-geolocation
```

## Setting up

### iOS

Apple requires privacy descriptions to be specified in `Info.plist` for location information:

- `NSLocationWhenInUseDescription`

### Android

This plugin automatically adds the following permissions to your `AndroidManifest.xml` file:

```xml
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
```

If your app requires GPS functionality to function, **you** should add the following to your `AndroidManifest.xml` file:

```xml
<uses-feature android:name="android.hardware.gps" android:required="true" />
```

The Google Play Store uses this property to decide whether it should show the app to devices without GPS capabilities.

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_geolocation::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import {
  checkPermissions,
  requestPermissions,
  getCurrentPosition,
  watchPosition
} from '@tauri-apps/plugin-log'

let permissions = await checkPermissions()
if (
  permissions.location === 'prompt' ||
  permissions.location === 'prompt-with-rationale'
) {
  permissions = await requestPermissions(['location'])
}

if (permissions.location === 'granted') {
  const pos = await getCurrentPosition()

  await watchPosition(
    { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },
    (pos) => {
      console.log(pos)
    }
  )
}
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## Contributed By

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://crabnebula.dev" target="_blank">
          <img src="contributors/crabnebula.svg" alt="CrabNebula" width="283">
        </a>
      </td>
      <td align="center" valign="middle">
        <a href="https://rescue.co" target="_blank">
            <img src="contributors/rescue.png" alt="Rescue.co" width="283" height="90">
        </a>
      </td>
    </tr>
  </tbody>
</table>

## Partners

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://crabnebula.dev" target="_blank">
          <img src="https://github.com/tauri-apps/plugins-workspace/raw/v2/.github/sponsors/crabnebula.svg" alt="CrabNebula" width="283">
        </a>
      </td>
    </tr>
  </tbody>
</table>

For the complete list of sponsors please visit our [website](https://tauri.app#sponsors) and [Open Collective](https://opencollective.com/tauri).

## License

Code: (c) 2015 - Present - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.
