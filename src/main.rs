use std::fs;
use clap::Parser;

mod parse_config;
mod generation;
mod cli;

use generation::generation;

use cli::Cli;

use parse_config::get_value_from_line;
use parse_config::fill_charset;
use parse_config::PasswordConfig; //then use to the content

fn from_file(config: &mut PasswordConfig) {

    let path = "config.txt";
    let content = fs::read_to_string(path).unwrap_or_default(); //get all the file in one string "content" with \n

    for line in content.lines() {
        // println!("{line}");
        get_value_from_line(line, config).unwrap_or_default();
    }
}

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

    //Les valeurs renseignees dans config.txt vont ecraser les valuers par defaut
    from_file(&mut config);

    let cli = Cli::parse();

    cli.get_args_override(&mut config);

    // cli.to_config_or_default(&mut config);
    // puisque Cli.length et Cli.charset sont definis avec Option, 
    // ils seront a None si je ne les ai pas renseignes 

    // config.describe();



    config.describe();

    //fill charset with PasswordConfig infos
    let charset = fill_charset(&config)?;

    if charset.is_empty() {
        return Err("Not enough char in charset".into());
    }

    generation(&charset, config.length);
    Ok(())
}
