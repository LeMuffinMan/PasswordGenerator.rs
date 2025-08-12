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

    let path = "config.toml";
    //real toml handling to do
    let content = fs::read_to_string(path).unwrap_or_default(); //get all the file in one string "content" with \n

    for line in content.lines() {
        get_value_from_line(line, config).unwrap_or_default();
    }
}

fn get_entropy(config: &mut PasswordConfig, charset: &Vec<char>) -> f64 {
    
    let charset_size: f64 = charset.len() as f64;
    let entropy: f64 = config.length as f64 * charset_size.log2();
    entropy
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Todo : 
    //- parsing TOML : serde
    //- flag json pour la sortie
    //- cli : debug entropy json
    //- tests unitaires ?
    //
    //- audit de bruteforce 
    //- integration bitwarden
    //- passphrases ? wordlist embed (EFF large)
    
    //
    //PasswordConfig mutable for override:
    //- default strong values 
    //- modified if a config.txt is found
    //- modified if arguments are provided
    let mut config = PasswordConfig {
        length: 15,
        lowercase: true,
        uppercase: true,
        digit: true,
        symbol: true,
        duplicate: false,
        debug: true,
        entropy: true,
        json: true,
    };

    //values found in toml config file will override the defaults value
    from_file(&mut config);

    let cli = Cli::parse();

    //Values we got through parse() will override config.toml ones 
    cli.get_args_override(&mut config);
    
    if config.length == 0 {
        return Err("Error : length = 0".into());
    }

    if config.debug {
        config.describe();
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

    let entropy = get_entropy(&mut config, &charset);
    if config.entropy {
        println!("\nentropy = {entropy}");
    }

    Ok(())
}
