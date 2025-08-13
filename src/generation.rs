use rand::rng; // not in std 
use rand::prelude::IndexedRandom;

//type &[char] allow to use charset without consuming it ?
//it's a slice, it's borrowing
//may i want to consume it once i used it in this function ?
///using rand to pick random chars until we pushed length elements in the string
///optionally check for duplicated chars 
pub fn generation(charset: &[char], length: u8, duplicate: bool) -> String {

    //declare an instance of random number generator
    //rng or thread_rng ?
    let mut _rng = rng();

    //Declaring a new empty String
    let mut password = String::with_capacity(length as usize);

    while password.len() < length as usize {
        if let Some(&c) = charset.choose(&mut _rng) {
                if duplicate {
                    if password.contains(c) {
                        continue;
                    }
                }
                password.push(c);
            } 
        }
    password
}
