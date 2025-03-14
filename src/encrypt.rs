use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use std::sync::LazyLock;

static KEY: LazyLock<Key<Aes256Gcm>> = LazyLock::new(|| Aes256Gcm::generate_key(OsRng));

pub trait Encryptor {
    fn encrypt(&self) -> EncryptResult<Vec<u8>>;
    fn decrypt(&self) -> EncryptResult<Vec<u8>>;
}

impl Encryptor for [u8] {
    fn encrypt(&self) -> EncryptResult<Vec<u8>> {
        let cipher = Aes256Gcm::new(&KEY);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
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
