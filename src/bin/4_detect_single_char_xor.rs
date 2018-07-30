//!
//! ## Executable for: Set 1 - Problem 4
//!
//! # Examples
//!
//! ```shell
//! ./detect_single_char_xor
//! ```
//!

use std::f64;
use std::fs::File;
use std::io::prelude::*;

extern crate cryptopals;
use cryptopals::byte_stream::ByteStream;

fn main() {
    let mut f = File::open("./data/S1P4.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let strs = contents.split_whitespace().collect::<Vec<&str>>();

    let mut best_score = f64::INFINITY;
    let mut best_bs = ByteStream::new();
    for s in strs {
        let mut bs = ByteStream::from_hex(s).unwrap();
        let (byte, score) = bs.break_single_byte_xor();
        bs.byte_xor(byte);
        if score < f64::INFINITY {
            println!("{}\t{}", score, bs.into_ascii());
        }
        if score < best_score {
            best_score = score;
            best_bs = bs;
        }
    }

    println!("\n\n{}\t{}", best_score, best_bs.into_ascii());
}
