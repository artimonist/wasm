use flate2::bufread::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use std::io::prelude::*;

pub trait Compressor {
    fn compress(&self) -> CompressResult<Vec<u8>>;
    fn decompress(&self) -> CompressResult<Vec<u8>>;
}

impl Compressor for [u8] {
    fn compress(&self) -> CompressResult<Vec<u8>> {
        let mut e = DeflateEncoder::new(self, Compression::default());
        let mut buf = Vec::new();
        e.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn decompress(&self) -> CompressResult<Vec<u8>> {
        let mut d = DeflateDecoder::new(self);
        let mut buf = Vec::new();
        d.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

type CompressResult<T = ()> = Result<T, std::io::Error>;

use artimonist::bitcoin::hex::{DisplayHex, FromHex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(content: &str, compress: bool) -> String {
    if compress {
        content
            .as_bytes()
            .compress()
            .expect("compress failed.")
            .to_lower_hex_string()
    } else {
        let data = Vec::from_hex(content).expect("invalid hex string.");
        String::from_utf8(data.decompress().expect("decompress failed."))
            .expect("invalid utf8 string.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let content = "hello world";
        let compressed = compress(content, true);
        let decompressed = compress(&compressed, false);
        assert_eq!(content, decompressed);
    }
}
