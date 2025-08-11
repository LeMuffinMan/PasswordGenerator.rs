use std::fs;
use std::error::Error;

struct PasswordConfig {
    length: u8,
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

fn get_value_from_line(line: &str, config: &mut PasswordConfig) -> Result<(), Box<dyn Error>> {
    
    let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        // if let Some((key, value)) est une forme de déstructuration pattern matching.
        // "Si mon Option (retournee par split_once) est Some((x, y)), mets x dans key et y dans value et exécute le bloc".

        if let Some((key, value)) = line.split_once('=') {

            let key = key.trim();
            let value = value.trim();

            match key {
                "length" => {
                    config.length = value.parse()?
                }
                "lowercase" => {
                    config.lowercase = value.parse()?
                }
                "uppercase" => {
                    config.uppercase = value.parse()?
                }
                "digit" => {
                    config.digit = value.parse()?
                }
                "symbol" => {
                    config.symbol = value.parse()?
                }
                _ => {
                    return Err(format!("Error : unknown key {key}").into());
                }
            }
        } else {
            return Err(format!("Error : invalid line : {line}").into())
        }
    Ok(())
}

//Synthesis ex0..=3 : 
//write a program that read in a config file to fill a struct containing the configuration to
//generate a passwrod 
//errors must be handled
//no warning at compile, no panic allowed

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
    Ok(())
}
