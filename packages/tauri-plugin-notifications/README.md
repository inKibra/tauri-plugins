# Tauri Plugin Notifications

This plugin provides APIs for handling push notifications in Tauri applications, supporting iOS platforms.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-notifications = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-notifications@VERSION" }
```

### Install JavaScript Guest Bindings

Install the JavaScript bindings:

```sh
npm add @inkibra/tauri-plugin-notifications
# or
yarn add @inkibra/tauri-plugin-notifications
# or
pnpm add @inkibra/tauri-plugin-notifications
```

## Usage

First, register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notifications::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Then use the plugin in your JavaScript code:

```typescript
import { 
  checkPermissions, 
  requestPermissions, 
  registerForRemoteNotifications,
  watchNotifications 
} from '@inkibra/tauri-plugin-notifications';

// Check notification permissions
const permissionStatus = await checkPermissions();

// Request notification permissions
const requestResult = await requestPermissions();

// Register for remote notifications
const registrationResult = await registerForRemoteNotifications();

// Watch for notification events
await watchNotifications((event) => {
  console.log('Notification received:', event);
});
```

### Available Functions

1. `checkPermissions(): Promise<NotificationPermissionStatus>`
   - Checks current notification permission status
   - Returns status as "prompt", "denied", or "granted"

2. `requestPermissions(): Promise<NotificationPermissionStatus>`
   - Requests notification permissions from the user
   - Returns the resulting permission status

3. `checkRegistrationStatus(): Promise<NotificationRegistrationStatus>`
   - Checks if the app is registered for remote notifications
   - Returns registration status and device token if available

4. `registerForRemoteNotifications(): Promise<NotificationRegistrationResult>`
   - Registers the app for remote notifications
   - Returns success status and device token or error message

5. `watchNotifications(callback: (event: NotificationEvent) => void): Promise<WatchNotificationResult>`
   - Sets up a listener for notification events
   - Returns success status of the watch operation

### Types

```typescript
interface NotificationPermissionStatus {
  status: "prompt" | "denied" | "granted";
}

interface NotificationRegistrationStatus {
  isRegistered: boolean;
  token?: string;
}

interface NotificationRegistrationResult {
  success: boolean;
  token?: string;
  error?: string;
}

type NotificationEventType = 
  | "BACKGROUND_TAP"
  | "FOREGROUND_TAP"
  | "FOREGROUND_DELIVERY"
  | "BACKGROUND_DELIVERY";

interface NotificationEvent {
  type: NotificationEventType;
  payload: Record<string, string>;
}

interface WatchNotificationResult {
  success: boolean;
}
```

## iOS Setup

1. Enable Push Notifications capability in your Xcode project
2. Configure your Apple Developer account for push notifications
3. Add required entitlements:
   - `aps-environment` (development or production)
4. Configure your app's Info.plist with required background modes:
   - Remote notifications

## Platform Support

| Platform | Status |
|----------|--------|
| iOS      | ✅     |
| Desktop  | ❌     |

Desktop platforms will return appropriate "not supported" responses for all operations.

## Example

```typescript
import { 
  requestPermissions, 
  registerForRemoteNotifications,
  watchNotifications 
} from '@inkibra/tauri-plugin-notifications';

async function setupNotifications() {
  try {
    // Request permissions
    const permissionResult = await requestPermissions();
    if (permissionResult.status === 'granted') {
      // Register for notifications
      const registration = await registerForRemoteNotifications();
      if (registration.success) {
        console.log('Successfully registered for notifications');
        console.log('Device token:', registration.token);
      } else {
        console.error('Registration failed:', registration.error);
      }
    } else {
      console.log('Notification permissions not granted:', permissionResult.status);
    }
    
    // Set up notification listener
    const watchResult = await watchNotifications((event) => {
      switch (event.type) {
        case 'BACKGROUND_TAP':
          console.log('User tapped notification in background');
          break;
        case 'FOREGROUND_TAP':
          console.log('User tapped notification in foreground');
          break;
        case 'FOREGROUND_DELIVERY':
          console.log('Notification received in foreground');
          break;
        case 'BACKGROUND_DELIVERY':
          console.log('Notification received in background');
          break;
      }
      console.log('Notification payload:', event.payload);
    });
    
    if (watchResult.success) {
      console.log('Successfully set up notification listener');
    }
  } catch (error) {
    console.error('Error setting up notifications:', error);
  }
}

// Call the setup function
setupNotifications();
```

## Contributing

We welcome contributions! Please feel free to submit issues and pull requests.

## License

This plugin is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

This plugin is maintained by the Inkibra team and the Tauri community.
