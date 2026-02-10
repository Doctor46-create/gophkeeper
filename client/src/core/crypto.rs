use aes_gcm::aead::OsRng;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::Aead};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub const NONCE_SIZE: usize = 12;

pub fn encrypt_string(data: &str, password: &str) -> Result<String> {
  let mut hasher = Sha256::new();
  hasher.update(password.as_bytes());
  let key = hasher.finalize();

  let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));

  let mut nonce_bytes = [0u8; NONCE_SIZE];
  OsRng.fill_bytes(&mut nonce_bytes);

  let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

  let ciphertext = cipher
    .encrypt(nonce, data.as_bytes())
    .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

  let mut combined = nonce_bytes.to_vec();
  combined.extend_from_slice(&ciphertext);

  Ok(STANDARD.encode(combined))
}

pub fn decrypt_string(encrypted_data: &str, password: &str) -> Result<String> {
  let combined = STANDARD
    .decode(encrypted_data)
    .map_err(|e| anyhow::anyhow!("Invalid base64: {}", e))?;

  if combined.len() < NONCE_SIZE {
    return Err(anyhow::anyhow!("Ciphertext too short"));
  }

  let (nonce_bytes, ciphertext) = combined.split_at(NONCE_SIZE);
  let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);

  let mut hasher = Sha256::new();
  hasher.update(password.as_bytes());
  let key = hasher.finalize();

  let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
  let plaintext = cipher
    .decrypt(nonce, ciphertext)
    .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

  String::from_utf8(plaintext).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
}

pub fn generate_id(data: &str) -> String {
  let mut hasher = Sha256::new();
  hasher.update(data.as_bytes());
  let result = hasher.finalize();
  let mut hex = String::new();
  for byte in result.iter().take(16) {
    write!(&mut hex, "{:02x}", byte).unwrap();
  }
  hex
}
