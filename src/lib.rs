
pub fn hex_to_base64(s: String) -> String {
    return byte_array_to_base64(hex_to_byte_array(s));
}

fn byte_array_to_base64(v: Vec<u8>) -> String {
    return v[1].to_string();
}

// Assumes s is even length
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
    use super::*;

    #[test]
    fn it_converts_hex_to_byte_array() {
        let hex = "4ac9".to_string();
        let expected = [74, 201];
        assert_eq!(hex_to_byte_array(hex), expected);
    }

    // #[test]
    // fn it_converts_hex_to_base64() {
    //     let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b6520612\
    //                0706f69736f6e6f7573206d757368726f6f6d"
    //         .to_string();
    //     let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG\
    //                     11c2hyb29t"
    //         .to_string();
    //     assert_eq!(hex_to_base64(hex), expected);
    // }
}
