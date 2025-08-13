use clap::Parser;
use crate::passwordconfig::PasswordConfig;

#[derive(Parser)]
#[command(name = "PasswordGenerator.rs")]
#[command(about = "A simple CLI to generate passwords")]
#[command(version, about, long_about = None)]

pub struct Cli
{

    //we need to set a default value here for the config.toml 
    #[arg(short, long, help = "Path to config file", default_value = "config.toml")]
    pub file: String,

    #[arg(short, long, help = "Length of password")]
    pub length: Option<u8>,

    #[arg(short,
        long,
        help = "\"luds\" activates l(owercase), u(ppercase), d(igit), s(ymbol)", 
        value_name = "CHARSET",
        num_args = 0..=1,
        default_missing_value = "luds" )]
    pub charset: Option<String>, 

    #[arg(long, help = "Enable debug output")]
    pub debug: bool,
    
    #[arg(long, help = "Show entropy information")]
    pub entropy: bool,
    
    #[arg(long, help = "Output in JSON format")]
    pub json: bool,
    
    #[arg(long, help = "Allow duplicate characters")]
    pub duplicate: bool,
    
}

impl Cli {
    ///Once the config struct is built, we override the settings with the ones Clap parsed
    pub fn args_override(&self, config: &mut PasswordConfig)
    {
        if let Some(length) = self.length {
            config.length = length;
        }

        if let Some(ref cs) = self.charset {
            config.lowercase = cs.contains('l');
            config.uppercase = cs.contains('u');
            config.digit = cs.contains('d');
            config.symbol = cs.contains('s');
        }

        if self.debug {
            config.debug = true;
        }
        if self.entropy {
            config.entropy = true;
        }
        if self.json {
            config.json = true;
        }
        if self.duplicate {
            config.duplicate = true;
        }
    }
    ///Main methode to build the config : tries to parse a TOML config file, or attribute default
    ///value if none are provided or if there is no valid config file available
    pub fn build_config (&self) -> PasswordConfig {

        let mut config = PasswordConfig::from_file(&self.file); 

        self.args_override(&mut config);

        if config.debug {
            config.describe();
        }
        //we could print the struct this way but describe() methode is more readable
        // if config.debug {
        //     println!("config struct built: {:?}\n", config);
        // } 
        
        config
    }
}
