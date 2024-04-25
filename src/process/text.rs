use std::{fs, io::Read, path::Path};

use crate::{get_reader, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub trait TextSign {
    /// Sign the data from the reader and return the signature
    // &[u8] implements Read
    // 动态分派
    // 注意 io 的效率才是最核心的
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    // 静态分派
    /// Verify the data from the reader and the signature
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    // 返回的 Self 必须是一个有固定长度的这样的数据结构
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", signed);

    Ok(())
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> Result<()> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;

    let valid = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };
    println!("{}", valid);

    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    // 使用的时候需要显是的加一个 mut
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // 注意生命周期
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        // hash 直到函数结束才会被 free 掉
        Ok(hash == sig)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let sig = Signature::from_bytes(sig);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Self::new(key);
        Ok(verifier)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blade3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello";
        let sig = blade3.sign(&mut &data[..]).unwrap();
        assert!(blade3.verify(&mut &data[..], &sig).unwrap());

        Ok(())
    }
}
