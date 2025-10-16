# Storage API

The Pubky SDK provides two ways to access storage: authenticated (session-based) and public (read-only).

## Session Storage (Authenticated)

Session storage allows you to read and write your own data using an authenticated session. This is used when you need to store or modify data on your homeserver.

```rust
--8<-- "snippets/pubky/src/main.rs:storage_session"
```

Key features:
- Uses absolute paths like `"/pub/my.app/data.txt"`
- Requires authentication via a `PubkySession`
- Supports read and write operations
- All operations are authenticated using your session

## Public Storage (Read-Only)

Public storage allows you to read publicly accessible data from any user without authentication.

```rust
--8<-- "snippets/pubky/src/main.rs:storage_public"
```

Key features:
- Uses addressed format like `"pubky://<user>/pub/app/file.txt"`
- No authentication required
- Read-only access
- Can list directory contents with pagination
- Works with any user's public data

**Convention:** Store your app's public data under a domain-like folder in `/pub`, for example `/pub/mycoolnew.app/`.
