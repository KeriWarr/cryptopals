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
use cryptopals::byte_stream::ByteStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments");
    }
    let mut bs1 = ByteStream::from_hex(&args[1]).unwrap();
    let bs2 = ByteStream::from_hex(&args[2]).unwrap();
    bs1.repeating_xor(&bs2);
    println!("{}", bs1.into_hex());
}
