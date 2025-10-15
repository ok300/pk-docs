use std::{error::Error, sync::Arc, time::Duration};

use pkarr::InMemoryCache;
use pkarr::{Client, Keypair, SignedPacket};
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
