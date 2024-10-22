use anyhow::Result;

use hkdf::Hkdf;
use sha2::Sha256;

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"#$%&@^`~";

pub struct Config {
    len: u32,
    lower: bool,
    upper: bool,
    digits: bool,
    special: bool,
}

pub struct Password {
    char_list: Vec<u8>,
    len: u32,
}

impl Password {
    pub fn init(config: Config) -> Self {
        let mut char_list = Vec::new();

        if config.lower {
            char_list.extend_from_slice(LOWER);
        }
        if config.upper {
            char_list.extend_from_slice(UPPER);
        }
        if config.digits {
            char_list.extend_from_slice(DIGITS);
        }
        if config.special {
            char_list.extend_from_slice(SPECIAL);
        }

        Self {
            char_list,
            len: config.len,
        }
    }

    pub fn generate_from(self, key: [u8; 32]) -> Result<Vec<u8>> {
        let modulus = self.char_list.len();
        let mut password = Vec::new();

        let hk = Hkdf::<Sha256>::new(None, &key);

        let mut buf = vec![0u8; self.len.try_into()?];

        hk.expand(b"boron", &mut buf).unwrap();

        for i in &buf {
            let char_index = *i as usize % modulus;
            password.push(self.char_list[char_index]);
        }

        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const KEY: [u8; 32] = [
            69, 195, 208, 45, 141, 188, 69, 115, 231, 103, 78, 14, 179, 219, 226, 24, 113, 37, 46,
            149, 4, 226, 32, 7, 188, 125, 31, 215, 212, 100, 155, 207,
        ];

        const CONFIG: Config = Config {
            len: 12,
            lower: true,
            upper: true,
            digits: true,
            special: true,
        };

        let boron_password = Password::init(CONFIG);

        let password = boron_password.generate_from(KEY).unwrap();

        println!("password: {}", String::from_utf8(password).unwrap());
    }
}
