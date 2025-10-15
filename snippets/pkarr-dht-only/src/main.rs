use std::{error::Error, sync::Arc};

use pkarr::Client;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_dht_only]
fn init_client_dht_only() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder()
        .no_relays()
        // Optionally add custom bootstrap DHT nodes
        // .bootstrap(vec!["127.0.0.1:45555".to_string()])
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_dht_only]