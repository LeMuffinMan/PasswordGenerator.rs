use std::fs;

// use rand::prelude::IndexedRandom;

mod parse_config; //first include the module
mod generation;

use generation::generation;

use parse_config::get_value_from_line;
use parse_config::fill_charset;
use parse_config::PasswordConfig; //then use to the content

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //PasswordConfig mutable :
    //- default values 
    //- modified if a config.txt is found
    //- modified is arguments are found
    let mut config = PasswordConfig {
        length: 15,
        lowercase: true,
        uppercase: true,
        digit: true,
        symbol: true,
    };

    let path = "config.txt";
    let content = fs::read_to_string(path)?; //get all the file in one string "content" with \n

    for line in content.lines() {
        // println!("{line}");
        get_value_from_line(line, &mut config)?;
    }
    config.describe();

    //fill charset with PasswordConfig infos
    let charset = fill_charset(&config)?;

    if charset.is_empty() {
        return Err("Not enough char in charset".into());
    }

    generation(&charset, config.length);

    Ok(())
}
