use clap::Parser;

mod generation;
use generation::generation;

mod cli;
use cli::Cli;

mod passwordconfig;
use passwordconfig::PasswordConfig;

mod charset;
use charset::fill_charset;

fn get_entropy(config: &PasswordConfig, charset: &Vec<char>) -> f64 {
    
    let charset_size: f64 = charset.len() as f64;
    let entropy: f64 = config.length as f64 * charset_size.log2();
    entropy
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //interpret and allow to use arguments given
    let cli = Cli::parse();

    //will build the config struct following the config.toml provided, or use defaults values
    //Then, override the config with the arguments provided
    let config = cli.build_config();

    if config.length == 0 {
        return Err("Error : length = 0".into());
    }

    //create the charset following the settings of struct config
    let charset = fill_charset(&config)?;

    if config.debug {
        println!("charset : {:?}", charset);
    }

    if charset.is_empty() {
        return Err("Eroor : charset is empty".into());
    }

    if config.length as usize > charset.len() {
        return Err("Error : Charset is too small to guarantee no duplicated char".into());
    }

    let password = generation(&charset, config.length, config.duplicate);
    println!("Generated password : {password}");

    let entropy = get_entropy(&config, &charset);
    if config.entropy {
        println!("\nentropy = {entropy}\n");
    }

    if config.json {
        let serialized = serde_json::to_string(&config).unwrap();
        println!("\njson: \n{}", serialized);
    }

    Ok(())
}

    //Todo : 
    //- tests unitaires ?
    //- Readme
    //
    //- audit de bruteforce 
    //- integration bitwarden
    //- passphrases ? wordlist embed (EFF large)
    
