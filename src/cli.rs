use clap::Parser;
use crate::parse_config::PasswordConfig;

#[derive(Parser)]
#[command(name = "PasswordGenerator.rs")]
#[command(about = "A simple CLI to generate passwords")]
#[command(version, about, long_about = None)]

pub struct Cli
{
    #[arg(short, long, help = "lenght : -l 7 or --length 7")]
    pub length: Option<u8>,

    #[arg(short,
        long,
        help = "--charset luds activates l(owercase), u(ppercase), d(igit), s(ymbol)", 
        value_name = "CHARSET",
        num_args = 0..=1,
        default_missing_value = "luds" )]
    pub charset: Option<String>, 
}

impl Cli {
    pub fn get_args_override(&self, config: &mut PasswordConfig) {
        
        if let Some(length) = self.length {
            config.length = length;
        }

        if let Some(ref cs) = self.charset {
            config.lowercase = cs.contains('l');
            config.uppercase = cs.contains('u');
            config.digit = cs.contains('d');
            config.symbol = cs.contains('s');
            //flags entropy debug et json
        }
    }
}
    // pub fn to_config_or_default(&self) -> PasswordConfig {
    //
    //     let length = self.length.unwrap_or(15);

        //declarer un tuple de 4 elements pour les 4 bool du charset
        //il faut utilise ref dans le some, pour borrow charset sans le modifier, on lit simplement
        //si on a eu --charset sans rien on entre dnas la bracket : on cherche chaque char, cs.contains
        //renvoie un bool, et est attribue en fonction de la position des charset dans le tuple que
        //j'ai declare, d'ou le premier '=' !!   ICI, d'ou aussi la parenthese autour des cs.contains
        // let (lowercase, uppercase, digit, symbol) = if let Some(ref cs) = self.charset {
        //     (
            //ici cs, est une variable cree pour operer sur une copie de charset, avec la methode contains
                //pourquoi le shadowing n'est pas possible et je dois passer par cs ?
            //     cs.contains('l'),
            //     cs.contains('u'),
            //     cs.contains('d'),
            //     cs.contains('s'),
            // )
        //si charset est None (pas renseigne en arg), on passe dans le else et on donne des valeurs par
        //defaut
        // } else {
        //     (true, true, true, true)
        // };
        //Maintenant qu'on a la length, et un tupe de 4 elements pour le charset, on creer une
        //nouvelle instance de PasswordConfig et on l'initalise avec length, et les elements de
        //notre tuple
        // let mut config = PasswordConfig {
        //     length: length,
        //     lowercase: lowercase,
        //     uppercase: uppercase,
        //     digit: digit,
        //     symbol: symbol,
        // };
        // return config
        //pas de ; et derniere instruction de la fonction -> c'est son return
    // }
// }
