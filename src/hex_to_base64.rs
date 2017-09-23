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
pub fn hex_to_base64(s: &String) -> String {
    if s.len() % 2 != 0 {
        panic!("Input hex string must have even length.");
    }
    let byte_array = hex_to_byte_array(s);

    byte_array_to_base64(&byte_array)
}

fn hex_to_byte_array(s: &String) -> Vec<u8> {
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


#[cfg(test)]
mod tests {
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
