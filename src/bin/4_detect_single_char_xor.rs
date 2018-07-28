//!
//! ## Executable for: Set 1 - Problem 4
//!
//! # Examples
//!
//! ```shell
//! ./detect_single_char_xor
//! ```
//!

use std::fs::File;
use std::io::prelude::*;

extern crate cryptopals;
use cryptopals::xor::detect_single_char_xor;

fn main() {
    let mut f = File::open("./data/S1P4.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    let strs = contents.split_whitespace().collect::<Vec<&str>>();
    let (index, cleartext) = detect_single_char_xor(&strs);
    println!(
        "Index: {}\nOriginal: {}\nCleartext: {}",
        index,
        strs[index],
        cleartext
    );
}
