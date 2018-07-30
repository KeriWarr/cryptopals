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
use cryptopals::byte_stream::ByteStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let bs = ByteStream::from_hex(&args[1]).unwrap();
    println!("{}", bs.into_b64());
}
