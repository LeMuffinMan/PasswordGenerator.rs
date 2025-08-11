use std::fs;
use std::error::Error;

struct PasswordConfig {
    length: u8,
    lowercase: bool,
    uppercase: bool,
    digit: bool,
    symbol: bool,
}

impl PasswordConfig {
    fn describe(&self) {
        println!("length = {}", self.length);
        println!("lowercase = {}", self.lowercase);
        println!("uppercase = {}", self.uppercase);
        println!("digit = {}", self.digit);
        println!("symbol = {}", self.symbol);
    }
}

fn get_value_from_line(line: &str, config: &mut PasswordConfig) -> Result<(), Box<dyn Error>> {
    
    let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        // if let Some((key, value)) est une forme de déstructuration pattern matching.
        // "Si mon Option (retournee par split_once) est Some((x, y)), mets x dans key et y dans value et exécute le bloc".

        if let Some((key, value)) = line.split_once('=') {

            let key = key.trim();
            let value = value.trim();

            match key {
                "length" => {
                    config.length = value.parse()? // pas besoin de mettre parse::<u8>() ici ?
                }
                "lowercase" => {
                    config.lowercase = value.parse()?
                }
                "uppercase" => {
                    config.uppercase = value.parse()?
                }
                "digit" => {
                    config.digit = value.parse()?
                }
                "symbol" => {
                    config.symbol = value.parse()?
                }
                _ => {
                    return Err(format!("Error : unknown key {key}").into());
                }
            }
        } else {
            return Err(format!("Error : invalid line : {line}").into())
        }
    Ok(())
}

//Synthesis ex0..=3 : 
//write a program that read in a config file to fill a struct containing the configuration to
//generate a passwrod 
//errors must be handled
//no warning at compile, no panic allowed

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
    Ok(())
}


//Doc :
//
//- Box<dyn std::error::Error> : 
//
// if let Some((key, value)) est une forme de déstructuration pattern matching.
// "Si mon Option (retournee par split_once) est Some((x, y)), mets x dans key et y dans value et exécute le bloc".
//
// c'est un type d'erreur dynamique "empaquete" dans un Box
//une sorte de pointeur vers une zone mémoire sur la heap. 
//La fonction peut retourner n'importe quel type d'erreur qui implémente le trait std::error::Error,
//sans connaître à la compilation le type exact de cette erreur.
//
//Utile pour gerer des erreurs de differents types en meme temps


//- .into() : 
//une methode qui permet de convertir un type en un autre. Ici utilise pour renvoyer l'erreur et le
//combiner avec Box<dyn std::error::Error>


//-  Some() : 
// une variante de l'énumération Option<T> : Some(value) / None 
// Utile pour savoir si une option retournee contient une valuer ou None et gerer le cas du retour
// NULL qui n'est pas a prendre comme une erreur
//Some() est une forme de conteneur qui représente la présence d'une valeur dans un contexte où elle peut être optionnelle,
//via l'énumération Option<T> en Rust.
//Cela facilite la gestion sûre et explicite des cas où des données peuvent ne pas exister.


//String vs &str :
//
//String est un type propriétaire (owned type) :
//
// Il possède et gère son propre espace mémoire, alloué sur le tas (heap).
//
// C’est un type mutable, dont la taille peut changer à l’exécution (on peut l’agrandir ou le modifier).
//
// Il contient un pointeur vers les données, la longueur actuelle, et la capacité allouée.
//
// Utile quand on veut construire, modifier ou posséder une chaîne de caractères.
//
// Ex : let mut s = String::from("Bonjour"); s.push_str(" le monde");
//
//
// &str est une vue (slice) non propriétaire, généralement immuable :
//
// C’est une référence vers une séquence de caractères UTF-8 stockée ailleurs (ex: dans un String ou dans un littéral de chaîne).
//
// Sa taille est fixe (immutable), on ne peut pas la modifier ni changer sa longueur.
//
// Elle contient un pointeur vers les données et leur longueur, mais ne possède pas l’allocation mémoire.
//
// Utile pour passer des chaînes en lecture seule, par exemple en argument de fonction.
//
// Ex : let s: &str = "Bonjour"; ou let slice = &s[0..3]; où s est un String.
