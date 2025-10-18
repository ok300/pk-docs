# PKDNS Records

A PKARR client can read (resolve) and write (publish) DHT records.


## Resolve Record

To resolve a record, you need a `PublicKey`. The `resolve` method on the `Client` will return a `SignedPacket` if one is found for the given `PublicKey`.

```rust
--8<-- "snippets/pkarr/src/main.rs:resolve_record"
```

## Publish Record

To publish a record, you need a `Keypair` to sign the packet. The `publish` method on the `Client` will publish the `SignedPacket` to the DHT.

```rust
--8<-- "snippets/pkarr/src/main.rs:publish_record"
```
