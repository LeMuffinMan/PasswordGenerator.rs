
use std::error::Error;

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

pub struct PasswordConfig {
    pub length: u8,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digit: bool,
    pub symbol: bool,
}

impl PasswordConfig {
    pub fn describe(&self) {
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

pub fn get_value_from_line(line: &str, config: &mut PasswordConfig) -> Result<(), Box<dyn Error>> {
    
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

    println!("charset : {:?}", charset);
    Ok(charset)
}
