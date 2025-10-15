use std::{error::Error, num::NonZeroUsize, sync::Arc};

use pkarr::{Client, InMemoryCache};

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
        // Set a custom cache size
        .cache(Arc::new(InMemoryCache::new(
            NonZeroUsize::new(5_000).unwrap(),
        )))
        .build()?;
    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_with_opts]

