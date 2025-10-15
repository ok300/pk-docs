use pubky::prelude::*;

pub fn main() {
    let keypair = Keypair::random();
    let public_key = keypair.public_key();

    println!("Keypair generated successfully!");
    println!("Public Key: {}", public_key);
}