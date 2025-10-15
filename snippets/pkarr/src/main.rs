use pkarr::{Client, Keypair, SignedPacket};
use std::{error::Error, time::Duration};

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
    use pkarr::InMemoryCache;
    use std::{num::NonZeroUsize, sync::Arc};
    use url::Url;

    let client = Client::builder()
        // Set a custom cache size.
        .cache(Arc::new(InMemoryCache::new(
            NonZeroUsize::new(5_000).unwrap(),
        )))
        // Add custom relays.
        .relays(&[Url::parse("https://my-relay.com")?])?
        // Add custom bootstrap nodes.
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

    let public_key_str = "9fdaa3b3cb04f24328975a4862419d2a2a46639c33659a101f653457a40b9d16";

    let signed_packet = client.resolve(&public_key_str.parse()?).await;

    if let Some(signed_packet) = signed_packet {
        println!("Resolved signed packet: {:?}", signed_packet);
    } else {
        println!("No record found for the public key");
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

    // Create a signed packet.
    let signed_packet = SignedPacket::builder()
        .txt(
            "_proto".try_into().unwrap(),
            "foo=bar".try_into().unwrap(),
            30,
        )
        .build(&keypair)?;

    if client.publish(&signed_packet, None).await.is_ok() {
        println!("Published successfully!");
    } else {
        println!("Failed to publish.");
    }

    Ok(())
}
// --8<-- [end:publish_record]