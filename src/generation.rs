use rand::prelude::IndexedRandom;
use rand::rng; // not in std

// - generation.rs imports rand::rng and rand::prelude::IndexedRandom, and calls charset.choose(&mut _rng).
// - The conventional approach is rand::thread_rng() and the SliceRandom trait (rand::seq::SliceRandom). The current imports are likely to fail or be unstable across rand versions.
// - Fix: use rand::seq::SliceRandom and thread_rng() or a CSPRNG (see Security).
// - Use rand::rngs::OsRng with SliceRandom::choose() / choose_multiple(), or
// - Use rand_chacha::ChaCha20Rng seeded from OsRng for speed + reproducibility under test.
// - Current approach retries random picks and does O(n) contains() scans every iteration. This gets slow as the password grows.

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
            if !duplicate && password.contains(c) {
                continue;
            }
            password.push(c);
        }
    }
    password
}
