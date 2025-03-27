use crate::config::Config;
use crate::matching::Cipher as CipherTrait;
use openssl::{
    base64::decode_block,
    base64::encode_block,
    symm::{Cipher, Crypter, Mode},
};

pub struct TrippleDes;

pub fn decrypt(text: &str) -> String {
    let config = Config::new();
    let cipher = Cipher::des_ede3_cbc();
    let ciphertext = decode_block(text).expect("");
    let mut crypter = Crypter::new(
        cipher,
        Mode::Decrypt,
        config.tdes_key.as_bytes(),
        Some(config.iv.as_bytes()),
    )
    .expect("");
    crypter.pad(true);

    let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
    let mut count = crypter.update(&ciphertext, &mut plaintext).expect("");
    count += crypter.finalize(&mut plaintext[count..]).expect("");
    plaintext.truncate(count);
    String::from_utf8(plaintext).expect("")
}

pub fn encrypt(text: &str) -> String {
    let config = Config::new();
    let cipher = Cipher::des_ede3_cbc();
    let mut crypter = Crypter::new(
        cipher,
        Mode::Encrypt,
        config.tdes_key.as_bytes(),
        Some(config.iv.as_bytes()),
    )
    .expect("");
    crypter.pad(true);

    let mut ciphertext = vec![0; text.len() + cipher.block_size()];
    let mut count = crypter.update(text.as_bytes(), &mut ciphertext).expect("");
    count += crypter.finalize(&mut ciphertext[count..]).expect("");
    ciphertext.truncate(count);

    String::from(encode_block(&ciphertext))
}

impl CipherTrait for TrippleDes {
    fn encrypt(text: &str) -> String {
        encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        decrypt(text)
    }
}
