# Tauri Plugin Auth

This plugin provides authentication APIs for Tauri applications, supporting iOS platforms.

## Key Benefits

One of the key benefits of this authentication plugin is its seamless integration with the iOS keychain:

1. **Keychain Integration**: The plugin leverages ASWebAuthenticationSession, which automatically uses the iOS keychain for password management. This means:
   - Passwords can be securely saved in the device's keychain.
   - Users can take advantage of AutoFill functionality for faster and more secure logins.
   - If a user has previously logged in to your service in Safari or another app, the credentials can be automatically suggested, improving the user experience.

2. **Security**: By using the system's built-in authentication session, the plugin ensures that the authentication process happens in a secure, sandboxed environment, separate from your app's process.

3. **Consistency**: The authentication flow presents a familiar interface to users, consistent with other iOS apps, which can increase trust and ease of use.

## Installation

### Install the Core Plugin

Install the Core plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-auth = { git = "https://github.com/inkibra/tauri-plugins", tag = "@inkibra/tauri-plugin-auth@VERSION", package="tauri-plugin-auth" }
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
npm add @inkibra/tauri-plugin-auth
# or
yarn add @inkibra/tauri-plugin-auth
# or
pnpm add @inkibra/tauri-plugin-auth
```

Note: If you're using a monorepo with Lerna, make sure to set the registry in your root `.npmrc` file or use Lerna's scoped registry configuration.

## Usage

First, you need to register the core plugin with Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_auth::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterward, you can use the plugin in your JavaScript code:

```typescript
import { authenticate } from '@inkibra/tauri-plugin-auth';

// Authenticate the user
const authResult = await authenticate({
  authUrl: 'https://your-auth-url.com',
  callbackScheme: 'your-app-scheme'
});
```

### Available Functions

1. `authenticate(args: AuthenticateArgs): Promise<AuthResult>`
   - Initiates the authentication process using ASWebAuthenticationSession.

### Types

```typescript:packages/tauri-plugin-auth/index.ts
startLine: 3
endLine: 12
```

## iOS Setup

1. Ensure you have set up your authentication server and configured the necessary URLs.
2. In your Xcode project, add your app's custom URL scheme to the "URL Types" section in the "Info" tab of your target's settings.

## Example

Here's a basic example of how to use the plugin:

```typescript
import { authenticate } from '@inkibra/tauri-plugin-auth';

async function performAuthentication() {
  const authResult = await authenticate({
    authUrl: 'https://your-auth-server.com/login',
    callbackScheme: 'your-app-scheme'
  });

  if (authResult.success) {
    console.log('Authentication successful. Token:', authResult.token);
    // Handle successful authentication
  } else {
    console.error('Authentication failed:', authResult.error);
    // Handle authentication failure
  }
}
```

When using this authentication plugin, your server needs to handle the authentication process and redirect back to your app using the custom URL scheme. Here's a simple example of how you might set up a Node.js server to handle this:

Here's just the sample Node.js server code for URL scheme redirection:

```javascript
const express = require('express');
const app = express();

// Your authentication logic goes here
function authenticateUser(username, password) {
  // Implement your authentication logic
  // Return a token if authentication is successful
}

app.get('/login', (req, res) => {
  const { username, password, redirect_uri } = req.query;
  
  try {
    const token = authenticateUser(username, password);
    
    // Assuming the redirect_uri is your app's custom URL scheme
    // For example: your-app-scheme://auth-callback
    const redirectUrl = `${redirect_uri}?token=${token}`;
    
    res.redirect(redirectUrl);
  } catch (error) {
    const errorRedirectUrl = `${redirect_uri}?error=${encodeURIComponent(error.message)}`;
    res.redirect(errorRedirectUrl);
  }
});

app.listen(3000, () => {
  console.log('Authentication server running on port 3000');
});
```

This code provides a basic example of how a Node.js server might handle authentication and redirect back to your app using a custom URL scheme.

## Security

This plugin follows Tauri's security recommendations and implements the following features:

- Uses ASWebAuthenticationSession for secure authentication flow
- Sandboxed execution on supported platforms

## Contributing

We welcome contributions to this plugin! Please feel free to submit issues and pull requests.

## License

This plugin is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Acknowledgments

This plugin is maintained by the Inkibra team and the Tauri community.
