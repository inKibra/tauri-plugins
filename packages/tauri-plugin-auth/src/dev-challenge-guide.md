# Tauri Plugin Fitness - Developer Project Guide

## Project Overview

Create a new Tauri plugin that interfaces with Apple's HealthKit to record and manage workout data on iOS devices. This plugin will be integrated into the existing Inkibra Tauri plugins ecosystem.

## Project Structure

Based on the existing plugins in the repository, your plugin should follow this structure:

packages/tauri-plugin-fitness/
├── Cargo.toml
├── README.md
├── build.rs
├── ios/
│   ├── Package.swift
│   ├── Sources/
│   │   └── FitnessPlugin.swift
│   └── Tests/
└── src/
    ├── lib.rs
    ├── mobile.rs
    ├── commands.rs
    └── models.rs

## Key Requirements

1. **iOS Integration**
   - Implement HealthKit integration using `HKWorkoutBuilder`
   - Handle permissions for HealthKit access
   - Record workout data

2. **Plugin Commands**
   - `check_permissions`: Check HealthKit authorization status
   - `request_permissions`: Request HealthKit access
   - `record_workout`: Record a workout session

## Implementation Steps

### 1. Create Basic Plugin Structure

Create the plugin following the pattern shown in other plugins. Reference the auth plugin structure:

```rust
// src/lib.rs
use tauri::{
  plugin::{Builder, TauriPlugin}, // Import necessary Tauri plugin components
  Manager, Runtime, // Import Manager and Runtime traits
};

pub use models::*; // Re-export models for external use

#[cfg(desktop)]
mod desktop; // Desktop-specific implementation
#[cfg(mobile)]
mod mobile; // Mobile-specific implementation

mod commands; // Module for command definitions
mod error; // Module for error handling
mod models; // Module for data models

pub use error::{Error, Result}; // Re-export error types

#[cfg(desktop)]
use desktop::Auth; // Use desktop Auth implementation
#[cfg(mobile)]
use mobile::Auth; // Use mobile Auth implementation

// Trait to extend Tauri's App with Auth functionality
pub trait AuthExt<R: Runtime> {
  fn auth(&self) -> &Auth<R>; // Method to access Auth
}

// Implementation of AuthExt for any type that implements Manager
impl<R: Runtime, T: Manager<R>> crate::AuthExt<R> for T {
  fn auth(&self) -> &Auth<R> {
    self.state::<Auth<R>>().inner() // Access the Auth state
  }
}

// Function to initialize the plugin
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("auth") // Create a new plugin builder
    .invoke_handler(tauri::generate_handler![ // Define command handlers
      commands::authenticate,
    ])
    .setup(|app, api| { // Setup function for the plugin
      #[cfg(mobile)]
      let auth = mobile::init(app, api)?; // Initialize mobile Auth
      #[cfg(desktop)]
      let auth = desktop::init(app, api)?; // Initialize desktop Auth
      app.manage(auth); // Manage the Auth state in the app
      Ok(()) // Return success
    })
    .build() // Build the plugin
}
```

### 2. Setup iOS Package

Create a Swift package similar to other plugins. Reference:

```swift
// ios/Package.swift
// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription // Import the PackageDescription module

let package = Package(
    name: "tauri-plugin-auth", // Name of the package
    platforms: [
        .macOS(.v10_13), // Supported macOS version
        .iOS(.v13), // Supported iOS version
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-auth", // Name of the library
            type: .static, // Static library type
            targets: ["tauri-plugin-auth"]), // Target for the library
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api") // Dependency on Tauri API
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-auth", // Name of the target
            dependencies: [
                .byName(name: "Tauri") // Dependency on Tauri
            ],
            path: "Sources") // Path to the source files
    ]
)
```

### 3. Implement HealthKit Integration
Create `FitnessPlugin.swift` with these key components:

```swift
// ios/Sources/FitnessPlugin.swift
import HealthKit // Import HealthKit framework
import SwiftRs // Import SwiftRs for Tauri integration
import Tauri // Import Tauri framework

class FitnessPlugin: Plugin { // Define the FitnessPlugin class
    private let healthStore = HKHealthStore() // Create an instance of HKHealthStore
    
    @objc public func checkPermissions(_ invoke: Invoke) throws {
        // Check authorization status for HealthKit
    }
    
    @objc public func requestPermissions(_ invoke: Invoke) throws {
        // Request HealthKit permissions
    }
    
    @objc public func recordWorkout(_ invoke: Invoke) throws {
        // Implement HKWorkoutBuilder to record workouts
    }
}
```

### 4. Integration with Example App

Add the plugin to the example app following the pattern shown here:

```rust
// src-tauri/src/lib.rs
pub fn run() {
    tauri::Builder::default() // Start building the Tauri application
        .invoke_handler(tauri::generate_handler![greet]) // Define command handlers
        .plugin(tauri_plugin_sharing::init()) // Initialize sharing plugin
        .plugin(tauri_plugin_context_menu::init()) // Initialize context menu plugin
        .plugin(tauri_plugin_map_display::init()) // Initialize map display plugin
        .plugin(tauri_plugin_haptic_feedback::init()) // Initialize haptic feedback plugin
        .plugin(tauri_plugin_geolocation::init()) // Initialize geolocation plugin
        .plugin(tauri_plugin_iap::init()) // Initialize in-app purchases plugin
        .plugin(tauri_plugin_auth::init()) // Initialize authentication plugin
        .plugin(tauri_plugin_notifications::init()) // Initialize notifications plugin
        .run(tauri::generate_context!()) // Run the application with generated context
        .expect("error while running tauri application"); // Handle any errors
}
```

## Testing Requirements

1. **Permission Handling**
   - Test permission checks and requests
   - Verify proper error handling

2. **Workout Recording**
   - Test workout data recording
   - Verify HealthKit integration

3. **Integration Testing**
   - Test plugin in example app
   - Verify all commands work as expected

## Deliverables

1. Complete plugin implementation
2. Documentation in README.md
3. Integration with example app
4. Unit tests
5. Example usage code

## Resources

- Tauri Plugin Development Guide: [Tauri Docs](https://tauri.app/develop/plugins/)
- HealthKit Documentation: [Apple Developer](https://developer.apple.com/documentation/healthkit)
- Example Plugin Structure: See auth plugin implementation in the repository

## Success Criteria

- All commands working correctly
- Proper error handling
- Successful HealthKit integration
- Clean code following repository patterns
- Comprehensive documentation
- Working integration in example app

This project tests understanding of:

- Tauri plugin development
- iOS/HealthKit integration
- Rust/Swift interop
- Mobile permissions handling
- API design
- Documentation
