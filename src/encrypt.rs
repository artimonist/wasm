use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key,
};
use rand::Rng;
use std::sync::LazyLock;

static KEY: LazyLock<Key<Aes256Gcm>> = LazyLock::new(|| rand::rng().random::<[u8; 32]>().into());

pub trait Encryptor {
    fn encrypt(&self) -> EncryptResult<Vec<u8>>;
    fn decrypt(&self) -> EncryptResult<Vec<u8>>;
}

impl Encryptor for [u8] {
    fn encrypt(&self) -> EncryptResult<Vec<u8>> {
        let cipher = Aes256Gcm::new(&KEY);
        let nonce = rand::rng().random::<[u8; 12]>().into();
        let data = cipher.encrypt(&nonce, self)?;
        Ok([nonce.to_vec(), data].concat())
    }

    fn decrypt(&self) -> EncryptResult<Vec<u8>> {
        let cipher = Aes256Gcm::new(&KEY);
        let data = cipher.decrypt(self[..12].into(), &self[12..])?;
        Ok(data)
    }
}

type EncryptResult<T = ()> = Result<T, aes_gcm::Error>;

use artimonist::bitcoin::hex::{DisplayHex, FromHex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(content: &str, encrypt: bool) -> String {
    if encrypt {
        content
            .as_bytes()
            .encrypt()
            .expect("encrypt failed.")
            .to_lower_hex_string()
    } else {
        let data = Vec::from_hex(content).expect("invalid hex string.");
        String::from_utf8(data.decrypt().expect("decrypt failed.")).expect("invalid utf8 string.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let content = "hello world";
        let encrypted = encrypt(content, true);
        let decrypted = encrypt(&encrypted, false);
        assert_eq!(content, decrypted);
    }
}
