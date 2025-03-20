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

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(content: &str, compress: bool) -> String {
    if compress {
        URL_SAFE.encode(content.as_bytes().compress().expect("compress failed."))
    } else {
        let data = URL_SAFE.decode(content).expect("invalid hex string.");
        String::from_utf8(data.decompress().expect("decompress failed."))
            .expect("invalid utf8 string.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let content = "xprv9s21ZrQH143K3DsMn27o9Dw3iwDWJa6ztqdbeyVoMm1UjeK4PQYZPqxpyu5hYGm3qzzB2p1HvZHGoEK1Vwu84SvbcuygptA9kguvhXfDVYN";
        let compressed = compress(content, true);
        println!("compressed: {}", compressed);
        let decompressed = compress(&compressed, false);
        assert_eq!(content, decompressed);
    }
}
