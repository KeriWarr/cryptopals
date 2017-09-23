//!
//! ## Executable for: Set 1 - Problem 3
//!
//! # Examples
//!
//! ```shell
//! ./single_byte_xor_cypher 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
//! ```
//!

extern crate cryptopals;
use cryptopals::xor::xor_cypher_decrypt_char_frequency;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        std::process::exit(1);
    }
    println!("{}", xor_cypher_decrypt_char_frequency(&args[1]));
    std::process::exit(0);
}
