// use std::fs;
use clap::Parser;
// use serde::{Deserialize, Serialize};

mod parse_config;
mod generation;
mod cli;

use generation::generation;

use cli::Cli;

use parse_config::PasswordConfig; //then use to the content

const LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z',
];

const UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

const DIGIT: [char; 10] = ['0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9'];

const SYMBOL: [char; 10] = ['!', '@', '#', '$', '%',
    '^', '&', '*', '(', ')'];


pub fn fill_charset(config: &PasswordConfig)
    -> Result<Vec<char>, Box<dyn std::error::Error>>
{
    let mut charset: Vec<char> = Vec::new();
    if config.lowercase {
        charset.extend_from_slice(&LOWERCASE);
    }
    if config.uppercase {
        charset.extend_from_slice(&UPPERCASE);
    }
    if config.digit {
        charset.extend_from_slice(&DIGIT);
    }
    if config.symbol {
        charset.extend_from_slice(&SYMBOL);
    }

    if charset.is_empty() {
        return Err("Charset empty".into());
    }

    Ok(charset)
}

fn get_entropy(config: &PasswordConfig, charset: &Vec<char>) -> f64 {
    
    let charset_size: f64 = charset.len() as f64;
    let entropy: f64 = config.length as f64 * charset_size.log2();
    entropy
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    let config = cli.build_config();

    if config.length == 0 {
        return Err("Error : length = 0".into());
    }

    //fill charset with PasswordConfig infos
    let charset = fill_charset(&config)?;

    if config.debug {
        println!("charset : {:?}", charset);
    }

    if charset.is_empty() {
        return Err("Eroor : charset is empty".into());
    }

    if config.length as usize > charset.len() {
        return Err("Warning : Charset is too small to guarantee no duplicated char".into());
    }

    let password = generation(&charset, config.length, config.duplicate);
    println!("\nGenerated password : {password}");

    let entropy = get_entropy(&config, &charset);
    if config.entropy {
        println!("\nentropy = {entropy}");
    }

    Ok(())
}

    //Todo : 
    //- parsing TOML : serde
    //- flag json pour la sortie
    //- cli : debug entropy json
    //- tests unitaires ?
    //
    //- audit de bruteforce 
    //- integration bitwarden
    //- passphrases ? wordlist embed (EFF large)
    
