# PKDNS

PKDNS (Public Key DNS) allows you to resolve and publish `_pubky` records that map public keys to homeservers.

## Generic PKDNS Client

You can access the PKDNS client through a signer to perform PKDNS operations.

```rust
--8<-- "snippets/pubky/src/lib.rs:pkdns_generic"
```

The PKDNS client provides methods for resolving and publishing homeserver records.

## Homeserver Operations

PKDNS is primarily used for homeserver discovery and management. You can resolve other users' homeservers and publish your own.

```rust
--8<-- "snippets/pubky/src/lib.rs:pkdns_homeserver"
```

### Operations

- **Resolve another user's homeserver**: Use `pubky.get_homeserver_of(&user_id)` to find where a user's data is hosted
- **Publish your homeserver (if stale)**: Use `pkdns().publish_homeserver_if_stale(None)` to update your record only when needed
- **Force republish**: Use `pkdns().publish_homeserver_force(Some(&new_homeserver))` when migrating to a new homeserver
- **Fetch your own record**: Use `pkdns().get_homeserver()` to retrieve your current homeserver information

The homeserver record is published via PKARR and cached by the DHT, allowing other users to discover where your data is stored.
