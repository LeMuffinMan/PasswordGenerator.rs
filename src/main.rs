use clap::Parser;

mod generation;
use generation::generation;

mod cli;
use cli::Cli;

mod passwordconfig;
use passwordconfig::PasswordConfig;

mod charset;
use charset::fill_charset;

// - Entropy = length * log2(|charset|) assumes independent draws with replacement.
// - If duplicates are disallowed, the correct model is sampling without replacement: log2(P(n, k)) where P(n, k) = n!/(n-k)!
// - Fix: Compute both cases accurately and display the one matching the active mode.
//                                                  &[char] ?
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
        println!("charset : {charset:?}");
    }

    if charset.is_empty() {
        return Err("Error : charset is empty".into());
    }

    if !config.duplicate && config.length as usize > charset.len() {
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
        println!("{serialized}");
    }

    Ok(())
}

//Todo :
//- tests unitaires ?
// - charset building from config (all flag combinations)
// - generation with and without duplicates (length, uniqueness invariants)
// - entropy accuracy with and without replacement
// - CI pour la doc 
//
//- audit de bruteforce
//- integration bitwarden
//- passphrases ? wordlist embed (EFF large)

// - When --json is specified, consider suppressing all other stdout lines (or add --quiet for that purpose).
// - Add --min-lower/--min-upper/--min-digit/--min-symbol constraints if you want policy compliance generation.
