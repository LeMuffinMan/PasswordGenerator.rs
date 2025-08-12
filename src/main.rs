use std::fs;

use rand::prelude::IndexedRandom;
use rand::rng; // not in std 

mod parse_config; //first include the module

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

    //declare an instance of random number generator
    let mut _rng = rng();

    //Declaring a new empty String
    let mut password = String::with_capacity(config.length);

    //fill charset with PasswordConfig infos
    let charset = fill_charset(&config)?;

    for _ in 0..config.length {

        if let Some(&c) = charset.choose(&mut _rng) {
            password.push(c); //add at the end
        }
    }
    println!("Generated password : {password}");
    Ok(())
}
