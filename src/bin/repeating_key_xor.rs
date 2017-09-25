//!
//! ## Executable for: Set 1 - Problem 2
//!
//! # Examples
//!
//! ```shell
//! ./fixed_xor 1c0111001f010100061a024b53535009181c 686974207468652062756c6c277320657965
//! ```
//!

extern crate cryptopals;
use cryptopals::xor::repeating_key_xor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments");
    }
    println!("{}", repeating_key_xor(&args[1], &args[2]));
}
