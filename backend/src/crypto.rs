use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine as _, engine::general_purpose};

#[derive(Clone)]
pub struct CryptoConfig {
    key: Vec<u8>,
}

impl CryptoConfig {
    pub fn new() -> Self {
        let key = std::env::var("ENCRYPTION_KEY").unwrap_or_else(|_| {
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()
        });

        let key_bytes = hex::decode(&key).unwrap_or_else(|_| key.as_bytes().to_vec());

        if key_bytes.len() != 32 {
            panic!("ENCRYPTION_KEY must be 32 bytes (64 hex characters)");
        }

        Self { key: key_bytes }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| e.to_string())?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| e.to_string())?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| e.to_string())?;

        let decoded = general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| e.to_string())?;

        if decoded.len() < 12 {
            return Err("invalid encrypted data".to_string());
        }

        let (nonce_bytes, ciphertext) = decoded.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| e.to_string())?;

        String::from_utf8(plaintext).map_err(|e| e.to_string())
    }
}
