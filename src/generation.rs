use rand::rng; // not in std 
use rand::prelude::IndexedRandom;

//type &[char] allow to use charset without consuming it ?
//it's a slice, it's borrowing
//may i want to consume it once i used it in this function ?
pub fn generation(charset: &[char], length: u8) -> String {

    //declare an instance of random number generator
    //not needed using thread_rng ?
    let mut _rng = rng();

    //Declaring a new empty String
    let mut password = String::with_capacity(length as usize);

    for _ in 0..length {

        if let Some(&c) = charset.choose(&mut _rng) {
            password.push(c); //add at the end
        }
    }
    println!("Generated password : {password}");
    password
} 
