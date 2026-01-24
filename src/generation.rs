use rand::prelude::IndexedRandom;
use rand::rng;

pub fn generation(charset: &[char], length: u8, duplicate: bool) -> String {
    //declare an instance of random number generator
    let mut _rng = rng();

    let mut password = String::with_capacity(length as usize);

    while password.len() < length as usize {
        if let Some(&c) = charset.choose(&mut _rng) {
            if !duplicate && password.contains(c) {
                continue;
            }
            password.push(c);
        }
    }
    password
}
