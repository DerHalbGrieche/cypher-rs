use clap::{Arg, Command};
use openssl::{
    base64::decode_block,
    symm::{Cipher, Crypter, Mode},
};

use crate::config::Config;

pub fn new_decrypt_command() -> Command {
    let command = Command::new("decrypt")
        .arg(Arg::new("text"))
        .arg_required_else_help(true);
    command
}

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
