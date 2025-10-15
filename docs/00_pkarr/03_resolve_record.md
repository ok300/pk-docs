To resolve a record, you need a `PublicKey`. The `resolve` method on the `Client` will return a `SignedPacket` if one is found for the given `PublicKey`.

```rust
--8<-- "snippets/pkarr/src/main.rs:resolve_record"
```