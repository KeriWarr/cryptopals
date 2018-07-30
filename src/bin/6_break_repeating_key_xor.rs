//!
//! ## Executable for: Set 1 - Problem 6
//!
//! # Examples
//!
//! ```shell
//! ./break_repeating_key_xor
//! ```
//!

use cryptopals::byte_stream::ByteStream;
use std::fs::File;
use std::io::prelude::*;

extern crate cryptopals;

fn main() {
    let mut f = File::open("./data/S1P6.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents = contents.replace("\n", "");
    let mut bs = ByteStream::from_b64(&contents).unwrap();

    let key = bs.break_repeating_key_xor().unwrap();

    bs.repeating_xor(&key);
    println!("Plaintext: {}\n", bs.into_ascii());
    println!("Key: {}", key.into_ascii());
}
