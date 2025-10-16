# PUT - Store Data

To store data on a homeserver, use the `put` method on the session's storage. In Pubky, you work with a `PubkySession` that provides authenticated access.

```rust
--8<-- "snippets/pubky/src/main.rs:put"
```

The PUT operation:

- Requires a `PubkySession` obtained from signing up or signing in
- Takes a path (not a full URL) relative to your homeserver
- Accepts data in various formats (strings, bytes, etc.)
- Creates or overwrites the data at the specified path
- All operations are authenticated using your session
