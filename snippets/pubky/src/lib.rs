use pubky::prelude::*;

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

#[allow(dead_code)]
// --8<-- [start:storage_session]
async fn storage_session() -> pubky::Result<()> {
    let pubky = Pubky::new()?;
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // Sign in to get a session
    let session = signer.signin().await?;

    // Get the session storage
    let storage = session.storage();

    // Write data
    storage
        .put("/pub/my.app/data.txt", "Hello from session storage")
        .await?;

    // Read data
    let text = storage.get("/pub/my.app/data.txt").await?.text().await?;
    println!("Retrieved: {}", text);

    Ok(())
}
// --8<-- [end:storage_session]

#[allow(dead_code)]
// --8<-- [start:storage_public]
async fn storage_public() -> pubky::Result<()> {
    let pubky = Pubky::new()?;

    // Get the public storage accessor
    let public = pubky.public_storage();

    // The public key of the user whose data you want to access
    let user_pubkey = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid user public key");

    // Read a file
    let file = public
        .get(format!("pubky://{}/pub/acme.app/file.txt", user_pubkey))
        .await?
        .bytes()
        .await?;
    println!("Retrieved file: {:?}", file);

    // List directory entries with limit
    let entries = public
        .list(format!("pubky://{}/pub/acme.app/", user_pubkey))?
        .limit(10)
        .send()
        .await?;

    for entry in entries {
        println!("Entry: {}", entry.to_pubky_url());
    }

    Ok(())
}
// --8<-- [end:storage_public]

#[allow(dead_code)]
// --8<-- [start:pkdns_generic]
fn pkdns_generic() -> pubky::Result<()> {
    let pubky = Pubky::new()?;
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // Get the PKDNS client from the signer
    let _pkdns = signer.pkdns();

    // The pkdns client can be used to publish and resolve records
    println!("PKDNS client ready");

    Ok(())
}
// --8<-- [end:pkdns_generic]

#[allow(dead_code)]
// --8<-- [start:pkdns_homeserver]
async fn pkdns_homeserver() -> pubky::Result<()> {
    let pubky = Pubky::new()?;
    let keypair = Keypair::random();
    let signer = pubky.signer(keypair);

    // Resolve another user's homeserver
    let other_user = PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
        .expect("Valid public key");
    let homeserver = pubky.get_homeserver_of(&other_user).await;
    println!("Other user's homeserver: {:?}", homeserver);

    // Publish your own homeserver record (if stale)
    signer.pkdns().publish_homeserver_if_stale(None).await?;
    println!("Published homeserver record");

    // Force republish (e.g., for homeserver migration)
    let new_homeserver =
        PublicKey::try_from("o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56uyy")
            .expect("Valid homeserver public key");
    signer
        .pkdns()
        .publish_homeserver_force(Some(&new_homeserver))
        .await?;
    println!("Force published homeserver record");

    // Fetch your own homeserver record
    let my_homeserver = signer.pkdns().get_homeserver().await?;
    println!("My homeserver: {:?}", my_homeserver);

    Ok(())
}
// --8<-- [end:pkdns_homeserver]

#[allow(dead_code)]
// --8<-- [start:qr_auth]
async fn qr_auth() -> pubky::Result<()> {
    let pubky = Pubky::new()?;

    // Build capabilities for acme.app
    let caps = Capabilities::builder()
        .read_write("/pub/acme.app/")
        .finish();

    // Start the auth flow
    let flow = pubky.start_auth_flow(&caps)?;

    // Get the authorization URL to display as QR code or deeplink
    let auth_url = flow.authorization_url();
    println!("Scan this QR code or open this URL: {}", auth_url);

    // Await approval from the signing device
    // (In real usage, this would wait for the user to approve on their device)
    let _session = flow.await_approval().await?;

    println!("Auth approved! Session ready.");

    Ok(())
}
// --8<-- [end:qr_auth]
