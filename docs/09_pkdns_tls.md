# PKDNS TLS

PKDNS TLS (Public Key DNS over TLS) provides a secure DNS-over-TLS service for resolving PKARR records. It allows clients to query DNS records using TLS-encrypted connections and enables servers to host PKDNS services.

## Connecting to a PKDNS TLS Endpoint

To connect to a PKDNS TLS endpoint, you can use a Pkarr public key as the URL with `reqwest`. The PKARR client integration with `reqwest` automatically handles the DNS resolution and TLS connection.

TODO

This example demonstrates:

- Using a Pkarr public key as a URL (e.g., `https://<public_key>/`)
- Creating a `reqwest` client with PKDNS TLS support using the PKARR client
- Making HTTP requests over the TLS-secured connection to the Pkarr domain

## Serving a PKDNS TLS Service

To serve your own PKDNS TLS service, you can run an HTTP server with TLS using your keypair and publish the server information to the DHT. This makes your service accessible via its public key.

TODO

This example demonstrates:

- Generating a keypair for the server identity
- Publishing server DNS records (HTTPS SVCB and address records) to the DHT
- Running an HTTPS server with TLS using the keypair's self-signed certificate
- Making the service accessible via `https://<public_key>/`

!!! note
    When deploying a PKDNS TLS service in production, ensure you:

    - Republish the DNS records periodically (recommended: every hour) and when the server address changes
    - Use a public IP address if you want the service to be accessible from other networks
    - Configure appropriate timeout and connection limits
    - Implement rate limiting to prevent abuse
    - Monitor and log service activity for security and debugging
