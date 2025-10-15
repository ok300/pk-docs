use anyhow::Result;
use pubky::prelude::*;

fn generate_keys() {
    // ANCHOR: generate-keys
    let keypair = Keypair::random();
    let public_key = keypair.public_key();
    // ANCHOR_END: generate-keys

    println!("Keypair generated successfully!");
    println!("Public Key: {}", public_key);
}

fn pkarr_client() -> Result<()> {
    // ANCHOR: pkarr-client
    let client = PubkyHttpClient::builder().build()?;
    let pkarr_client = client.pkarr();
    // ANCHOR_END: pkarr-client

    println!("Successfully got pkarr client: {:?}", pkarr_client);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    generate_keys();
    pkarr_client()?;
    Ok(())
}