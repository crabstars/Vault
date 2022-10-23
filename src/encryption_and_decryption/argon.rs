use anyhow::anyhow;
use chacha20poly1305::{
    aead::{stream, KeyInit},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use std::str;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    vec,
};
use zeroize::Zeroize;

macro_rules! empty_all {
    ($($item:expr), *) => {
        $(
            $item.zeroize();
        )*
    }
}
// Orientation: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
fn argon2_config<'a>() -> argon2::Config<'a> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    }
}

pub fn encrypt_text(
    text: &str,
    dist_file_path: &PathBuf,
    password: &str,
) -> Result<(), anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config())?;

    // [..32] skips the salt
    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());
    let mut dist_file = File::create(dist_file_path)?;

    dist_file.write_all(&salt)?;
    dist_file.write_all(&nonce)?;

    let ciphertext = stream_encryptor
        .encrypt_next(text.as_bytes())
        .map_err(|err| anyhow!("Encrypting file: {}", err))?;
    dist_file.write_all(&ciphertext)?;

    empty_all!(nonce, key, salt);

    Ok(())
}

pub fn decrypt_text(
    encrypted_file_path: &PathBuf,
    password: &str,
) -> Result<String, anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut file_content = String::new();

    let mut read_count = encrypted_file.read(&mut salt)?;
    if read_count != salt.len() {
        return Err(anyhow!("Error reading salt."));
    }

    read_count = encrypted_file.read(&mut nonce)?;
    if read_count != nonce.len() {
        return Err(anyhow!("Error reading nonce."));
    }

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config())?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    let mut buf: Vec<u8> = vec![];
    encrypted_file.read_to_end(&mut buf)?;
    let text = stream_decryptor
        .decrypt_next(buf.as_slice())
        .map_err(|err| anyhow!("Decrypting file: {}", err))?;
    file_content.push_str(str::from_utf8(&text)?);

    empty_all!(nonce, key, salt);
    Ok(file_content)
}
