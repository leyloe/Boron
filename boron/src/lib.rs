use argon2::Argon2;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use anyhow::Result;

type HmacSha256 = Hmac<Sha256>;

pub struct Boron2 {
    pub master_key: [u8; 32],
}

impl Boron2 {
    pub fn init_master(password: &[u8], salt: Option<&[u8; 32]>) -> Result<Self> {
        let salt = salt.unwrap_or(&[0u8; 32]);

        let params = argon2::Params::new(128 * 1024, 4, 4, Some(32)).unwrap();

        let argon = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        let mut hash = [0u8; 32];
        argon.hash_password_into(password, salt, &mut hash).unwrap();

        Ok(Self { master_key: hash })
    }

    pub fn get_entry(key: [u8; 32], label: &[u8], username: Option<&[u8]>) -> Result<[u8; 32]> {
        let username = username.unwrap_or(b"");

        let mut mac = HmacSha256::new_from_slice(&key)?;

        mac.update(label);
        mac.update(username);

        let result = mac.finalize();

        Ok(result.into_bytes().into())
    }

    pub fn derive_key(root_key: [u8; 32], path: &[u32]) -> Result<[u8; 32]> {
        let mut key = root_key;

        for i in path {
            key = Self::derive_child(key, *i)?;
        }

        Ok(key)
    }

    pub fn derive_child(parent_key: [u8; 32], idx: u32) -> Result<[u8; 32]> {
        let mut mac = HmacSha256::new_from_slice(&parent_key)?;

        mac.update(&idx.to_be_bytes());

        let result = mac.finalize();

        Ok(result.into_bytes().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        const PATH: [u32; 4] = [0, 0, 0, 14];

        let boron = Boron2::init_master("password".as_bytes(), None)?;

        let key = Boron2::derive_key(boron.master_key, &PATH)?;

        let entry = Boron2::get_entry(key, b"xmpp", None)?;

        println!("{:?}", entry);

        Ok(())
    }
}
