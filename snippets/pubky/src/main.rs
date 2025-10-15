mod generate_keys;
mod pkarr_client;

fn main() -> anyhow::Result<()> {
    generate_keys::main();
    pkarr_client::main()?;
    Ok(())
}