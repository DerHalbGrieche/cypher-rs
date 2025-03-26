use serde_derive::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub tdes_key: String,
    pub iv: String,
    pub public_key_path: String,
    pub private_key_path: String,
    pub keymaps_path: String,
}

impl Config {
    // pub fn new() -> Self {
    //     Config {
    //         tdes_key: String::from("your-3des-key-here000000"),
    //         iv: String::from("cypheriv"),
    //     }
    // }
    pub fn new() -> Self {
        let file = read_to_string("config/config.yaml").expect("No config.yaml");
        let data: Config = serde_yaml::from_str(&file).expect("Failed to load config.yaml");

        data
    }
}
