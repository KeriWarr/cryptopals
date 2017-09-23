//!
//! Functions for:
//! - converting string encodings into other string encodings
//! - converting string encodings into byte vectors
//! - converting byte vectors into string encodings

use std::fmt::Write;

///
/// Generates a base64 encoding of a hex string
///
/// # Panics
/// - If `s` is not of even length
/// - If `s` contains non-hexadecimal characters
///
pub fn hex_to_base64(s: &String) -> String {
    if s.len() % 2 != 0 {
        panic!("Input hex string must have even length.");
    }
    let byte_array = hex_to_byte_array(s);

    byte_array_to_base64(&byte_array)
}

pub fn hex_to_byte_array(s: &String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < s.len() {
        v.push(u8::from_str_radix(&s[index..(index + 2)], 16).unwrap());
        index += 2;
    }

    v
}

fn byte_array_to_base64(v: &Vec<u8>) -> String {
    let mut bits = 0;
    let mut s = String::new();
    while bits < 8 * v.len() {
        // take the next byte, padded with zeroes
        let v_bits = (v[bits / 8] as u16) << 8;
        // if there is a subsequent byte, add it on
        let v_bits = if bits < 8 * (v.len() - 1) {
            v_bits + (v[bits / 8 + 1] as u16)
        } else {
            v_bits
        };

        let offset = bits % 8;
        let base_64_value = ((v_bits & (0b1111110000000000 >> offset)) >> (10 - offset)) as u8;

        let base_64_char = base_64_to_ascii(base_64_value) as char;
        s.push(base_64_char);
        bits += 6;
    }

    s
}

fn base_64_to_ascii(n: u8) -> u8 {
    if n > 63 {
        panic!("n was not a valid base64 int");
    }

    (if n <= 25 {
         n + 65
     } else if n >= 26 && n <= 51 {
         n + 71
     } else if n >= 52 && n <= 61 {
         n - 4
     } else if n == 62 {
         43
     } else {
         47
     })
}

pub fn byte_array_to_hex(v: &Vec<u8>) -> String {
    let mut s = String::new();
    for &byte in v {
        write!(&mut s, "{:x}", byte).expect("Unable to write");
    }

    s
}


#[cfg(test)]
mod tests {
    mod hex_to_byte_array {
        use super::super::hex_to_byte_array;

        #[test]
        fn it_converts_the_empty_string() {
            let hex = "".to_string();
            let expected = [];
            assert_eq!(hex_to_byte_array(&hex), expected);
        }

        #[test]
        fn it_converts_a_short_string() {
            let hex = "4ac9".to_string();
            let expected = [74, 201];
            assert_eq!(hex_to_byte_array(&hex), expected);
        }

        #[test]
        fn it_converts_a_long_string() {
            let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b652\
                       06120706f"
                .to_string();
            let expected = [
                73,
                39,
                109,
                32,
                107,
                105,
                108,
                108,
                105,
                110,
                103,
                32,
                121,
                111,
                117,
                114,
                32,
                98,
                114,
                97,
                105,
                110,
                32,
                108,
                105,
                107,
                101,
                32,
                97,
                32,
                112,
                111,
            ];
            assert_eq!(hex_to_byte_array(&hex), expected);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_odd_length_string() {
            let hex = "4ac93".to_string();
            hex_to_byte_array(&hex);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_non_hex_characters() {
            let hex = "4ag9".to_string();
            hex_to_byte_array(&hex);
        }
    }

    mod byte_array_to_hex {
        use super::super::byte_array_to_hex;

        #[test]
        fn it_converts_the_empty_string() {
            let byte_array = vec![];
            let expected = "".to_string();
            assert_eq!(byte_array_to_hex(&byte_array), expected);
        }

        #[test]
        fn it_converts_a_short_string() {
            let byte_array = vec![74, 201];
            let expected = "4ac9".to_string();
            assert_eq!(byte_array_to_hex(&byte_array), expected);
        }

        #[test]
        fn it_converts_a_long_string() {
            let byte_array = vec![
                73,
                39,
                109,
                32,
                107,
                105,
                108,
                108,
                105,
                110,
                103,
                32,
                121,
                111,
                117,
                114,
                32,
                98,
                114,
                97,
                105,
                110,
                32,
                108,
                105,
                107,
                101,
                32,
                97,
                32,
                112,
                111,
            ];
            let expected = "49276d206b696c6c696e6720796f757220627261696e206c696b652\
                            06120706f"
                .to_string();
            assert_eq!(byte_array_to_hex(&byte_array), expected);
        }
    }

    mod hex_to_base64 {
        use super::super::hex_to_base64;

        #[test]
        fn it_converts_the_empty_string() {
            let hex = "".to_string();
            let expected = "";
            assert_eq!(hex_to_base64(&hex), expected);
        }

        #[test]
        fn it_converts_a_short_string() {
            let hex = "4ac9".to_string();
            let expected = "Ssk";
            assert_eq!(hex_to_base64(&hex), expected);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_odd_length_string() {
            let hex = "4ac93".to_string();
            hex_to_base64(&hex);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_non_hex_characters() {
            let hex = "4ag9".to_string();
            hex_to_base64(&hex);
        }

        #[test]
        fn it_converts_hex_to_base64() {
            let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b652\
                       06120706f69736f6e6f7573206d757368726f6f6d"
                .to_string();
            let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3\
                            VzIG11c2hyb29t"
                .to_string();
            assert_eq!(hex_to_base64(&hex), expected);
        }
    }
}
