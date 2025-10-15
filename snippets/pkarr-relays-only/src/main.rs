use std::{error::Error, sync::Arc};

use pkarr::Client;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_pkarr_client_relays_only]
fn init_client_relays_only() -> Result<Arc<Client>, Box<dyn Error>> {
    let client = Client::builder()
        // Optionally add custom relays
        // .relays(&[Url::parse("https://a.custom.relay")?])?
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_relays_only]