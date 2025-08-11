

struct PasswordConfig {
    length: u8,
    lowercase: bool,
    uppercase: bool,
    digit: bool,
    symbol: bool,
}

enum Strength {
    Weak,
    Medium,
    Strong,
}

fn strength_label(strength: &Strength) -> String {
    match &strength {
        Strength::Weak => "The password strength is weak".to_string(),
        Strength::Medium => "The password strength is medium".to_string(),
        Strength::Strong => "The password strength is strong".to_string(),
    }

}

//impl : methode associee a la struct passwordconfig
impl PasswordConfig {
    fn describe(&self) {
        println!("length = {}", self.length);
        println!("lowercase = {}", self.lowercase);
        println!("uppercase = {}", self.uppercase);
        println!("digit = {}", self.digit);
        println!("symbol = {}", self.symbol);
    }
    //&self : ne modifie pas la struct
    //&mut self : peut modifier la struct
    //pas de self : fonction associee / statique ? (fn name() -> Self)
    //methode qui consomme la struct (fn name (self))
}

fn main() {
    let config = PasswordConfig {
        length: 15,
        lowercase: true,
        uppercase: true,
        digit: true,
        symbol: true,
    };

    config.describe();
}


