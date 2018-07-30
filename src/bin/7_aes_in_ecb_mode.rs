use cryptopals::byte_stream::ByteStream;
use std::fs::File;
use std::io::prelude::*;

extern crate cryptopals;

fn main() {
	let mut f = File::open("./data/S1P7.txt").expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	contents = contents.replace("\n", "");
	let mut bs = ByteStream::from_b64(&contents).unwrap();
	let key = ByteStream::from_ascii("YELLOW SUBMARINE").unwrap();

	bs.decrypt_aes_128_ecb(key);

	println!("{}", bs.into_ascii());
}
