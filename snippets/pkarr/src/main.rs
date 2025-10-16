use std::{error::Error, net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use pkarr::dns::rdata::SVCB;
use pkarr::InMemoryCache;
use pkarr::{Client, Keypair, PublicKey, SignedPacket};
use reqwest::Method;
use url::Url;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client]
fn init_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder().build()?;
    Ok(client)
}
// --8<-- [end:init_pkarr_client]

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_with_opts]
fn init_client_with_opts() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
        // Set a custom cache size.
        .cache(Arc::new(InMemoryCache::new(5_000.try_into()?)))
        // Set custom relays.
        .relays(&[Url::parse("https://my-relay.com")?])?
        // Set custom bootstrap nodes.
        .bootstrap(&["127.0.0.1:6881"])
        // Set a custom request timeout.
        .request_timeout(Duration::from_secs(10))
        .build()?;

    Ok(client)
}
// --8<-- [end:init_pkarr_client_with_opts]

#[allow(dead_code)]
// --8<-- [start:resolve_record]
async fn resolve_record() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;

    let pk = "9fdaa3b3cb04f24328975a4862419d2a2a46639c33659a101f653457a40b9d16";

    match client.resolve(&pk.parse()?).await {
        Some(signed_packet) => println!("Resolved packet: {signed_packet:?}"),
        None => println!("No record found for the public key"),
    }

    Ok(())
}
// --8<-- [end:resolve_record]

#[allow(dead_code)]
// --8<-- [start:publish_record]
async fn publish_record() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;

    // Generate a new keypair.
    let keypair = Keypair::random();

    // Create a signed packet with a TXT record.
    let signed_packet = SignedPacket::builder()
        .txt("_proto".try_into()?, "foo=bar".try_into()?, 30)
        .build(&keypair)?;

    match client.publish(&signed_packet, None).await {
        Ok(()) => println!("Published successfully!"),
        Err(e) => println!("Failed to publish: {e}"),
    }

    Ok(())
}
// --8<-- [end:publish_record]

#[allow(dead_code)]
// --8<-- [start:connect_pkdns_tls]
async fn connect_pkdns_tls() -> Result<(), Box<dyn Error>> {
    // Example URL using a Pkarr public key (PKDNS TLS)
    let url = "https://9fdaa3b3cb04f24328975a4862419d2a2a46639c33659a101f653457a40b9d16/";

    // Create a Reqwest client with PKDNS TLS support
    let reqwest_client = if PublicKey::try_from(url).is_err() {
        // If it is not a Pkarr domain, use normal Reqwest
        reqwest::Client::new()
    } else {
        // Build a PKARR client
        let client = Client::builder().build()?;
        // Use the PKARR client with Reqwest for PKDNS TLS
        reqwest::ClientBuilder::from(client).build()?
    };

    println!("GET {url}..");
    let response = reqwest_client.request(Method::GET, url).send().await?;

    let body = response.text().await?;
    println!("{body}");

    Ok(())
}
// --8<-- [end:connect_pkdns_tls]

#[allow(dead_code)]
// --8<-- [start:serve_pkdns_tls]
async fn serve_pkdns_tls() -> Result<(), Box<dyn Error>> {
    // Generate a keypair for the server
    let keypair = Keypair::random();
    
    // Create a PKARR client for publishing DNS records
    let client = Client::builder().build()?;
    
    // Define the server address
    let addr: SocketAddr = "127.0.0.1:8443".parse()?;
    
    println!("Server listening on {addr}");
    
    // Publish server information to the DHT
    // This makes the server discoverable via PKDNS
    publish_server_pkarr(&client, &keypair, &addr).await?;
    
    println!("Server running on https://{}", keypair.public_key());
    
    // Set up the HTTPS server with TLS using the keypair
    let server = axum_server::bind_rustls(
        addr,
        RustlsConfig::from_config(Arc::new(keypair.to_rpk_rustls_server_config())),
    );
    
    // Create a simple router
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    
    // Run the server
    server.serve(app.into_make_service()).await?;
    
    Ok(())
}

async fn publish_server_pkarr(
    client: &Client,
    keypair: &Keypair,
    socket_addr: &SocketAddr,
) -> Result<(), Box<dyn Error>> {
    // Create SVCB record for HTTPS service
    let mut svcb = SVCB::new(0, ".".try_into()?);
    svcb.set_port(socket_addr.port());
    
    // Build and sign a DNS packet with HTTPS and address records
    let signed_packet = SignedPacket::builder()
        .https(".".try_into()?, svcb, 60 * 60)
        .address(".".try_into()?, socket_addr.ip(), 60 * 60)
        .sign(&keypair)?;
    
    // Publish the packet to the DHT
    client.publish(&signed_packet, None).await?;
    
    Ok(())
}
// --8<-- [end:serve_pkdns_tls]
