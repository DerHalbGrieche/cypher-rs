use openssl::base64;

use crate::matching::Cipher;

pub struct Base64;

impl Base64 {
    pub fn encrypt(text: &str) -> String {
        String::from(base64::encode_block(text.as_bytes()))
    }
    pub fn decrypt(text: &str) -> String {
        String::from_utf8(base64::decode_block(text).expect("")).expect("")
    }
}

impl Cipher for Base64 {
    fn encrypt(text: &str) -> String {
        Base64::encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        Base64::decrypt(text)
    }
}
