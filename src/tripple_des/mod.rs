mod decrypt;
mod encrypt;

use crate::matching::Cipher;
use clap::Command;

pub struct TrippleDes;

impl Cipher for TrippleDes {
    fn encrypt(text: &str) -> String {
        encrypt::encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        decrypt::decrypt(text)
    }
}

pub fn new_3des_command() -> Command {
    let tripple_des_command = Command::new("3DES").subcommands([
        encrypt::new_encrypt_command(),
        decrypt::new_decrypt_command(),
    ]);
    tripple_des_command
}
