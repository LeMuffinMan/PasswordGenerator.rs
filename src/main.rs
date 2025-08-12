use std::fs;

use rand::prelude::IndexedRandom;
use rand::rng; // not in std 

mod parse_config; //first include the module

use parse_config::get_value_from_line;
use parse_config::fill_charset;
use parse_config::PasswordConfig; //then use to the content

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
