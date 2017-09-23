//!

use string_utils::{byte_array_to_hex, hex_to_byte_array};

///
/// Generates an xor'd hex encoding of two hex strings
///
/// # Panics
/// - If `s1` is not the same length as `s2`
/// - If `s1` or `s2` contains non-hexadecimal characters
///
pub fn hex_fixed_xor(s1: &String, s2: &String) -> String {
    if s1.len() != s2.len() {
        panic!("Input strings must be the same length");
    }

    byte_array_to_hex(&fixed_xor(&hex_to_byte_array(s1), &hex_to_byte_array(s2)))
}

fn fixed_xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    if v1.len() != v2.len() {
        panic!("Input vectors must be the same length");
    }

    let mut v = Vec::new();
    let mut index = 0;
    while index < v1.len() {
        v.push(v1[index] ^ v2[index]);
        index += 1;
    }

    v
}


#[cfg(test)]
mod tests {
    mod hex_fixed_xor {
        use super::super::hex_fixed_xor;

        #[test]
        fn it_xors_empty_strings() {
            let hex = "".to_string();
            assert_eq!(hex_fixed_xor(&hex, &hex), hex);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_odd_length_strings() {
            let hex = "4ac93".to_string();
            hex_fixed_xor(&hex, &hex);
        }

        #[test]
        #[should_panic]
        fn it_panics_on_non_hex_characters() {
            let hex = "4ag9".to_string();
            hex_fixed_xor(&hex, &hex);
        }

        #[test]
        fn it_xors_hex_strings() {
            let hex1 = "1c0111001f010100061a024b53535009181c".to_string();
            let hex2 = "686974207468652062756c6c277320657965".to_string();
            let expected = "746865206b696420646f6e277420706c6179".to_string();
            assert_eq!(hex_fixed_xor(&hex1, &hex2), expected);
        }
    }
}
