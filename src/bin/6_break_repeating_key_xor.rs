//!
//! ## Executable for: Set 1 - Problem 6
//!
//! # Examples
//!
//! ```shell
//! ./break_repeating_key_xor
//! ```
//!

use std::fs::File;
use std::io::prelude::*;

extern crate cryptopals;
use cryptopals::xor::break_repeating_key_xor;

fn main() {
    let mut f = File::open("./data/S1P6.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    contents = contents.replace("\n", "");
    let (cleartext, key) = break_repeating_key_xor(&contents);
    println!("ClearText: {}\nKey: {}", cleartext, key);
}
