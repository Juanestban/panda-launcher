//! Signs release files for the updater (ADR 0010). The key is a 32-byte seed, hex, in `PANDA_UPDATE_SIGNING_KEY`.
//!
//!   cargo run -p panda-core --example sign -- --public    print the public key to put in PANDA_UPDATE_PUBLIC_KEY
//!   cargo run -p panda-core --example sign -- FILE...     write FILE.sig: ed25519 over FILE's SHA-256 digest
//!
//! Make a key with `openssl rand -hex 32` and keep it only in the GitHub secret and a password manager.

use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let seed = std::env::var("PANDA_UPDATE_SIGNING_KEY").map_err(|_| "PANDA_UPDATE_SIGNING_KEY is not set")?;
  let seed = seed.trim();
  if seed.len() != 64 {
    return Err("PANDA_UPDATE_SIGNING_KEY must be 64 hex characters".into());
  }
  let mut bytes = [0u8; 32];
  for (i, b) in bytes.iter_mut().enumerate() {
    *b = u8::from_str_radix(&seed[i * 2..i * 2 + 2], 16)?;
  }
  let key = SigningKey::from_bytes(&bytes);

  let files: Vec<String> = std::env::args().skip(1).collect();
  if files.iter().any(|a| a == "--public") {
    println!("{}", key.verifying_key().as_bytes().iter().map(|b| format!("{b:02x}")).collect::<String>());
    return Ok(());
  }
  if files.is_empty() {
    return Err("usage: sign --public | sign FILE...".into());
  }
  for file in files {
    let digest = Sha256::digest(std::fs::read(&file)?);
    std::fs::write(format!("{file}.sig"), key.sign(&digest).to_bytes())?;
    println!("signed {file}");
  }
  Ok(())
}
