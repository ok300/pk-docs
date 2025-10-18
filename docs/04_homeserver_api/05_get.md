# GET - Retrieve Data

To retrieve data from a homeserver, use the `get` method on the `PublicStorage` API. This allows you to read data from any user without authentication.

```rust
--8<-- "snippets/pubky/src/lib.rs:get"
```

The GET operation:

- Uses the `public_storage()` API from your `Pubky` instance
- Takes a `pubky://` URL that specifies the public key and path
- Returns the data stored at that location
- Can read data from any public key (if the data is public)
- Returns standard HTTP response with bytes

## Listing Directory Contents

To list the contents of a directory:

```rust
--8<-- "snippets/pubky/src/lib.rs:list"
```

Directory listings work the same way - just use a path ending with `/` to list the contents of that directory.
