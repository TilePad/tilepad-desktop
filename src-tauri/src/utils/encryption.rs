use std::path::Path;

use x25519_dalek::{PublicKey, StaticSecret};

/// New-type to store the server private key
#[derive(Clone)]
pub struct ServerKeyPair {
    pub private_key: StaticSecret,
    pub public_key: PublicKey,
}

/// Sets up the server private key, loads an existing key if one is present at `path`
/// otherwise will generate the save a new one
pub async fn setup_private_key(path: &Path) -> std::io::Result<StaticSecret> {
    // Try read existing private key
    if let Some(private_key) = read_private_key(path).await? {
        return Ok(private_key);
    }

    // Generate and write a new private key
    let private_key = StaticSecret::random();
    write_private_key(path, &private_key).await?;
    Ok(private_key)
}

/// Write `private_key` to a file at `path`
async fn write_private_key(path: &Path, private_key: &StaticSecret) -> std::io::Result<()> {
    let key_bytes = private_key.as_bytes();

    // Create parent path if missing
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        tokio::fs::create_dir_all(parent).await?;
    }

    tokio::fs::write(path, key_bytes).await?;
    Ok(())
}

/// Read a private key from `path`
async fn read_private_key(path: &Path) -> std::io::Result<Option<StaticSecret>> {
    if !path.exists() {
        return Ok(None);
    }

    if !path.is_file() {
        return Err(std::io::Error::other("private key file is not a file"));
    }

    let bytes = tokio::fs::read(path).await?;

    if bytes.len() != 32 {
        return Err(std::io::Error::other(
            "unexpected key size, expected 32 bytes",
        ));
    }

    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&bytes[0..32]);

    let secret = StaticSecret::from(buffer);
    Ok(Some(secret))
}
