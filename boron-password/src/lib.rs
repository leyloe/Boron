use anyhow::Result;

use hkdf::Hkdf;
use sha2::Sha256;

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"#$%&@^`~";

#[derive(PartialEq)]
pub enum Options {
    Lower,
    Upper,
    Digits,
    Special,
}

pub struct BoronPassword {
    char_list: Vec<u8>,
    len: u32,
}

impl BoronPassword {
    pub fn init(options: &[Options], len: u32) -> Self {
        let mut char_list = Vec::new();

        if options.contains(&Options::Lower) {
            char_list.extend_from_slice(LOWER);
        }
        if options.contains(&Options::Upper) {
            char_list.extend_from_slice(UPPER);
        }
        if options.contains(&Options::Digits) {
            char_list.extend_from_slice(DIGITS);
        }
        if options.contains(&Options::Special) {
            char_list.extend_from_slice(SPECIAL);
        }

        Self { char_list, len }
    }

    pub fn generate_from(self, key: [u8; 32]) -> Result<Vec<u8>> {
        let modulus = self.char_list.len();
        let mut password = Vec::new();

        let hk = Hkdf::<Sha256>::new(None, &key);

        let mut okm = vec![0u8; self.len.try_into()?];

        hk.expand(b"boron", &mut okm).unwrap();

        for i in &okm {
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

        const OPTIONS: [Options; 4] = [
            Options::Lower,
            Options::Digits,
            Options::Special,
            Options::Upper,
        ];

        let boron_password = BoronPassword::init(&OPTIONS, 12);

        let password = boron_password.generate_from(KEY).unwrap();

        println!("password: {}", String::from_utf8(password).unwrap());
    }
}
