# Tauri Plugin OTA (Over-the-Air Updates)

This plugin provides over-the-air (OTA) update capabilities for Tauri iOS applications. It allows updating JavaScript and other frontend assets after App Store approval without submitting a new binary.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-ota = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-ota@VERSION", package="tauri-plugin-ota" }
```

### Install JavaScript Guest Bindings

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
npm add @inkibra/tauri-plugin-ota
# or
yarn add @inkibra/tauri-plugin-ota
# or
pnpm add @inkibra/tauri-plugin-ota
```

## Usage

First, you need to register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_ota::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### "Splash-then-Update" Architecture

The OTA plugin works with a "splash-then-update" architecture:

1. App launches and displays a bundled splash screen
2. The splash screen calls `prepare()` to check for updates
3. If updates are available, they are downloaded and returned directly in the response
4. The splash screen calls `start()` to load either the updated or bundled JavaScript content
5. The JavaScript content calls `register()` to run its startup logic

### Complete Example Flow

#### 1. Splash Screen (splash.html)

The splash screen is bundled with your app and handles the update process:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Updating...</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {
            font-family: system-ui;
            text-align: center;
            padding: 20px;
            background-color: #f5f5f7;
            color: #1d1d1f;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }
        .logo {
            width: 80px;
            height: 80px;
            margin-bottom: 20px;
        }
        .progress-container {
            width: 80%;
            max-width: 300px;
            height: 8px;
            background-color: #d1d1d6;
            border-radius: 4px;
            overflow: hidden;
            margin: 20px 0;
        }
        .progress-bar {
            height: 100%;
            width: 0%;
            background-color: #0071e3;
            transition: width 0.3s;
        }
        .status-text {
            font-size: 14px;
            margin-top: 10px;
        }
        .retry-button {
            display: none;
            margin-top: 20px;
            padding: 8px 16px;
            background-color: #0071e3;
            color: white;
            border: none;
            border-radius: 4px;
            font-size: 14px;
            cursor: pointer;
        }
    </style>
</head>
<body>
    <img src="logo.png" alt="App Logo" class="logo">
    <h1>Loading App</h1>
    <div class="progress-container">
        <div class="progress-bar"></div>
    </div>
    <div class="status-text">Checking for updates...</div>
    <button class="retry-button" onclick="startApp()">Retry</button>

    <script>
        document.addEventListener('DOMContentLoaded', () => {
            startApp();
        });

        async function startApp() {
            try {
                // Hide retry button if visible
                document.querySelector('.retry-button').style.display = 'none';
                
                // Set initial progress
                document.querySelector('.progress-bar').style.width = '10%';
                document.querySelector('.status-text').textContent = 'Checking for updates...';
                
                // Import the OTA plugin
                const { prepare, start } = await import('@inkibra/tauri-plugin-ota');
                
                try {
                    // Check for updates
                    document.querySelector('.progress-bar').style.width = '30%';
                    const updateInfo = await prepare("https://your-cdn.com/app/manifest.json");
                    
                    // Show update info if available
                    if (updateInfo.update) {
                        document.querySelector('.status-text').textContent = 
                            `Update found: ${updateInfo.manifest?.version || 'Unknown version'}`;
                    } else {
                        document.querySelector('.status-text').textContent = 'Using bundled version';
                    }
                    
                    document.querySelector('.progress-bar').style.width = '60%';
                } catch (err) {
                    console.warn("Update preparation failed:", err);
                    document.querySelector('.status-text').textContent = `Update check failed: ${err.message}`;
                    // Continue anyway - we'll use bundled assets
                }
                
                // Start the app - this will load either updated content or bundled content
                document.querySelector('.progress-bar').style.width = '90%';
                document.querySelector('.status-text').textContent = 'Starting application...';
                
                await start();
                
                // At this point, the app.js will take over
                document.querySelector('.progress-bar').style.width = '100%';
                
            } catch (err) {
                console.error('Error starting app:', err);
                // Show error and retry button
                document.querySelector('.status-text').textContent = `Error: ${err.message || 'Unknown error'}`;
                document.querySelector('.retry-button').style.display = 'block';
            }
        }
    </script>
</body>
</html>
```

#### 2. Application Entry Point (app.js)

This is your main application code that gets either bundled with the app or updated via OTA:

```javascript
// app.js - Your main application code
import { register } from '@inkibra/tauri-plugin-ota';

// Register the startup handler that will run when the script is loaded
register(() => {
  // This is the entry point of your application
  console.log('Application started!');
  
  // Initialize your application
  initApp();
});

function initApp() {
  // Your application initialization code
  document.body.innerHTML = `
    <div id="app">
      <h1>My Awesome App</h1>
      <p>Version: 1.0.1</p>
      <button id="action-button">Click Me</button>
    </div>
  `;
  
  // Add event listeners
  document.getElementById('action-button').addEventListener('click', () => {
    alert('Button clicked!');
  });
  
  // Load your app modules, set up routing, etc.
}

// Other application code...
```

### JavaScript API

The plugin provides a simple API with three main functions:

```typescript
/**
 * Prepare the app by checking for updates and applying them if available
 * @param manifestUrl URL to the update manifest JSON
 * @returns Information about the update status including content if update is available
 */
async function prepare(manifestUrl: string): Promise<{
  update?: string;          // The actual JavaScript content if an update is available
  manifest?: {              // The manifest data if available
    version: string;
    url: string;
    hash: string;
    notes?: string;
  };
  error?: string;           // Error message if something went wrong
}>

/**
 * Start the app by loading the appropriate JavaScript file
 * Using Blob for dynamic loading if update is available
 */
async function start(): Promise<void>

/**
 * Register a startup handler that will be called when the app starts
 * @param handler Function to call when the app starts
 */
function register(handler: () => void | Promise<void>): void
```

## CDN Requirements

The plugin expects your CDN to host:

1. **Manifest File** (`manifest.json`):
   ```json
   {
     "version": "1.0.1",
     "url": "https://your-cdn.com/updates/app-1.0.1.js",
     "hash": "sha256-hash-of-javascript-file",
     "notes": "Optional release notes"
   }
   ```

2. **JavaScript File**:
   - A single JavaScript file containing your application code
   - Should be compiled/bundled with all dependencies 
   - Must call the `register()` function with your app's startup logic

## How It Works

1. **Prepare Phase**:
   - Fetches manifest from CDN
   - Compares versions
   - Downloads new JavaScript if newer version available
   - Verifies SHA-256 hash for security
   - Returns JavaScript content directly in response
   - Stores content in app's resource directory for offline use

2. **Start Phase**:
   - Uses Blob API to create a script URL from the update content
   - Dynamically loads the script via a `<script>` tag
   - Falls back to bundled app.js if no update is available

3. **Register Phase**:
   - The loaded JavaScript must call `register()` with its startup function
   - This ensures the app's code runs after everything is properly loaded

## App Store Compliance

This plugin is designed to be fully App Store compliant by:

- Only updating content that runs within the WebView sandbox
- Not executing dynamic code outside the WebView
- Not changing the app's functionality in a way that would violate App Store guidelines

## Security

The plugin implements several security features:

1. **Hash Verification**: Ensures downloaded updates are authentic and unmodified
2. **Fallback Mechanism**: Always maintains bundled assets as a fallback if updates fail
3. **App Store Compliance**: Only updates WebView content, not native code

## Contributing

We welcome contributions to this plugin! Please feel free to submit issues and pull requests.

## License

This plugin is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

This plugin is maintained by the Inkibra team and the Tauri community.
