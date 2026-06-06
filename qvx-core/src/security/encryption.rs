use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::{Aead, OsRng};
use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};

pub struct EncryptionCore {
    cipher: Aes256Gcm,
}

impl EncryptionCore {
    pub fn new(password: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key_bytes = hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, [u8; 12])> {
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher.encrypt(nonce, data)
            .map_err(|e| anyhow!("Encryption error: {}", e))?;
        Ok((ciphertext, nonce_bytes))
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption error: {}", e))?;
        Ok(plaintext)
    }
}
