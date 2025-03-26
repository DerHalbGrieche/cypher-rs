use crate::matching::Cipher;

pub struct Rot13;

impl Rot13 {
    fn transform(text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = (c as u8 - base + 13) % 26;
                    (base + offset) as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn encrypt(text: &str) -> String {
        Self::transform(text)
    }

    pub fn decrypt(text: &str) -> String {
        Self::transform(text)
    }
}

impl Cipher for Rot13 {
    fn encrypt(text: &str) -> String {
        Rot13::encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        Rot13::decrypt(text)
    }
}
