# QR Auth

QR Auth enables keyless authentication for third-party apps. Users can authorize apps by scanning a QR code or clicking a deeplink with their signing device (like Pubky Ring).

## Authorization Flow

The QR auth flow consists of three main steps: building capabilities, getting the authorization URL, and awaiting approval.

```rust
--8<-- "snippets/pubky/src/lib.rs:qr_auth"
```

## How It Works

1. **Build Capabilities**: Define what permissions your app needs using `Capabilities::builder()`. Use methods like `read_write()` to specify access patterns for your app's routes.

2. **Start Auth Flow**: Call `pubky.start_auth_flow(&caps)` to initiate the authorization process. This returns a `PubkyAuthFlow` object.

3. **Get Authorization URL**: Use `flow.authorization_url()` to get a URL that can be displayed as a QR code or sent as a deeplink. Users scan this with their signing device (e.g., [Pubky Ring](https://github.com/pubky/pubky-ring)).

4. **Await Approval**: Call `flow.await_approval()` to wait for the user to approve the request on their signing device. This returns a `PubkySession` that's bound to the authorized capabilities.

## Typical Usage

This pattern is ideal for:

- **Third-party apps** that need to access user data
- **Keyless applications** where the app doesn't hold the user's private keys
- **Cross-device authentication** where users sign in on one device to authorize another

The signing device (like Pubky Ring) holds the user's keys and approves the authorization request, providing the app with a session-bound credential.
