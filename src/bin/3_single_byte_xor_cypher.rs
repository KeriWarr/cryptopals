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
use cryptopals::byte_stream::ByteStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let mut bs = ByteStream::from_hex(&args[1]).unwrap();
    let (byte, _score) = bs.break_single_byte_xor();
    bs.byte_xor(byte);
    println!("{}", bs.into_ascii());
}
