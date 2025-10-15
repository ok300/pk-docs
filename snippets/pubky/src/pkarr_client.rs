use anyhow::Result;
use pubky::prelude::*;

pub fn main() -> Result<()> {
    let client = PubkyHttpClient::builder().build()?;
    let pkarr_client = client.pkarr();

    println!("Successfully got pkarr client: {:?}", pkarr_client);

    Ok(())
}