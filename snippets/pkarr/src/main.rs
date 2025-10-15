use pkarr::{Client, InMemoryCache};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};
use url::Url;

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
        // Set a custom cache size.
        .cache(Arc::new(InMemoryCache::new(
            NonZeroUsize::new(5_000).unwrap(),
        )))
        // Add custom relays.
        .relays(&[Url::parse("https://my-relay.com")?])?
        // Add custom bootstrap nodes.
        .bootstrap(&[SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8080)])
        // Set a custom request timeout.
        .request_timeout(Duration::from_secs(10))
        .build()?;

    Ok(Arc::new(client))
}
// --8<-- [end:init_pkarr_client_with_opts]

