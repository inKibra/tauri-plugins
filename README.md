# Inkibra Tauri Plugins

This repository contains a collection of Tauri plugins developed by Inkibra. These plugins extend the functionality of Tauri applications, providing various features for iOS platforms.

## Available Plugins

### 1. [Auth Plugin](packages/tauri-plugin-auth/README.md)

Provides authentication APIs for Tauri applications, with seamless integration with iOS keychain.

### 2. [IAP (In-App Purchases) Plugin](packages/tauri-plugin-iap/README.md)

Handles in-app purchases in Tauri applications for iOS platforms.

### 3. [Sharing Plugin](packages/tauri-plugin-sharing/README.md)

Enables sharing content from your Tauri application on iOS platforms.

### 4. [Map Display Plugin](packages/tauri-plugin-map-display/README.md)

Provides APIs for displaying and interacting with maps in Tauri applications on iOS.

## Installation

Each plugin can be installed separately. Please refer to the individual plugin READMEs for specific installation instructions.

## General Setup

Before installing any of the plugins, you need to configure npm to use the GitHub Packages registry for the `@inkibra` scope:

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

## Usage

Each plugin has its own usage instructions. Please refer to the individual plugin READMEs for detailed usage examples and API documentation.

## Contributing

We welcome contributions to these plugins! Please feel free to submit issues and pull requests for any of the plugins in this repository.

## License

These plugins are licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

These plugins are maintained by the Inkibra team and the Tauri community.
