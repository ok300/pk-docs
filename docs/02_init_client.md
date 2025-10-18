# Initialize Client

To interact with Pubky homeservers, you need to create a `Pubky` instance. The `Pubky` facade is the main entry point for the SDK and owns the transport layer.

## Basic Client

```rust
--8<-- "snippets/pubky/src/lib.rs:init_pubky_client"
```

## Testnet Mode

For development and testing, you can use testnet mode which connects to test homeservers:

```rust
--8<-- "snippets/pubky/src/lib.rs:init_pubky_client_testnet"
```

The testnet mode is useful for development without affecting production data or requiring production homeserver access.
