use crate::matching::Cipher;

pub struct Caesar;

impl Caesar {
    fn shift_char(c: char, shift: i8) -> char {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base) as i8;
            let new_offset = (offset + shift).rem_euclid(26);
            (base + new_offset as u8) as char
        } else {
            c
        }
    }

    pub fn encrypt(text: &str) -> String {
        text.chars().map(|c| Self::shift_char(c, 3)).collect()
    }

    pub fn decrypt(text: &str) -> String {
        text.chars().map(|c| Self::shift_char(c, -3)).collect()
    }
}

impl Cipher for Caesar {
    fn encrypt(text: &str) -> String {
        Caesar::encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        Caesar::decrypt(text)
    }
}
