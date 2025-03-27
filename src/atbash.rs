use crate::matching::Cipher;

pub struct Atbash;

impl Atbash {
    pub fn transform(text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = c as u8 - base;
                    (base + (25 - offset)) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Cipher for Atbash {
    fn encrypt(text: &str) -> String {
        Atbash::transform(text)
    }
    fn decrypt(text: &str) -> String {
        Atbash::transform(text)
    }
}
