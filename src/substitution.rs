use crate::{config::Config, matching::Cipher};
use std::{collections::HashMap, fs::File, io::Read};

pub struct Substitution {
    map: HashMap<char, char>, // Stores the mapping of characters
}

impl Substitution {
    pub fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Unable to open substitution key file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file content");

        let map: HashMap<String, String> =
            serde_yaml::from_str(&contents).expect("Failed to parse YAML into Hashmap");

        let cipher_map: HashMap<char, char> = map
            .into_iter()
            .map(|(k, v)| {
                let key = k.chars().next().expect("Invalid char key");
                let value = v.chars().next().expect("Invalid char value");
                (key, value)
            })
            .collect();

        Substitution { map: cipher_map }
    }

    // Encrypt: substitute each char with its mapped value
    pub fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.map.get(&c).cloned().unwrap_or(c))
            .collect()
    }

    // Decrypt: reverse the map to substitute back (to original characters)
    pub fn decrypt(&self, text: &str) -> String {
        let reverse_map: HashMap<char, char> = self.map.iter().map(|(k, v)| (*v, *k)).collect();
        text.chars()
            .map(|c| reverse_map.get(&c).cloned().unwrap_or(c))
            .collect()
    }
}

impl Cipher for Substitution {
    fn encrypt(text: &str) -> String {
        let config = Config::new();
        let encrypter = Substitution::new(&config.keymaps_path);
        encrypter.encrypt(text)
    }

    fn decrypt(text: &str) -> String {
        let config = Config::new();
        let decrypter = Substitution::new(&config.keymaps_path);
        decrypter.decrypt(text)
    }
}
