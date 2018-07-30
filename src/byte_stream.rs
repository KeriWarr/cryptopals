// use core::fmt;

use std::f64;
use std::fmt;
use std::slice::Chunks;
use std::slice::ChunksMut;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Debug)]
pub struct FromHexError {
    string: String,
    valid_up_to: usize,
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid hexidecimal string from index {}",
            self.valid_up_to
        )
    }
}

#[derive(Debug)]
pub struct FromB64Error {
    string: String,
    valid_up_to: usize,
}

impl fmt::Display for FromB64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid base 64 string from index {}", self.valid_up_to)
    }
}

#[derive(Debug)]
pub struct FromAsciiError {
    string: String,
    valid_up_to: usize,
}

impl fmt::Display for FromAsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid ascii string from index {}", self.valid_up_to)
    }
}

#[derive(PartialOrd, Eq, Ord, PartialEq, Debug)]
pub struct ByteStream {
    data: Vec<u8>,
}

pub trait IntoByteStream {
    fn into_byte_stream(self) -> ByteStream;
}

const GALOIS_MUL_9: [u8; 256] = [
    0x00, 0x09, 0x12, 0x1b, 0x24, 0x2d, 0x36, 0x3f, 0x48, 0x41, 0x5a, 0x53, 0x6c, 0x65, 0x7e, 0x77,
    0x90, 0x99, 0x82, 0x8b, 0xb4, 0xbd, 0xa6, 0xaf, 0xd8, 0xd1, 0xca, 0xc3, 0xfc, 0xf5, 0xee, 0xe7,
    0x3b, 0x32, 0x29, 0x20, 0x1f, 0x16, 0x0d, 0x04, 0x73, 0x7a, 0x61, 0x68, 0x57, 0x5e, 0x45, 0x4c,
    0xab, 0xa2, 0xb9, 0xb0, 0x8f, 0x86, 0x9d, 0x94, 0xe3, 0xea, 0xf1, 0xf8, 0xc7, 0xce, 0xd5, 0xdc,
    0x76, 0x7f, 0x64, 0x6d, 0x52, 0x5b, 0x40, 0x49, 0x3e, 0x37, 0x2c, 0x25, 0x1a, 0x13, 0x08, 0x01,
    0xe6, 0xef, 0xf4, 0xfd, 0xc2, 0xcb, 0xd0, 0xd9, 0xae, 0xa7, 0xbc, 0xb5, 0x8a, 0x83, 0x98, 0x91,
    0x4d, 0x44, 0x5f, 0x56, 0x69, 0x60, 0x7b, 0x72, 0x05, 0x0c, 0x17, 0x1e, 0x21, 0x28, 0x33, 0x3a,
    0xdd, 0xd4, 0xcf, 0xc6, 0xf9, 0xf0, 0xeb, 0xe2, 0x95, 0x9c, 0x87, 0x8e, 0xb1, 0xb8, 0xa3, 0xaa,
    0xec, 0xe5, 0xfe, 0xf7, 0xc8, 0xc1, 0xda, 0xd3, 0xa4, 0xad, 0xb6, 0xbf, 0x80, 0x89, 0x92, 0x9b,
    0x7c, 0x75, 0x6e, 0x67, 0x58, 0x51, 0x4a, 0x43, 0x34, 0x3d, 0x26, 0x2f, 0x10, 0x19, 0x02, 0x0b,
    0xd7, 0xde, 0xc5, 0xcc, 0xf3, 0xfa, 0xe1, 0xe8, 0x9f, 0x96, 0x8d, 0x84, 0xbb, 0xb2, 0xa9, 0xa0,
    0x47, 0x4e, 0x55, 0x5c, 0x63, 0x6a, 0x71, 0x78, 0x0f, 0x06, 0x1d, 0x14, 0x2b, 0x22, 0x39, 0x30,
    0x9a, 0x93, 0x88, 0x81, 0xbe, 0xb7, 0xac, 0xa5, 0xd2, 0xdb, 0xc0, 0xc9, 0xf6, 0xff, 0xe4, 0xed,
    0x0a, 0x03, 0x18, 0x11, 0x2e, 0x27, 0x3c, 0x35, 0x42, 0x4b, 0x50, 0x59, 0x66, 0x6f, 0x74, 0x7d,
    0xa1, 0xa8, 0xb3, 0xba, 0x85, 0x8c, 0x97, 0x9e, 0xe9, 0xe0, 0xfb, 0xf2, 0xcd, 0xc4, 0xdf, 0xd6,
    0x31, 0x38, 0x23, 0x2a, 0x15, 0x1c, 0x07, 0x0e, 0x79, 0x70, 0x6b, 0x62, 0x5d, 0x54, 0x4f, 0x46,
];

const GALOIS_MUL_11: [u8; 256] = [
    0x00, 0x0b, 0x16, 0x1d, 0x2c, 0x27, 0x3a, 0x31, 0x58, 0x53, 0x4e, 0x45, 0x74, 0x7f, 0x62, 0x69,
    0xb0, 0xbb, 0xa6, 0xad, 0x9c, 0x97, 0x8a, 0x81, 0xe8, 0xe3, 0xfe, 0xf5, 0xc4, 0xcf, 0xd2, 0xd9,
    0x7b, 0x70, 0x6d, 0x66, 0x57, 0x5c, 0x41, 0x4a, 0x23, 0x28, 0x35, 0x3e, 0x0f, 0x04, 0x19, 0x12,
    0xcb, 0xc0, 0xdd, 0xd6, 0xe7, 0xec, 0xf1, 0xfa, 0x93, 0x98, 0x85, 0x8e, 0xbf, 0xb4, 0xa9, 0xa2,
    0xf6, 0xfd, 0xe0, 0xeb, 0xda, 0xd1, 0xcc, 0xc7, 0xae, 0xa5, 0xb8, 0xb3, 0x82, 0x89, 0x94, 0x9f,
    0x46, 0x4d, 0x50, 0x5b, 0x6a, 0x61, 0x7c, 0x77, 0x1e, 0x15, 0x08, 0x03, 0x32, 0x39, 0x24, 0x2f,
    0x8d, 0x86, 0x9b, 0x90, 0xa1, 0xaa, 0xb7, 0xbc, 0xd5, 0xde, 0xc3, 0xc8, 0xf9, 0xf2, 0xef, 0xe4,
    0x3d, 0x36, 0x2b, 0x20, 0x11, 0x1a, 0x07, 0x0c, 0x65, 0x6e, 0x73, 0x78, 0x49, 0x42, 0x5f, 0x54,
    0xf7, 0xfc, 0xe1, 0xea, 0xdb, 0xd0, 0xcd, 0xc6, 0xaf, 0xa4, 0xb9, 0xb2, 0x83, 0x88, 0x95, 0x9e,
    0x47, 0x4c, 0x51, 0x5a, 0x6b, 0x60, 0x7d, 0x76, 0x1f, 0x14, 0x09, 0x02, 0x33, 0x38, 0x25, 0x2e,
    0x8c, 0x87, 0x9a, 0x91, 0xa0, 0xab, 0xb6, 0xbd, 0xd4, 0xdf, 0xc2, 0xc9, 0xf8, 0xf3, 0xee, 0xe5,
    0x3c, 0x37, 0x2a, 0x21, 0x10, 0x1b, 0x06, 0x0d, 0x64, 0x6f, 0x72, 0x79, 0x48, 0x43, 0x5e, 0x55,
    0x01, 0x0a, 0x17, 0x1c, 0x2d, 0x26, 0x3b, 0x30, 0x59, 0x52, 0x4f, 0x44, 0x75, 0x7e, 0x63, 0x68,
    0xb1, 0xba, 0xa7, 0xac, 0x9d, 0x96, 0x8b, 0x80, 0xe9, 0xe2, 0xff, 0xf4, 0xc5, 0xce, 0xd3, 0xd8,
    0x7a, 0x71, 0x6c, 0x67, 0x56, 0x5d, 0x40, 0x4b, 0x22, 0x29, 0x34, 0x3f, 0x0e, 0x05, 0x18, 0x13,
    0xca, 0xc1, 0xdc, 0xd7, 0xe6, 0xed, 0xf0, 0xfb, 0x92, 0x99, 0x84, 0x8f, 0xbe, 0xb5, 0xa8, 0xa3,
];

const GALOIS_MUL_13: [u8; 256] = [
    0x00, 0x0d, 0x1a, 0x17, 0x34, 0x39, 0x2e, 0x23, 0x68, 0x65, 0x72, 0x7f, 0x5c, 0x51, 0x46, 0x4b,
    0xd0, 0xdd, 0xca, 0xc7, 0xe4, 0xe9, 0xfe, 0xf3, 0xb8, 0xb5, 0xa2, 0xaf, 0x8c, 0x81, 0x96, 0x9b,
    0xbb, 0xb6, 0xa1, 0xac, 0x8f, 0x82, 0x95, 0x98, 0xd3, 0xde, 0xc9, 0xc4, 0xe7, 0xea, 0xfd, 0xf0,
    0x6b, 0x66, 0x71, 0x7c, 0x5f, 0x52, 0x45, 0x48, 0x03, 0x0e, 0x19, 0x14, 0x37, 0x3a, 0x2d, 0x20,
    0x6d, 0x60, 0x77, 0x7a, 0x59, 0x54, 0x43, 0x4e, 0x05, 0x08, 0x1f, 0x12, 0x31, 0x3c, 0x2b, 0x26,
    0xbd, 0xb0, 0xa7, 0xaa, 0x89, 0x84, 0x93, 0x9e, 0xd5, 0xd8, 0xcf, 0xc2, 0xe1, 0xec, 0xfb, 0xf6,
    0xd6, 0xdb, 0xcc, 0xc1, 0xe2, 0xef, 0xf8, 0xf5, 0xbe, 0xb3, 0xa4, 0xa9, 0x8a, 0x87, 0x90, 0x9d,
    0x06, 0x0b, 0x1c, 0x11, 0x32, 0x3f, 0x28, 0x25, 0x6e, 0x63, 0x74, 0x79, 0x5a, 0x57, 0x40, 0x4d,
    0xda, 0xd7, 0xc0, 0xcd, 0xee, 0xe3, 0xf4, 0xf9, 0xb2, 0xbf, 0xa8, 0xa5, 0x86, 0x8b, 0x9c, 0x91,
    0x0a, 0x07, 0x10, 0x1d, 0x3e, 0x33, 0x24, 0x29, 0x62, 0x6f, 0x78, 0x75, 0x56, 0x5b, 0x4c, 0x41,
    0x61, 0x6c, 0x7b, 0x76, 0x55, 0x58, 0x4f, 0x42, 0x09, 0x04, 0x13, 0x1e, 0x3d, 0x30, 0x27, 0x2a,
    0xb1, 0xbc, 0xab, 0xa6, 0x85, 0x88, 0x9f, 0x92, 0xd9, 0xd4, 0xc3, 0xce, 0xed, 0xe0, 0xf7, 0xfa,
    0xb7, 0xba, 0xad, 0xa0, 0x83, 0x8e, 0x99, 0x94, 0xdf, 0xd2, 0xc5, 0xc8, 0xeb, 0xe6, 0xf1, 0xfc,
    0x67, 0x6a, 0x7d, 0x70, 0x53, 0x5e, 0x49, 0x44, 0x0f, 0x02, 0x15, 0x18, 0x3b, 0x36, 0x21, 0x2c,
    0x0c, 0x01, 0x16, 0x1b, 0x38, 0x35, 0x22, 0x2f, 0x64, 0x69, 0x7e, 0x73, 0x50, 0x5d, 0x4a, 0x47,
    0xdc, 0xd1, 0xc6, 0xcb, 0xe8, 0xe5, 0xf2, 0xff, 0xb4, 0xb9, 0xae, 0xa3, 0x80, 0x8d, 0x9a, 0x97,
];

const GALOIS_MUL_14: [u8; 256] = [
    0x00, 0x0e, 0x1c, 0x12, 0x38, 0x36, 0x24, 0x2a, 0x70, 0x7e, 0x6c, 0x62, 0x48, 0x46, 0x54, 0x5a,
    0xe0, 0xee, 0xfc, 0xf2, 0xd8, 0xd6, 0xc4, 0xca, 0x90, 0x9e, 0x8c, 0x82, 0xa8, 0xa6, 0xb4, 0xba,
    0xdb, 0xd5, 0xc7, 0xc9, 0xe3, 0xed, 0xff, 0xf1, 0xab, 0xa5, 0xb7, 0xb9, 0x93, 0x9d, 0x8f, 0x81,
    0x3b, 0x35, 0x27, 0x29, 0x03, 0x0d, 0x1f, 0x11, 0x4b, 0x45, 0x57, 0x59, 0x73, 0x7d, 0x6f, 0x61,
    0xad, 0xa3, 0xb1, 0xbf, 0x95, 0x9b, 0x89, 0x87, 0xdd, 0xd3, 0xc1, 0xcf, 0xe5, 0xeb, 0xf9, 0xf7,
    0x4d, 0x43, 0x51, 0x5f, 0x75, 0x7b, 0x69, 0x67, 0x3d, 0x33, 0x21, 0x2f, 0x05, 0x0b, 0x19, 0x17,
    0x76, 0x78, 0x6a, 0x64, 0x4e, 0x40, 0x52, 0x5c, 0x06, 0x08, 0x1a, 0x14, 0x3e, 0x30, 0x22, 0x2c,
    0x96, 0x98, 0x8a, 0x84, 0xae, 0xa0, 0xb2, 0xbc, 0xe6, 0xe8, 0xfa, 0xf4, 0xde, 0xd0, 0xc2, 0xcc,
    0x41, 0x4f, 0x5d, 0x53, 0x79, 0x77, 0x65, 0x6b, 0x31, 0x3f, 0x2d, 0x23, 0x09, 0x07, 0x15, 0x1b,
    0xa1, 0xaf, 0xbd, 0xb3, 0x99, 0x97, 0x85, 0x8b, 0xd1, 0xdf, 0xcd, 0xc3, 0xe9, 0xe7, 0xf5, 0xfb,
    0x9a, 0x94, 0x86, 0x88, 0xa2, 0xac, 0xbe, 0xb0, 0xea, 0xe4, 0xf6, 0xf8, 0xd2, 0xdc, 0xce, 0xc0,
    0x7a, 0x74, 0x66, 0x68, 0x42, 0x4c, 0x5e, 0x50, 0x0a, 0x04, 0x16, 0x18, 0x32, 0x3c, 0x2e, 0x20,
    0xec, 0xe2, 0xf0, 0xfe, 0xd4, 0xda, 0xc8, 0xc6, 0x9c, 0x92, 0x80, 0x8e, 0xa4, 0xaa, 0xb8, 0xb6,
    0x0c, 0x02, 0x10, 0x1e, 0x34, 0x3a, 0x28, 0x26, 0x7c, 0x72, 0x60, 0x6e, 0x44, 0x4a, 0x58, 0x56,
    0x37, 0x39, 0x2b, 0x25, 0x0f, 0x01, 0x13, 0x1d, 0x47, 0x49, 0x5b, 0x55, 0x7f, 0x71, 0x63, 0x6d,
    0xd7, 0xd9, 0xcb, 0xc5, 0xef, 0xe1, 0xf3, 0xfd, 0xa7, 0xa9, 0xbb, 0xb5, 0x9f, 0x91, 0x83, 0x8d,
];

const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
];

const RCON: [u8; 256] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a,
    0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39,
    0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a,
    0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8,
    0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef,
    0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc,
    0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b,
    0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3,
    0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94,
    0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20,
    0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35,
    0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f,
    0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04,
    0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63,
    0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd,
    0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d,
];

impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream { data: Vec::new() }
    }

    pub fn from<I>(data: I) -> ByteStream
    where
        I: IntoByteStream,
    {
        data.into_byte_stream()
    }

    pub fn from_bytes(bytes: &[u8]) -> ByteStream {
        ByteStream {
            data: Vec::from(bytes),
        }
    }

    pub fn from_hex(s: &str) -> Result<ByteStream, FromHexError> {
        let mut bs = Self::new();
        for (i, c) in s.chars().enumerate() {
            match hex_decode(c) {
                Some(d) => {
                    if i % 2 == 0 {
                        bs.push(d * 16)
                    } else if let Some(last) = bs.data.last_mut() {
                        *last += d
                    } else {
                        panic!("this is a terrible mistake!")
                    }
                }
                None => {
                    return Err(FromHexError {
                        string: String::from(s),
                        valid_up_to: i,
                    })
                }
            }
        }
        Ok(bs)
    }

    pub fn from_b64(s: &str) -> Result<ByteStream, FromB64Error> {
        let mut bs = Self::new();
        let mut offset = 0;
        for (i, c) in s.chars().enumerate() {
            match b64_decode(c) {
                Some(d) => {
                    if let Some(last) = bs.data.last_mut() {
                        match offset {
                            0 => (),
                            2 => *last += d,
                            4 => *last += d / 4,
                            6 => *last += d / 16,
                            _ => panic!(""),
                        }
                    }
                    match offset {
                        0 => bs.push(d * 4),
                        2 => (),
                        4 => bs.push((d & 3) * 64),
                        6 => bs.push((d & 15) * 16),
                        _ => panic!(""),
                    }
                }
                None => {
                    return Err(FromB64Error {
                        string: String::from(s),
                        valid_up_to: i,
                    })
                }
            }
            offset = (offset + 6) % 8;
        }
        Ok(bs)
    }

    pub fn from_ascii(s: &str) -> Result<ByteStream, FromAsciiError> {
        let mut bs = Self::new();
        for (i, c) in s.chars().enumerate() {
            if c as u8 <= 0x7F {
                bs.push(c as u8);
            } else {
                return Err(FromAsciiError {
                    string: String::from(s),
                    valid_up_to: i,
                });
            }
        }
        Ok(bs)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    pub fn into_hex(self) -> String {
        let mut s = String::new();
        for b in self.data {
            s.push(hex_encode((b >> 4) & 15u8).unwrap());
            s.push(hex_encode(b & 15u8).unwrap());
        }
        s
    }

    pub fn into_b64(self) -> String {
        let mut s = String::new();
        let mut offset = 0;
        for (i, b) in self.data.iter().enumerate() {
            if offset == 0 {
                s.push(b64_encode(b >> 2).unwrap());
                offset = (offset + 6) % 8;
            }
            let left_octet = (b << (offset - 2)) & 63u8;
            let right_octet = self.data
                .get(i + 1)
                .unwrap_or(&0u8)
                .checked_shr(10 - offset)
                .unwrap_or(0u8);
            s.push(b64_encode(left_octet + right_octet).unwrap());
            offset = (offset + 6) % 8;
        }

        s
    }

    pub fn into_ascii(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
    }

    pub fn push(&mut self, byte: u8) {
        self.data.push(byte)
    }

    pub fn append(&mut self, other: &mut ByteStream) {
        self.data.append(&mut other.data)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn byte_xor(&mut self, byte: u8) {
        for b in self.data.iter_mut() {
            *b = *b ^ byte
        }
    }

    pub fn repeating_xor(&mut self, other: &Self) {
        for (i, b) in self.data.iter_mut().enumerate() {
            *b = *b ^ other.data[i % other.data.len()]
        }
    }

    pub fn iter(&self) -> Iter<u8> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.data.iter_mut()
    }

    pub fn iter_blocks(&self, size: usize) -> Chunks<u8> {
        self.data.chunks(size)
    }

    pub fn iter_blocks_mut(&mut self, size: usize) -> ChunksMut<u8> {
        self.data.chunks_mut(size)
    }

    pub fn score_letter_freq(&self) -> f64 {
        const EXPECTED: [f64; 26] = [
            8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 9.966, 0.153, 0.772, 4.025,
            2.406, 6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150,
            1.974, 0.074,
        ];

        let mut counts: [i32; 26] = [0; 26];
        let mut unusual_count = 0;

        for b in &self.data {
            if !b.is_ascii() {
                return f64::INFINITY;
            }
            if b.is_ascii_alphabetic() {
                counts[(b.to_ascii_uppercase() - 65) as usize] += 1;
            } else if b != &b' ' && b != &b',' && b != &b'.' && b != &b'\'' {
                unusual_count += 1;
            }
        }

        let mut score = 0.0;
        for i in 0..26 {
            if counts[i] == 0 {
                continue;
            }
            score +=
                ((100.0 * counts[i] as f64 / self.data.len() as f64).ln() - EXPECTED[i].ln()).abs();
        }

        score += unusual_count as f64;
        score *= 1.1f64.powf(unusual_count as f64);

        score
    }

    pub fn break_single_byte_xor(&mut self) -> (u8, f64) {
        let mut best_score = f64::INFINITY;
        let mut best_byte: u32 = 0;
        for i in 0..256 {
            self.byte_xor(i as u8);
            let score = self.score_letter_freq();
            if score < best_score {
                best_score = score;
                best_byte = i;
            }
            self.byte_xor(i as u8);
        }
        (best_byte as u8, best_score)
    }

    pub fn get_each_nth_byte(&self, n: i32, offset: i32) -> ByteStream {
        assert!(offset < n);

        let mut bs = ByteStream::new();

        let mut i = offset;
        while i < self.data.len() as i32 {
            bs.push(self.data[i as usize]);
            i += n;
        }

        bs
    }

    pub fn break_repeating_key_xor(&mut self) -> Option<Self> {
        let mut best_edit_dist = f64::INFINITY;
        let mut best_keysize: i32 = 0;

        for keysize in 2..40 {
            let mut iter = self.iter_blocks(keysize as usize);
            match (iter.next(), iter.next(), iter.next(), iter.next()) {
                (Some(b1), Some(b2), Some(b3), Some(b4)) => {
                    if b1.len() != b4.len() {
                        return None;
                    }
                    let dist = (edit_dist(b1, b2).unwrap()
                        + edit_dist(b1, b3).unwrap()
                        + edit_dist(b1, b4).unwrap()
                        + edit_dist(b2, b3).unwrap()
                        + edit_dist(b2, b4).unwrap()
                        + edit_dist(b3, b4).unwrap()) as f64
                        / keysize as f64;
                    if dist < best_edit_dist {
                        best_edit_dist = dist;
                        best_keysize = keysize;
                    }
                }
                _ => return None,
            }
        }

        let mut key = ByteStream::new();

        for offset in 0..best_keysize {
            let mut bs = self.get_each_nth_byte(best_keysize, offset);
            let (byte, _score) = bs.break_single_byte_xor();
            key.push(byte);
        }

        Some(key)
    }

    pub fn key_schedule_core(&mut self, i: i32) {
        self.data.rotate_left(1);
        self.data[0] = self.data[0] ^ RCON[i as usize];
    }

    pub fn get_aes_round_keys(&self, key: Self) -> Vec<ByteStream> {
        assert!(key.data.len() == 16);
        let mut keys = Vec::new();
        let mut last_key = key;
        for i in 1..12 {
            let mut t = ByteStream::from_bytes(&[
                last_key.data[12],
                last_key.data[13],
                last_key.data[14],
                last_key.data[15],
            ]);
            t.key_schedule_core(i);
            let mut new_key = ByteStream::new();

            for chunk in last_key.iter_blocks_mut(4) {
                let mut new_chunk =
                    ByteStream::from_bytes(&[chunk[0], chunk[1], chunk[2], chunk[3]]);
                new_chunk.repeating_xor(&t);
                new_key.append(&mut new_chunk);
            }

            assert_eq!(new_key.data.len(), 16);

            keys.push(last_key);
            last_key = new_key;
        }
        keys.push(last_key);
        keys.remove(0);
        keys
    }

    const ECB_BLOCK_SIZE: usize = 16;

    fn unshift_rows(&mut self) {
        for chunk in self.iter_blocks_mut(Self::ECB_BLOCK_SIZE) {
            if chunk.len() < Self::ECB_BLOCK_SIZE {
                panic!("unpadded blocks!");
            }
            let tmp = chunk[1];
            chunk[1] = chunk[13];
            chunk[13] = chunk[9];
            chunk[9] = chunk[5];
            chunk[5] = tmp;
            let tmp = chunk[2];
            chunk[2] = chunk[10];
            chunk[10] = tmp;
            let tmp = chunk[6];
            chunk[6] = chunk[14];
            chunk[14] = tmp;
            let tmp = chunk[3];
            chunk[3] = chunk[7];
            chunk[7] = chunk[11];
            chunk[11] = chunk[15];
            chunk[15] = tmp;
        }
    }

    fn unsub_bytes(&mut self) {
        for b in self.data.iter_mut() {
            *b = INV_S_BOX[*b as usize];
        }
    }

    fn unmix_columns(&mut self) {
        for chunk in self.iter_blocks_mut(Self::ECB_BLOCK_SIZE) {
            if chunk.len() < Self::ECB_BLOCK_SIZE {
                panic!("unpadded blocks!");
            }
            for col in 0..4 {
                chunk[0 + 4 * col] = GALOIS_MUL_14[chunk[0 + 4 * col] as usize]
                    ^ GALOIS_MUL_11[chunk[1 + 4 * col] as usize]
                    ^ GALOIS_MUL_13[chunk[2 + 4 * col] as usize]
                    ^ GALOIS_MUL_9[chunk[3 + 4 * col] as usize];
                chunk[1 + 4 * col] = GALOIS_MUL_9[chunk[0 + 4 * col] as usize]
                    ^ GALOIS_MUL_14[chunk[1 + 4 * col] as usize]
                    ^ GALOIS_MUL_11[chunk[2 + 4 * col] as usize]
                    ^ GALOIS_MUL_13[chunk[3 + 4 * col] as usize];
                chunk[2 + 4 * col] = GALOIS_MUL_13[chunk[0 + 4 * col] as usize]
                    ^ GALOIS_MUL_9[chunk[1 + 4 * col] as usize]
                    ^ GALOIS_MUL_14[chunk[2 + 4 * col] as usize]
                    ^ GALOIS_MUL_11[chunk[3 + 4 * col] as usize];
                chunk[3 + 4 * col] = GALOIS_MUL_11[chunk[0 + 4 * col] as usize]
                    ^ GALOIS_MUL_13[chunk[1 + 4 * col] as usize]
                    ^ GALOIS_MUL_9[chunk[2 + 4 * col] as usize]
                    ^ GALOIS_MUL_14[chunk[3 + 4 * col] as usize];
            }
        }
    }

    pub fn decrypt_aes_128_ecb(&mut self, key: Self) {
        let round_keys = self.get_aes_round_keys(key);

        // round 10
        self.repeating_xor(&round_keys[10]);
        self.unshift_rows();
        self.unsub_bytes();

        // rounds 9-1
        for i in (1..10).rev() {
            self.repeating_xor(&round_keys[i]);
            self.unmix_columns();
            self.unshift_rows();
            self.unsub_bytes();
        }

        // round 0
        self.repeating_xor(&round_keys[0]);
    }
}

impl IntoByteStream for Vec<u8> {
    fn into_byte_stream(self) -> ByteStream {
        ByteStream { data: self }
    }
}

impl<'a> IntoByteStream for &'a [u8] {
    fn into_byte_stream(self) -> ByteStream {
        ByteStream::from_bytes(self)
    }
}

impl<'a> IntoByteStream for &'a mut [u8] {
    fn into_byte_stream(self) -> ByteStream {
        ByteStream::from_bytes(self)
    }
}

impl Clone for ByteStream {
    fn clone(&self) -> Self {
        ByteStream {
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

fn edit_dist(a: &[u8], b: &[u8]) -> Option<u32> {
    if a.len() != b.len() {
        return None;
    }
    let mut dist = 0;
    for i in 0..a.len() {
        dist += (a[i] ^ b[i]).count_ones()
    }

    Some(dist)
}

fn b64_encode(n: u8) -> Option<char> {
    if n > 63 {
        return None;
    }

    Some(
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
        }) as char,
    )
}

fn hex_encode(n: u8) -> Option<char> {
    if n > 15 {
        return None;
    }

    Some((if n <= 9 { n + 48 } else { n + 87 }) as char)
}

fn b64_decode(c: char) -> Option<u8> {
    (if c >= 'A' && c <= 'Z' {
        Some(c as u8 - 65)
    } else if c >= 'a' && c <= 'z' {
        Some(c as u8 - 71)
    } else if c >= '0' && c <= '9' {
        Some(c as u8 + 4)
    } else if c == '+' {
        Some(62)
    } else if c == '/' {
        Some(63)
    } else {
        None
    })
}

fn hex_decode(c: char) -> Option<u8> {
    match c.to_digit(16) {
        Some(d) => Some(d as u8),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    mod byte_stream {
        use super::super::ByteStream;

        #[test]
        fn it_converts_hex_to_b64() {
            let bs = ByteStream::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
            assert_eq!(
                bs.into_b64(),
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            );
        }

        #[test]
        fn it_converts_b64_to_hex() {
            let bs = ByteStream::from_b64(
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            ).unwrap();
            assert_eq!(bs.into_hex(), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        }

        #[test]
        fn it_performs_xor() {
            let mut bs1 = ByteStream::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
            let bs2 = ByteStream::from_hex("686974207468652062756c6c277320657965").unwrap();
            bs1.repeating_xor(&bs2);
            assert_eq!(bs1.into_hex(), "746865206b696420646f6e277420706c6179");
        }
    }

    mod edit_dist {
        use super::super::edit_dist;

        #[test]
        fn it_calculates_edit_dist() {
            let b1 = String::from("this is a test").into_bytes();
            let b2 = String::from("wokka wokka!!!").into_bytes();

            assert_eq!(37, edit_dist(&b1, &b2).unwrap());
        }
    }
}