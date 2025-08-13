
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordConfig {
    #[serde(default = "default_length")]
    pub length: u8,
    #[serde(default = "default_true")]
    pub lowercase: bool,
    #[serde(default = "default_true")]
    pub uppercase: bool,
    #[serde(default = "default_true")]
    pub digit: bool,
    #[serde(default = "default_false")]
    pub symbol: bool,
    #[serde(default = "default_false")]
    pub duplicate: bool,
    #[serde(default = "default_false")]
    pub debug: bool,
    #[serde(default = "default_false")]
    pub entropy: bool,
    #[serde(default = "default_false")]
    pub json: bool,
}

fn default_length() -> u8 { 12 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }

impl Default for PasswordConfig {
    fn default() -> Self {
        PasswordConfig {
            length: default_length(),
            lowercase: default_true(),
            uppercase: default_true(),
            digit: default_true(),
            symbol: default_false(),
            duplicate: default_false(),
            debug: default_false(),
            entropy: default_false(),
            json: default_false(),
        }
    }
}

impl PasswordConfig {
    pub fn from_file(path: &str) -> Self {

        match fs::read_to_string(path) {
            Ok(toml_content) => {
                if toml_content.trim().is_empty() {
                    println!("Empty toml file, default values will be use");
                    return Self::default();
                }
                
                match toml::from_str::<Self>(&toml_content) {
                    Ok(config) => {
                        if config.debug {
                            println!("Loading config from {}", path);
                        }
                        config
                    }
                    Err(e) => {
                        println!("Error parsing {} ({}), default values will be use", path, e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                println!("File {} not found, default values will be use", path);
                Self::default()
            }
        }
    }
    pub fn describe(&self) {
        println!("length = {}", self.length);
        println!("lowercase = {}", self.lowercase);
        println!("uppercase = {}", self.uppercase);
        println!("digit = {}", self.digit);
        println!("symbol = {}", self.symbol);
        println!("duplicate = {}", self.duplicate);
        println!("debug = {}", self.debug);
        println!("entropy = {}", self.entropy);
        println!("json = {}", self.json);
        println!("");
    }
}

