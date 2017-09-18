fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        std::process::exit(1);
    }
    let s = args[1].clone();
    println!("{}", hex_to_base64(s));
    std::process::exit(0);
}

fn hex_to_base64(s: String) -> String {
    return byte_array_to_base64(hex_to_byte_array(s));
}

// #[test]
// fn it_converts_hex_to_base64() {
//     let hex = "49276d206b696c6c696e6720796f757220627261696e206c69
//     6b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
//     let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9
//     vim pc29ub3VzIG11c2hyb29t".to_string();
//     assert_eq!(hex_to_base64(hex), expected);
// }

fn byte_array_to_base64(v: Vec<u8>) -> String {
    return "".to_string();
}

fn hex_to_byte_array(s: String) -> Vec<u8> {
    let s_bytes = s.into_bytes();
    let mut v: Vec<u8> = Vec::new();
    let mut index = 0;
    while 2 * index < s_bytes.len() {
        let mut n = hex_ascii_to_int(*&s_bytes[2 * index]) * 16;
        if 2 * index + 1 < s_bytes.len() {
            n += hex_ascii_to_int(*&s_bytes[2 * index + 1]);
        }
        v.push(n);
        index += 1;
    }
    return v;
}

fn hex_ascii_to_int(n: u8) -> u8 {
    if n >= 48 && n <= 57 {
        return n - 48;
    } else if n >= 97 && n <= 102 {
        return n - 97;
    }
    panic!("invalid hex ascii value!");
}

#[test]
fn it_converts_hex_asciin_to_int() {
    let hex_ascii = 48;
    let expected = 0;
    assert_eq!(hex_ascii_to_int(hex_ascii), expected);
}
