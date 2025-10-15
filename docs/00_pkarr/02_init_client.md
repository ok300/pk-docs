# Initialize Client

To be able to re-use the client's internal caches, it's best practice to use a singleton pattern and store the client instance in an `Arc`:

```rust
--8<-- "snippets/pkarr/src/main.rs:init_pkarr_client"
```

The client builder allows you to optionally customize the client's behavior. For example, the amount of DHT records to cache, as well as their TTL:

```rust
--8<-- "snippets/pkarr/src/main.rs:init_pkarr_client_with_opts"
```

## Relays-only Mode

If you're planning to use the client in an environment with no UDP, or behind a NAT, your PKARR client won't be able to directly query the DHT and will instead rely on relays:

```rust
--8<-- "snippets/pkarr-relays-only/src/main.rs:init_pkarr_client_relays_only"
```

WASM applications fall in this scenario.

## DHT-only Mode

Conversely, if you want your client to exclusively rely on direct DHT queries, you can disable the relays:

```rust
--8<-- "snippets/pkarr-dht-only/src/main.rs:init_pkarr_client_dht_only"
```

Note however that direct DHT queries will likely be slower than if relays were enabled as well.
