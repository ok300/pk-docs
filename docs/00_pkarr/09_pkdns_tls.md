# PKDNS TLS

PKDNS TLS (Public Key DNS over TLS) provides a secure DNS-over-TLS service for resolving PKARR records. It allows clients to query DNS records using TLS-encrypted connections and enables servers to host PKDNS services.

## Connecting to a PKDNS TLS Endpoint

To connect to a PKDNS TLS endpoint, you can use the PKARR client with a relay that supports TLS connections. The client can query DNS records securely through the TLS-enabled relay.

```rust
--8<-- "snippets/pkarr/src/main.rs:connect_pkdns_tls"
```

This example demonstrates:
- Creating a client configured with a PKDNS TLS relay endpoint
- Resolving a public key through the TLS-secured connection
- Handling the response packet

## Serving a PKDNS TLS Service

To serve your own PKDNS TLS service, you need to set up a relay that accepts TLS connections and responds to PKARR resolution requests. This involves configuring a server with TLS certificates and implementing the PKDNS protocol.

```rust
--8<-- "snippets/pkarr/src/main.rs:serve_pkdns_tls"
```

This example demonstrates:
- Setting up a basic PKDNS TLS service configuration
- Listening for TLS-encrypted DNS queries
- Responding to PKARR record resolution requests

!!! note
    When deploying a PKDNS TLS service in production, ensure you:
    
    - Use valid TLS certificates from a trusted certificate authority
    - Configure appropriate timeout and connection limits
    - Implement rate limiting to prevent abuse
    - Monitor and log service activity for security and debugging
