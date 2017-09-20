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
use cryptopals::hex_to_base64::hex_to_base64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        std::process::exit(1);
    }
    let s = args[1].clone();
    println!("{}", hex_to_base64(s));
    std::process::exit(0);
}
