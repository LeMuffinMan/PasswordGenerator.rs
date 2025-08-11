use std::fs;
use std::error::Error;
use rand::prelude::IndexedRandom;
use rand::rng; // not in std 

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

struct PasswordConfig {
    length: usize,
    lowercase: bool,
    uppercase: bool,
    digit: bool,
    symbol: bool,
}

impl PasswordConfig {
    fn describe(&self) {
        println!("length = {}", self.length);
        println!("lowercase = {}", self.lowercase);
        println!("uppercase = {}", self.uppercase);
        println!("digit = {}", self.digit);
        println!("symbol = {}", self.symbol);
    }
}

fn which_key(key: &str, value: &str, config: &mut PasswordConfig) -> Result<(), Box<dyn Error>> { 

    let key = key.trim();
    let value = value.trim();

    match key {
        "length" => {
            config.length = value.parse()?; // pas besoin de mettre parse::<u8>() ici ?
            Ok(())
        }
        "lowercase" => {
            config.lowercase = value.parse()?;
            Ok(())
        }
        "uppercase" => {
            config.uppercase = value.parse()?;
            Ok(())
        }
        "digit" => {
            config.digit = value.parse()?;
            Ok(())
        }
        "symbol" => {
            config.symbol = value.parse()?;
            Ok(())
        }
        _ => {
            return Err(format!("Error : unknown key {key}").into());
        }
    }

}

fn get_value_from_line(line: &str, config: &mut PasswordConfig) -> Result<(), Box<dyn Error>> {
    
    let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        if let Some((key, value)) = line.split_once('=') {
            
            which_key(key, value, config)?;
        } else {
            return Err(format!("Error : invalid line : {line}").into())
        }
    Ok(())
}

fn fill_charset(config: &PasswordConfig)
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

    println!("charset : {:?}", charset);
    Ok(charset)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut config = PasswordConfig {
        length: 15,
        lowercase: true,
        uppercase: true,
        digit: true,
        symbol: true,
    };

    let path = "config.txt";
    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        // println!("{line}");
        get_value_from_line(line, &mut config)?;
    }
    config.describe();

    let mut _rng = rng();

    let mut password = String::with_capacity(config.length);

    let charset = fill_charset(&config)?;

    for _ in 0..config.length {

        if let Some(&c) = charset.choose(&mut _rng) {
            password.push(c);
        }
    }
    println!("Generated password : {password}");
    Ok(())
}
