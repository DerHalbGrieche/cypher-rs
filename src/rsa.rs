use crate::config::Config;
use crate::matching::Cipher;
use openssl::base64::{decode_block, encode_block};
use openssl::rsa::{Padding, Rsa};
use std::fs::File;
use std::io::Read;

pub struct RsaCommand {
    public_key: Rsa<openssl::pkey::Public>,
    private_key: Rsa<openssl::pkey::Private>,
}

impl RsaCommand {
    pub fn new(config: Config) -> Self {
        let mut pub_key_file =
            File::open(config.public_key_path).expect("Couldn't open Public Key");
        let mut pub_key_pem = Vec::new();
        pub_key_file
            .read_to_end(&mut pub_key_pem)
            .expect("Couldnt' read public Key");

        let public_key = Rsa::public_key_from_pem(&pub_key_pem).expect("Couldn't read public Key");

        let mut priv_key_file =
            File::open(&config.private_key_path).expect("Couldn't read private Key");
        let mut priv_key_pem = Vec::new();
        priv_key_file
            .read_to_end(&mut priv_key_pem)
            .expect("Couldn't read private Key");

        let private_key =
            Rsa::private_key_from_pem(&priv_key_pem).expect("Couldn't read private Key");

        RsaCommand {
            public_key: public_key,
            private_key: private_key,
        }
    }
    pub fn encrypt(&self, text: &str) -> String {
        let padding = Padding::PKCS1;
        let mut encrypted = vec![0; self.public_key.size() as usize];

        let len = self
            .public_key
            .public_encrypt(text.as_bytes(), &mut encrypted, padding)
            .expect("Failed to encrypt");
        encrypted.truncate(len);
        encode_block(&encrypted)
    }
    pub fn decrpt(&self, text: &str) -> String {
        let padding = Padding::PKCS1;
        let mut decrypted = vec![0; self.private_key.size() as usize];

        let len = self
            .private_key
            .private_decrypt(
                &decode_block(&text).expect("Failed to decode base64"),
                &mut decrypted,
                padding,
            )
            .expect("Failed to decrypt");

        decrypted.truncate(len);

        String::from_utf8(decrypted).expect("Invalid text")
    }
}

impl Cipher for RsaCommand {
    fn encrypt(text: &str) -> String {
        let config = Config::new();
        let rsa = RsaCommand::new(config);
        rsa.encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        let config = Config::new();
        let rsa = RsaCommand::new(config);
        rsa.decrpt(text)
    }
}
