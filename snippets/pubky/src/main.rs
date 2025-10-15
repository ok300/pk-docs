use pubky::prelude::*;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_pubky_client]
fn init_client() -> pubky::Result<Pubky> {
    let pubky = Pubky::new()?;
    Ok(pubky)
}
// --8<-- [end:init_pubky_client]

#[allow(dead_code)]
// --8<-- [start:init_pubky_client_testnet]
fn init_client_testnet() -> pubky::Result<Pubky> {
    // Create a client configured for testnet mode
    let pubky = Pubky::testnet()?;
    Ok(pubky)
}
// --8<-- [end:init_pubky_client_testnet]

#[allow(dead_code)]
// --8<-- [start:signup]
async fn signup() -> pubky::Result<()> {
    let pubky = Pubky::new()?;

    // Generate a new keypair for the user
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // The homeserver's public key (as a string)
    let homeserver = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid homeserver public key");

    // Sign up to the homeserver
    let _session = signer.signup(&homeserver, None).await?;

    println!("Successfully signed up to homeserver");

    Ok(())
}
// --8<-- [end:signup]

#[allow(dead_code)]
// --8<-- [start:put]
async fn put() -> pubky::Result<()> {
    let pubky = Pubky::new()?;
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // The homeserver's public key
    let homeserver = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid homeserver public key");
    
    // Sign up to the homeserver
    let session = signer.signup(&homeserver, None).await?;

    // The content to store
    let content = "Hello, Pubky!";

    // PUT the content to the homeserver
    session.storage().put("/pub/example.txt", content).await?;

    println!("Successfully stored data");

    Ok(())
}
// --8<-- [end:put]

#[allow(dead_code)]
// --8<-- [start:get]
async fn get() -> pubky::Result<()> {
    let pubky = Pubky::new()?;

    // The public key of the user whose data you want to retrieve
    let user_pubkey = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid user public key");

    // The path to the data you want to retrieve
    let url = format!("pubky://{}/pub/example.txt", user_pubkey);

    // GET the content from the homeserver
    let response = pubky.public_storage().get(url).await?;
    let content = response.bytes().await?;

    println!("Retrieved content: {:?}", content);

    Ok(())
}
// --8<-- [end:get]

#[allow(dead_code)]
// --8<-- [start:delete]
async fn delete() -> pubky::Result<()> {
    let pubky = Pubky::new()?;
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // The homeserver's public key
    let homeserver = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid homeserver public key");
    
    // Sign up to the homeserver
    let session = signer.signup(&homeserver, None).await?;

    // DELETE the content from the homeserver
    session.storage().delete("/pub/example.txt").await?;

    println!("Successfully deleted data");

    Ok(())
}
// --8<-- [end:delete]

#[allow(dead_code)]
// --8<-- [start:list]
async fn list() -> pubky::Result<()> {
    let pubky = Pubky::new()?;

    // The public key of the user whose data you want to list
    let user_pubkey = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid user public key");

    // List contents of a path
    let url = format!("pubky://{}/pub/", user_pubkey);
    
    let response = pubky.public_storage().get(url).await?;
    let listing = response.text().await?;

    println!("Directory listing: {}", listing);

    Ok(())
}
// --8<-- [end:list]
