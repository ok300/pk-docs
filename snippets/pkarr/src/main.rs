use std::{error::Error, sync::Arc};

use pkarr::Client;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client]
fn init_client() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder().build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client]

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_with_opts]
fn init_client_with_opts() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder()
        .cache_size(5_000) // Set a custom cache size
        .minimum_ttl(5 * 60) // Set a custom minimum TTL, in seconds
        .maximum_ttl(24 * 60 * 60) // Set a custom maximum TTL, in seconds
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_with_opts]

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_relays_only]
fn init_client_relays_only() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder()
        .no_dht()
        // Optionally add custom relays
        // .extra_relays(&["https://a.custom.relay"])?
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_relays_only]

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_dht_only]
fn init_client_dht_only() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder()
        .no_relays()
        // Optionally add custom bootstrap DHT nodes
        // .extra_bootstrap(&["127.0.0.1:45555"])
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_dht_only]
