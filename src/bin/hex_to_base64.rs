//!
//! ## Executable for: Set 1 - Problem 1
//!
//! # Examples
//!
//! ```shell
//! ./hex_to_base64 0123456789abcdef
//! ```
//!

extern crate cryptopals;
use cryptopals::string_utils::hex_to_base64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    println!("{}", hex_to_base64(&args[1]));
}
