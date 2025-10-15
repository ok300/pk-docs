# Get Started

## Add Dependency

Add the dependencies for `pkarr` to your project's `Cargo.toml`:

To use both the DHT and relays (recommended):
```toml
--8<-- "snippets/pkarr/Cargo.toml:pkarr-deps"
```

To use only the DHT:
```toml
--8<-- "snippets/pkarr-dht-only/Cargo.toml:pkarr-deps-dht-only"
```

To use only relays:
```toml
--8<-- "snippets/pkarr-relays-only/Cargo.toml:pkarr-deps-relays-only"
```

Note the available [feature flags](https://docs.rs/pkarr/latest/pkarr/#feature-flags).
