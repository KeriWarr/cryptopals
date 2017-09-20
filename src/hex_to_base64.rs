//!
//! Logic for Set 1 - Problem 1
//!

///
/// Converts a hex string into a base64 string
///
/// # Panics
/// - If `s` is not of even length
/// - If `s` contains non-hexadecimal characters
///
pub fn hex_to_base64(s: String) -> String {
    if s.len() % 2 != 0 {
        panic!("Input hex string must have even length.");
    }
    return byte_array_to_base64(hex_to_byte_array(s));
}

fn byte_array_to_base64(v: Vec<u8>) -> String {
    return v[1].to_string();
}

fn hex_to_byte_array(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < s.len() {
        v.push(u8::from_str_radix(&s[index..(index + 2)], 16).unwrap());
        index += 2;
    }
    return v;
}


#[cfg(test)]
mod tests {
    mod hex_to_byte_array {
        use super::super::hex_to_byte_array;

        #[test]
        fn it_converts_the_empty_string() {
            let hex = "".to_string();
            let expected = [];
            assert_eq!(hex_to_byte_array(hex), expected);
        }

        #[test]
        fn it_converts_a_short_string() {
            let hex = "4ac9".to_string();
            let expected = [74, 201];
            assert_eq!(hex_to_byte_array(hex), expected);
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
            assert_eq!(hex_to_byte_array(hex), expected);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_odd_length_string() {
            let hex = "4ac93".to_string();
            hex_to_byte_array(hex);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_non_hex_characters() {
            let hex = "4ag9".to_string();
            hex_to_byte_array(hex);
        }
    }

    mod hex_to_base64 {
        use super::super::hex_to_base64;

        #[test]
        fn it_converts_hex_to_base64() {
            let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b652\
                       06120706f69736f6e6f7573206d757368726f6f6d"
                .to_string();
            let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3\
                            VzIG11c2hyb29t"
                .to_string();
            assert_eq!(hex_to_base64(hex), expected);
        }
    }
}
