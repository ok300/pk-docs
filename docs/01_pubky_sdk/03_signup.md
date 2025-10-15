# Signup

Before you can store data on a homeserver, you need to sign up with that homeserver using your keypair. In Pubky, you first create a `PubkySigner` from your keypair, then use it to sign up.

```rust
--8<-- "snippets/pubky/src/main.rs:signup"
```

The signup process:

1. Create a `Pubky` instance
2. Generate or use an existing keypair
3. Create a `PubkySigner` from your keypair using `pubky.signer(keypair)`
4. Identify the homeserver you want to use (by its public key)
5. Call `signup` on the signer with the homeserver's public key
6. Optionally provide a signup code if the homeserver requires one

After successful signup, your public key will be associated with the homeserver and published via PKARR. The signup returns a `PubkySession` which you can use for authenticated operations.
