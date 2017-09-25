//!

use std::collections::HashMap;
use std::str;
use std::f32;
use string_utils::{byte_array_to_hex, hex_to_byte_array, bytes_to_ascii_string,
                   base_64_to_byte_array};

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

pub fn xor_cypher_decrypt_char_frequency(s: &String) -> (String, f32, u8) {
    let bytes = hex_to_byte_array(s);
    let mut min_score = f32::INFINITY;
    let mut best_candidate = "".to_string();
    let mut best_key = 0;

    for key in 0..255 as u8 {
        let cypher = vec![key; bytes.len()];
        let cleartext_candidate = fixed_xor(&bytes, &cypher);
        let ascii_string = match bytes_to_ascii_string(&cleartext_candidate) {
            Some(s) => s,
            None => {
                continue;
            }
        };
        let score = score_candidate(&ascii_string);
        if score < min_score {
            min_score = score;
            best_candidate = ascii_string;
            best_key = key;
        }
    }

    (best_candidate, min_score, best_key)
}

fn score_candidate(s: &String) -> f32 {
    let mut map: HashMap<char, u8> = HashMap::new();

    // https://www.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
    let corpus_frequency_data: [(char, f32); 26] = [
        ('a', 0.0812),
        ('b', 0.0149),
        ('c', 0.0271),
        ('d', 0.0432),
        ('e', 0.1202),
        ('f', 0.0230),
        ('g', 0.0203),
        ('h', 0.0592),
        ('i', 0.0731),
        ('j', 0.0010),
        ('k', 0.0069),
        ('l', 0.0398),
        ('m', 0.0261),
        ('n', 0.0695),
        ('o', 0.0768),
        ('p', 0.0182),
        ('q', 0.0011),
        ('r', 0.0602),
        ('s', 0.0628),
        ('t', 0.0910),
        ('u', 0.0288),
        ('v', 0.0111),
        ('w', 0.0209),
        ('x', 0.0017),
        ('y', 0.0211),
        ('z', 0.0007),
    ];

    let stripped_string = s.replace(" ", "");

    for c in s.replace(" ", "").to_lowercase().chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut score = 0.0;
    for &(c, corpus_frequency) in corpus_frequency_data.iter() {
        let letter_frequency = *map.get(&c).unwrap_or(&0) as f32 / stripped_string.len() as f32;
        let letter_score = ((letter_frequency * 100.0 + 1.0).log(2.0) -
                                (corpus_frequency * 100.0 + 1.0).log(2.0))
            .abs();
        score += letter_score;
    }

    let mut modifier = 2.0;
    for c in s.chars() {
        if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == ' ' || c == '.' || c == '\'' {
            modifier *= 1.15;
        } else {
            modifier /= 1.2;
        }
    }
    score -= modifier;
    score += 1.0 / modifier;

    score
}

pub fn detect_single_char_xor(v: &Vec<&str>) -> (usize, String) {
    let mut min_score = f32::INFINITY;
    let mut best_cleartext = "".to_string();
    let mut best_index = 0;

    for (index, s) in v.iter().enumerate() {
        let (best_decoding, score, _) = xor_cypher_decrypt_char_frequency(&s.to_string());
        if score < min_score {
            min_score = score;
            best_cleartext = best_decoding;
            best_index = index;
        }
    }

    (best_index, best_cleartext)
}

pub fn repeating_key_xor(s: &String, key: &String) -> String {
    let bytes = s.clone().into_bytes();
    let key_bytes = key.clone().into_bytes();
    let mut cypher = Vec::new();

    for i in 0..bytes.len() {
        cypher.push(key_bytes[i % key_bytes.len()]);
    }
    byte_array_to_hex(&fixed_xor(&bytes, &cypher))
}

pub fn break_repeating_key_xor(s: &String) -> (String, String) {
    let bytes = base_64_to_byte_array(s);
    let mut best_sizes = [(f32::INFINITY, 0), (f32::INFINITY, 0)];
    for keysize in 2..41 {
        let mut total_distance: f32 = 0.0;
        let mut blocks = 0;
        while blocks < 4 {
            if (blocks + 2) * keysize > bytes.len() {
                break;
            }
            total_distance += hamming_distance(
                &bytes[(blocks * keysize)..((blocks + 1) * keysize)],
                &bytes[((blocks + 1) * keysize)..((blocks + 2) * keysize)],
            ) as f32;
            blocks += 1;
        }
        if blocks < 2 {
            panic!("did not get to check enough blocks at keysize: {}", keysize);
        }
        let average_distance = total_distance / blocks as f32;
        if average_distance < best_sizes[0].0 {
            best_sizes[1] = best_sizes[0];
            best_sizes[0] = (average_distance, keysize);
        } else if average_distance < best_sizes[1].0 {
            best_sizes[1] = (average_distance, keysize);
        }
    }

    let mut best_key: Vec<u8> = Vec::new();
    let mut best_score: f32 = f32::INFINITY;
    let mut best_key_byte_cleartexts: Vec<String> = Vec::new();

    for size in [best_sizes[0].1, best_sizes[1].1].iter() {
        let mut blocks: Vec<&[u8]> = Vec::new();
        let mut index = 0;
        let mut key: Vec<u8> = Vec::new();
        let mut key_byte_cleartexts: Vec<String> = Vec::new();
        let mut total_score: f32 = 0.0;
        while index * size + size <= bytes.len() {
            blocks.push(&bytes[(index * size)..((index + 1) * size)]);
            index += 1;
        }

        for key_byte_index in 0..(*size as usize) {
            let mut key_byte_bytes: Vec<u8> = Vec::new();
            for block in &blocks {
                key_byte_bytes.push(block[key_byte_index]);
            }
            let (key_byte_cleartext, score, byte_key) =
                xor_cypher_decrypt_char_frequency(&byte_array_to_hex(&key_byte_bytes));
            key.push(byte_key);
            total_score += score;
            key_byte_cleartexts.push(key_byte_cleartext);
        }

        if total_score < best_score {
            best_score = total_score;
            best_key = key;
            best_key_byte_cleartexts = key_byte_cleartexts;
        }
    }

    (
        best_key_byte_cleartexts.join(""),
        bytes_to_ascii_string(&best_key).unwrap(),
    )
}

// consider output overflow
fn hamming_distance(v1: &[u8], v2: &[u8]) -> u8 {
    if v1.len() != v2.len() {
        panic!("vectors must be the same length");
    }

    let mut total = 0;

    for (b1, b2) in v1.iter().zip(v2.iter()) {
        total += (b1 ^ b2).count_ones() as u8;
    }

    total
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

    mod xor_cypher_decrypt_char_frequency {
        use super::super::xor_cypher_decrypt_char_frequency;

        #[test]
        fn it_solves_the_example() {
            let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
                .to_string();
            let expected = "Cooking MC's like a pound of bacon".to_string();
            let (result, _, _) = xor_cypher_decrypt_char_frequency(&hex);
            assert_eq!(result, expected);
        }
    }

    mod repeating_key_xor {
        use super::super::repeating_key_xor;

        #[test]
        fn it_solves_the_example() {
            let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
                .to_string();
            let key = "ICE".to_string();
            let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string();
            assert_eq!(repeating_key_xor(&input, &key), expected);
        }
    }

    mod hamming_distance {
        use super::super::hamming_distance;

        #[test]
        fn it_solves_the_example() {
            let s1 = "this is a test".to_string();
            let s2 = "wokka wokka!!!".to_string();
            let expected = 37;
            assert_eq!(
                hamming_distance(&s1.into_bytes(), &s2.into_bytes()),
                expected
            );
        }
    }
}
