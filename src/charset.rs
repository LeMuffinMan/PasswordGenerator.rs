use crate::passwordconfig::PasswordConfig;
//je comprend pas pourquoi ici je dois indiquer crate

const LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z',
];

const UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

const DIGIT: [char; 10] = ['0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9'];

const SYMBOL: [char; 10] = ['!', '@', '#', '$', '%',
    '^', '&', '*', '(', ')'];


pub fn fill_charset(config: &PasswordConfig)
    -> Result<Vec<char>, Box<dyn std::error::Error>>
{
    let mut charset: Vec<char> = Vec::new();
    if config.lowercase {
        charset.extend_from_slice(&LOWERCASE);
    }
    if config.uppercase {
        charset.extend_from_slice(&UPPERCASE);
    }
    if config.digit {
        charset.extend_from_slice(&DIGIT);
    }
    if config.symbol {
        charset.extend_from_slice(&SYMBOL);
    }

    if charset.is_empty() {
        return Err("Charset empty".into());
    }

    Ok(charset)
}
