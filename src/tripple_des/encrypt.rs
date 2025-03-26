use clap::{Arg, Command};
use openssl::{
    base64,
    symm::{Cipher, Crypter, Mode},
};

use crate::config::Config;

pub fn new_encrypt_command() -> Command {
    let command = Command::new("encrypt")
        .arg(Arg::new("text"))
        .arg_required_else_help(true);
    command
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

    String::from(base64::encode_block(&ciphertext))
}
