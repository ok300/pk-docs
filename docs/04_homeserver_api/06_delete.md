# DELETE - Remove Data

To delete data from a homeserver, use the `delete` method on the session's storage. Like PUT, this requires an authenticated session.

```rust
--8<-- "snippets/pubky/src/lib.rs:delete"
```

The DELETE operation:

- Requires a `PubkySession` obtained from signing up or signing in
- Takes a path (not a full URL) relative to your homeserver
- Removes the data at the specified location
- All operations are authenticated using your session
- Only the owner of the data can delete it
