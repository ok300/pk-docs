To publish a record, you need a `Keypair` to sign the packet. The `publish` method on the `Client` will publish the `SignedPacket` to the DHT.

```rust
--8<-- "snippets/pkarr/src/main.rs:publish_record"
```