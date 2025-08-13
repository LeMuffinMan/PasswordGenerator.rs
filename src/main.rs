use clap::Parser;

mod generation;
use generation::generation;

mod cli;
use cli::Cli;

mod parse_config;
use parse_config::PasswordConfig;

mod charset;
use charset::fill_charset;

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
        return Err("Error : Charset is too small to guarantee no duplicated char".into());
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
    //- flag json pour la sortie
    //- tests unitaires ?
    //
    //- audit de bruteforce 
    //- integration bitwarden
    //- passphrases ? wordlist embed (EFF large)
    
