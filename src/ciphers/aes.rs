//! Implementation of the Advanced Encryption Standard (AES) block cipher.
//!
//! # AES
//!
//! AES is a symmetric block cipher that encrypts and decrypts data in blocks of 128 bits.
//!
//! The algorithm consists of several steps:
//!
//! 1. **Key Expansion**: The original key is expanded into a key schedule.
//! 2. **Initial Round**: The input block is XORed with the first part of the key schedule.
//! 3. **Rounds**: A number of rounds are performed, each consisting of the following steps:
//!    a. **SubBytes**: Each byte of the block is replaced by a value from a lookup table.
//!    b. **ShiftRows**: The bytes of the block are shifted to the left.
//!    c. **MixColumns**: Each column of the block is multiplied with a fixed matrix.
//! 4. **Final Round**: The block is passed through the SubBytes and ShiftRows steps, but not MixColumns.
//! 5. **Output**: The block is XORed with the last part of the key schedule.
//!
//! The number of rounds depends on the key size:
//!
//! - 128-bit key: 10 rounds
//! - 192-bit key: 12 rounds
//! - 256-bit key: 14 rounds
//!
//! # Example
//!
//! ```rust
//! use rust_algorithms::ciphers::{aes_encrypt, AesKey::AesKey128};
//!
//! let key = [
//!    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
//!    0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
//! ];
//!
//! let plaintext = [
//!   0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
//!   0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
//! ];
//!
//! let ciphertext = aes_encrypt(&plaintext, AesKey128(key));
//!
//! assert_eq!(ciphertext, [
//!     0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
//!     0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32
//! ]);
//! ```
//!
//! # References
//!
//! - [FIPS 197](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf): Advanced Encryption Standard (AES)
//! - [Wikipedia](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard): Advanced Encryption Standard
//!
//! # Security Warning
//!
//! This library has not been reviewed by cryptographers. It is not suitable for security-critical applications.
//!
//! Use this library at your own risk.
//!

const AES_WORD_SIZE: usize = 4;
const AES_BLOCK_SIZE: usize = 16;
const AES_NUM_BLOCK_WORDS: usize = AES_BLOCK_SIZE / AES_WORD_SIZE;

type Byte = u8;
type Word = u32;

type AesWord = [Byte; AES_WORD_SIZE];

/// Precalculated values for x to the power of 2 in Rijndaels galois field.
/// Used as 'RCON' during the key expansion.
///
/// The values are calculated by multiplying the previous value by 2 in the
/// galois field, and then reducing the result modulo 0x11b.
const RCON: [Word; 256] = [
    // 0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
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

/// Rijndael S-box Substitution table.
///
/// This table is used to substitute each byte of the state matrix in the
/// subBytes step of the encryption process. It is also used in the key
/// expansion process to generate the round keys.
const SBOX: [Byte; 256] = [
    // 0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

/// Inverse Rijndael S-box Substitution table.
///
/// This table is used to substitute each byte of the state matrix in the
/// subBytes step of the decryption process.
///
/// The inverse S-box is calculated by taking the multiplicative inverse of
/// each element in the S-box.
const INV_SBOX: [Byte; 256] = [
    // 0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
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

#[rustfmt::skip]
/// Multiplication table for the Galois Field (GF).
///
/// The table is used to multiply two numbers in the Galois Field.
///
/// The table is a 16x256 matrix, where each row represents the multiplication
/// of a number with the corresponding row number.
///
/// The table is used in the MixColumns step of the encryption process.
///
/// The table is generated by multiplying the row number with the column number
/// in the Galois Field.
const GF_MUL_TABLE: [[Byte; 256]; 16] = [
    /* 0 */ [0u8; 256],
    /* 1 */ 
    [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 
        0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 
        0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 
        0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 
        0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 
        0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 
        0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 
        0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 
        0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 
        0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 
        0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 
        0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
    ],
    /* 2 */ 
    [
        0x00, 0x02, 0x04, 0x06, 0x08, 0x0a, 0x0c, 0x0e, 0x10, 0x12, 0x14, 0x16, 0x18, 0x1a, 0x1c, 0x1e,
        0x20, 0x22, 0x24, 0x26, 0x28, 0x2a, 0x2c, 0x2e, 0x30, 0x32, 0x34, 0x36, 0x38, 0x3a, 0x3c, 0x3e,
        0x40, 0x42, 0x44, 0x46, 0x48, 0x4a, 0x4c, 0x4e, 0x50, 0x52, 0x54, 0x56, 0x58, 0x5a, 0x5c, 0x5e,
        0x60, 0x62, 0x64, 0x66, 0x68, 0x6a, 0x6c, 0x6e, 0x70, 0x72, 0x74, 0x76, 0x78, 0x7a, 0x7c, 0x7e,
        0x80, 0x82, 0x84, 0x86, 0x88, 0x8a, 0x8c, 0x8e, 0x90, 0x92, 0x94, 0x96, 0x98, 0x9a, 0x9c, 0x9e,
        0xa0, 0xa2, 0xa4, 0xa6, 0xa8, 0xaa, 0xac, 0xae, 0xb0, 0xb2, 0xb4, 0xb6, 0xb8, 0xba, 0xbc, 0xbe,
        0xc0, 0xc2, 0xc4, 0xc6, 0xc8, 0xca, 0xcc, 0xce, 0xd0, 0xd2, 0xd4, 0xd6, 0xd8, 0xda, 0xdc, 0xde,
        0xe0, 0xe2, 0xe4, 0xe6, 0xe8, 0xea, 0xec, 0xee, 0xf0, 0xf2, 0xf4, 0xf6, 0xf8, 0xfa, 0xfc, 0xfe,
        0x1b, 0x19, 0x1f, 0x1d, 0x13, 0x11, 0x17, 0x15, 0x0b, 0x09, 0x0f, 0x0d, 0x03, 0x01, 0x07, 0x05,
        0x3b, 0x39, 0x3f, 0x3d, 0x33, 0x31, 0x37, 0x35, 0x2b, 0x29, 0x2f, 0x2d, 0x23, 0x21, 0x27, 0x25,
        0x5b, 0x59, 0x5f, 0x5d, 0x53, 0x51, 0x57, 0x55, 0x4b, 0x49, 0x4f, 0x4d, 0x43, 0x41, 0x47, 0x45,
        0x7b, 0x79, 0x7f, 0x7d, 0x73, 0x71, 0x77, 0x75, 0x6b, 0x69, 0x6f, 0x6d, 0x63, 0x61, 0x67, 0x65,
        0x9b, 0x99, 0x9f, 0x9d, 0x93, 0x91, 0x97, 0x95, 0x8b, 0x89, 0x8f, 0x8d, 0x83, 0x81, 0x87, 0x85,
        0xbb, 0xb9, 0xbf, 0xbd, 0xb3, 0xb1, 0xb7, 0xb5, 0xab, 0xa9, 0xaf, 0xad, 0xa3, 0xa1, 0xa7, 0xa5,
        0xdb, 0xd9, 0xdf, 0xdd, 0xd3, 0xd1, 0xd7, 0xd5, 0xcb, 0xc9, 0xcf, 0xcd, 0xc3, 0xc1, 0xc7, 0xc5,
        0xfb, 0xf9, 0xff, 0xfd, 0xf3, 0xf1, 0xf7, 0xf5, 0xeb, 0xe9, 0xef, 0xed, 0xe3, 0xe1, 0xe7, 0xe5
    ],
    /* 3 */ 
    [
        0x00, 0x03, 0x06, 0x05, 0x0c, 0x0f, 0x0a, 0x09, 0x18, 0x1b, 0x1e, 0x1d, 0x14, 0x17, 0x12, 0x11,
        0x30, 0x33, 0x36, 0x35, 0x3c, 0x3f, 0x3a, 0x39, 0x28, 0x2b, 0x2e, 0x2d, 0x24, 0x27, 0x22, 0x21,
        0x60, 0x63, 0x66, 0x65, 0x6c, 0x6f, 0x6a, 0x69, 0x78, 0x7b, 0x7e, 0x7d, 0x74, 0x77, 0x72, 0x71,
        0x50, 0x53, 0x56, 0x55, 0x5c, 0x5f, 0x5a, 0x59, 0x48, 0x4b, 0x4e, 0x4d, 0x44, 0x47, 0x42, 0x41,
        0xc0, 0xc3, 0xc6, 0xc5, 0xcc, 0xcf, 0xca, 0xc9, 0xd8, 0xdb, 0xde, 0xdd, 0xd4, 0xd7, 0xd2, 0xd1,
        0xf0, 0xf3, 0xf6, 0xf5, 0xfc, 0xff, 0xfa, 0xf9, 0xe8, 0xeb, 0xee, 0xed, 0xe4, 0xe7, 0xe2, 0xe1,
        0xa0, 0xa3, 0xa6, 0xa5, 0xac, 0xaf, 0xaa, 0xa9, 0xb8, 0xbb, 0xbe, 0xbd, 0xb4, 0xb7, 0xb2, 0xb1,
        0x90, 0x93, 0x96, 0x95, 0x9c, 0x9f, 0x9a, 0x99, 0x88, 0x8b, 0x8e, 0x8d, 0x84, 0x87, 0x82, 0x81,
        0x9b, 0x98, 0x9d, 0x9e, 0x97, 0x94, 0x91, 0x92, 0x83, 0x80, 0x85, 0x86, 0x8f, 0x8c, 0x89, 0x8a,
        0xab, 0xa8, 0xad, 0xae, 0xa7, 0xa4, 0xa1, 0xa2, 0xb3, 0xb0, 0xb5, 0xb6, 0xbf, 0xbc, 0xb9, 0xba,
        0xfb, 0xf8, 0xfd, 0xfe, 0xf7, 0xf4, 0xf1, 0xf2, 0xe3, 0xe0, 0xe5, 0xe6, 0xef, 0xec, 0xe9, 0xea,
        0xcb, 0xc8, 0xcd, 0xce, 0xc7, 0xc4, 0xc1, 0xc2, 0xd3, 0xd0, 0xd5, 0xd6, 0xdf, 0xdc, 0xd9, 0xda,
        0x5b, 0x58, 0x5d, 0x5e, 0x57, 0x54, 0x51, 0x52, 0x43, 0x40, 0x45, 0x46, 0x4f, 0x4c, 0x49, 0x4a,
        0x6b, 0x68, 0x6d, 0x6e, 0x67, 0x64, 0x61, 0x62, 0x73, 0x70, 0x75, 0x76, 0x7f, 0x7c, 0x79, 0x7a,
        0x3b, 0x38, 0x3d, 0x3e, 0x37, 0x34, 0x31, 0x32, 0x23, 0x20, 0x25, 0x26, 0x2f, 0x2c, 0x29, 0x2a,
        0x0b, 0x08, 0x0d, 0x0e, 0x07, 0x04, 0x01, 0x02, 0x13, 0x10, 0x15, 0x16, 0x1f, 0x1c, 0x19, 0x1a,
    ],
    /* 4 */ [0u8; 256],
    /* 5 */ [0u8; 256],
    /* 6 */ [0u8; 256],
    /* 7 */ [0u8; 256],
    /* 8 */ [0u8; 256],
    /* 9 */ 
    [
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
    ],
    /* A */ [0u8; 256],
    /* B */ 
    [
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
    ],
    /* C */ [0u8; 256],
    /* D */ 
    [
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
        0xdc, 0xd1, 0xc6, 0xcb, 0xe8, 0xe5, 0xf2, 0xff, 0xb4, 0xb9, 0xae, 0xa3, 0x80, 0x8d, 0x9a, 0x97
    ],
    /* E */ 
    [
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
        0xd7, 0xd9, 0xcb, 0xc5, 0xef, 0xe1, 0xf3, 0xfd, 0xa7, 0xa9, 0xbb, 0xb5, 0x9f, 0x91, 0x83, 0x8d
    ],
    /* F */ [0u8; 256],
];

/// AesKey represents an AES key of 128, 192, or 256 bits.
/// The key is represented as an array of bytes.
/// The key size determines the number of rounds in the AES algorithm.
/// The key size also determines the number of words in the key.
pub enum AesKey {
    /// AES 128-bit key, 16 bytes
    AesKey128([Byte; 16]),
    /// AES 192-bit key, 24 bytes
    AesKey192([Byte; 24]),
    /// AES 256-bit key, 32 bytes
    AesKey256([Byte; 32]),
}

/// AesMode represents the AES mode of operation. Either converting plaintext to
/// encrypted text (Encryption) or converting encrypted text to plaintext (Decryption).
#[derive(Clone, Copy)]
enum AesMode {
    /// Encryption mode
    /// This mode is used to convert plaintext to encrypted text.
    Encryption,
    /// Decryption mode
    /// This mode is used to convert encrypted text to plaintext.
    Decryption,
}

/// aes_encrypt encrypts the given plaintext using the given AES key.
/// The plaintext is padded to the AES block size using PKCS7 padding.
/// The key must be 128, 192, or 256 bits.
///
/// # Arguments
///
/// `plain_text` - The plaintext to encrypt
/// `key` - The AES key to use for encryption
///
/// # Returns
///
/// The encrypted text
///
/// # Panics
///
/// This function will not panic.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::ciphers::{AesKey::AesKey128, aes_encrypt, aes_decrypt};
/// use std::str;
///
/// let key = [
///     0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
///     0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
/// ];
///
/// let plain_text = b"Hello, world!";
/// let cipher_text = aes_encrypt(plain_text, AesKey128(key));
///
/// let round_trip = aes_decrypt(&cipher_text, AesKey128(key));
///
/// // Convert the round trip back to a string since
/// // the encryption procces may have added '0' padding to the plaintext.
/// let round_trip_str = str::from_utf8(&round_trip).unwrap().trim_end_matches(char::from(0));
///
/// assert_eq!(plain_text, round_trip_str.as_bytes());
/// ```
///
pub fn aes_encrypt(plain_text: &[Byte], key: AesKey) -> Vec<Byte> {
    let (key, num_rounds) = match key {
        AesKey::AesKey128(key) => (Vec::from(key), 10),
        AesKey::AesKey192(key) => (Vec::from(key), 12),
        AesKey::AesKey256(key) => (Vec::from(key), 14),
    };

    let round_keys = key_expansion(&key, num_rounds);
    let mut data = padding::<Byte>(plain_text, AES_BLOCK_SIZE);

    let round_key = &round_keys[0..AES_BLOCK_SIZE];
    add_round_key(&mut data, round_key);

    for round in 1..num_rounds {
        sub_bytes_blocks(&mut data, AesMode::Encryption);
        shift_rows_blocks(&mut data, AesMode::Encryption);
        mix_column_blocks(&mut data, AesMode::Encryption);
        let round_key = &round_keys[round * AES_BLOCK_SIZE..(round + 1) * AES_BLOCK_SIZE];
        add_round_key(&mut data, round_key);
    }

    sub_bytes_blocks(&mut data, AesMode::Encryption);
    shift_rows_blocks(&mut data, AesMode::Encryption);
    let round_key = &round_keys[num_rounds * AES_BLOCK_SIZE..(num_rounds + 1) * AES_BLOCK_SIZE];
    add_round_key(&mut data, round_key);

    data
}

/// aes_decrypt decrypts the given cipher text using the given AES key.
/// The key must be 128, 192, or 256 bits.
///
/// # Arguments
///
/// `cipher_text` - The cipher text to decrypt
/// `key` - The AES key to use for decryption
///
/// # Returns
///
/// The decrypted text in bytes. This may be padded with '0' bytes.
///
/// # Panics
///
/// This function will not panic.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::ciphers::{AesKey::AesKey128, aes_encrypt, aes_decrypt};
/// use std::str;
///
/// let key = [
///    0xc3, 0x50, 0x30, 0xb5, 0x84, 0xed, 0x31, 0xe1,
///    0x7e, 0x7e, 0xe4, 0x0b, 0x01, 0x23, 0x5b, 0xf9,
/// ];
///
/// let plain_text = b"All around the world!";
/// let cipher_text = aes_encrypt(plain_text, AesKey128(key));
///
/// let round_trip = aes_decrypt(&cipher_text, AesKey128(key));
///
/// // Convert the round trip back to a string since
/// // the encryption procces may have added '0' padding to the plaintext.
/// let round_trip_str = str::from_utf8(&round_trip).unwrap().trim_end_matches(char::from(0));
///
/// assert_eq!(plain_text, round_trip_str.as_bytes());
/// ```
///
pub fn aes_decrypt(cipher_text: &[Byte], key: AesKey) -> Vec<Byte> {
    let (key, num_rounds) = match key {
        AesKey::AesKey128(key) => (Vec::from(key), 10),
        AesKey::AesKey192(key) => (Vec::from(key), 12),
        AesKey::AesKey256(key) => (Vec::from(key), 14),
    };

    let round_keys = key_expansion(&key, num_rounds);
    let mut data = padding::<Byte>(cipher_text, AES_BLOCK_SIZE);

    let round_key = &round_keys[num_rounds * AES_BLOCK_SIZE..(num_rounds + 1) * AES_BLOCK_SIZE];
    add_round_key(&mut data, round_key);
    shift_rows_blocks(&mut data, AesMode::Decryption);
    sub_bytes_blocks(&mut data, AesMode::Decryption);

    for round in (1..num_rounds).rev() {
        let round_key = &round_keys[round * AES_BLOCK_SIZE..(round + 1) * AES_BLOCK_SIZE];
        add_round_key(&mut data, round_key);
        mix_column_blocks(&mut data, AesMode::Decryption);
        shift_rows_blocks(&mut data, AesMode::Decryption);
        sub_bytes_blocks(&mut data, AesMode::Decryption);
    }

    let round_key = &round_keys[0..AES_BLOCK_SIZE];
    add_round_key(&mut data, round_key);

    data
}

/// key_expansion expands the given initial key into a key schedule.
/// The key schedule is used to generate round keys for each round of the AES algorithm.
/// The key schedule is generated using the Rijndael key schedule algorithm.
///
/// # Arguments
/// `init_key` - The initial key to expand
/// `num_rounds` - The number of rounds in the AES algorithm
///
/// # Returns
///
/// The key schedule as a vector of bytes
///
fn key_expansion(init_key: &[Byte], num_rounds: usize) -> Vec<Byte> {
    let nr = num_rounds;
    // number of words in initial key
    let nk = init_key.len() / AES_WORD_SIZE;
    let nb = AES_NUM_BLOCK_WORDS;

    let key = init_key
        .chunks(AES_WORD_SIZE)
        .map(bytes_to_word)
        .collect::<Vec<Word>>();
    let mut key = padding::<Word>(&key, nk * (nr + 1));

    for i in nk..nb * (nr + 1) {
        let mut temp_word = key[i - 1];
        if i % nk == 0 {
            temp_word = sub_word(rot_word(temp_word), AesMode::Encryption) ^ RCON[i / nk];
        } else if nk > 6 && i % nk == 4 {
            temp_word = sub_word(temp_word, AesMode::Encryption);
        }
        key[i] = key[i - nk] ^ temp_word;
    }

    key.iter()
        .map(|&w| word_to_bytes(w))
        .collect::<Vec<AesWord>>()
        .concat()
}

/// add_round_key adds the given round key to the given data.
/// The data is modified in place.
/// The round key must be the same size as the data.
///
/// # Arguments
///
/// `data` - The data to add the round key to
/// `round_key` - The round key to add to the data
///
fn add_round_key(data: &mut [Byte], round_key: &[Byte]) {
    assert!(data.len() % AES_BLOCK_SIZE == 0 && round_key.len() == AES_BLOCK_SIZE);
    let num_blocks = data.len() / AES_BLOCK_SIZE;
    data.iter_mut()
        .zip(round_key.repeat(num_blocks))
        .for_each(|(s, k)| *s ^= k);
}

/// sub_bytes_blocks applies the AES S-Box to each block in the given data.
/// The data is modified in place.
///
/// # Arguments
///
/// `data` - The data to apply the S-Box to
/// `mode` - The AES mode to use
///
fn sub_bytes_blocks(data: &mut [Byte], mode: AesMode) {
    for block in data.chunks_mut(AES_BLOCK_SIZE) {
        sub_bytes(block, mode);
    }
}

/// shift_rows_blocks applies the AES ShiftRows operation to each block in the given data.
/// The data is modified in place.
///
/// # Arguments
///
/// `data` - The data to apply the ShiftRows operation to
/// `mode` - The AES mode to use
///
fn shift_rows_blocks(blocks: &mut [Byte], mode: AesMode) {
    for block in blocks.chunks_mut(AES_BLOCK_SIZE) {
        transpose_block(block);
        shift_rows(block, mode);
        transpose_block(block);
    }
}

/// mix_column_blocks applies the AES MixColumns operation to each block in the given data.
/// The data is modified in place.
///
/// # Arguments
///
/// `data` - The data to apply the MixColumns operation to
/// `mode` - The AES mode to use
///
fn mix_column_blocks(data: &mut [Byte], mode: AesMode) {
    for block in data.chunks_mut(AES_BLOCK_SIZE) {
        transpose_block(block);
        mix_column(block, mode);
        transpose_block(block);
    }
}

/// padding pads the given data to the given block size using the default value of the data type.
///
/// # Arguments
///
/// `data` - The data to pad
/// `block_size` - The block size to pad the data to
///
/// # Returns
///
/// The padded data
///
fn padding<T: Clone + Default>(data: &[T], block_size: usize) -> Vec<T> {
    if data.len() % block_size == 0 {
        Vec::from(data)
    } else {
        let num_blocks = data.len() / block_size + 1;
        let mut padded = Vec::from(data);
        padded.append(&mut vec![
            T::default();
            num_blocks * block_size - data.len()
        ]);
        padded
    }
}

/// sub_word applies the AES S-Box/InvS-Box to the given word.
///
/// # Arguments
///
/// `word` - The word to apply the S-Box/InvS-Box to
/// `mode` - The AES mode to use
///
/// # Returns
///
/// The word with the S-Box/InvS-Box applied
///
fn sub_word(word: Word, mode: AesMode) -> Word {
    let mut bytes = word_to_bytes(word);
    sub_bytes(&mut bytes, mode);
    bytes_to_word(&bytes)
}

/// sub_bytes applies the AES S-Box/InvS-Box to the given data.
/// The data is modified in place.
///
/// # Arguments
///
/// `data` - The data to apply the S-Box/InvS-Box to
/// `mode` - The AES mode to use
///
fn sub_bytes(data: &mut [Byte], mode: AesMode) {
    let sbox = match mode {
        AesMode::Encryption => &SBOX,
        AesMode::Decryption => &INV_SBOX,
    };
    for data_byte in data {
        *data_byte = sbox[*data_byte as usize];
    }
}

/// shift_rows applies the AES ShiftRows/InvShiftRows operation to the given block.
/// The block is modified in place.
///
/// # Arguments
///
/// `block` - The block to apply the ShiftRows/InvShiftRows operation to
/// `mode` - The AES mode to use
///
fn shift_rows(block: &mut [Byte], mode: AesMode) {
    // skip the first row, index begin from 1
    for row in 1..4 {
        let mut row_word: AesWord = [0u8; 4];
        row_word.copy_from_slice(&block[row * 4..row * 4 + 4]);
        for col in 0..4 {
            block[row * 4 + col] = match mode {
                AesMode::Encryption => row_word[(col + row) % 4],
                AesMode::Decryption => row_word[(col + 4 - row) % 4],
            }
        }
    }
}

/// mix_column applies the AES MixColumns/InvMixColumns operation to the given block.
/// The block is modified in place.
///
/// # Arguments
///
/// `block` - The block to apply the MixColumns/InvMixColumns operation to
/// `mode` - The AES mode to use
///
fn mix_column(block: &mut [Byte], mode: AesMode) {
    let mix_col_mat = match mode {
        AesMode::Encryption => [
            [0x02, 0x03, 0x01, 0x01],
            [0x01, 0x02, 0x03, 0x01],
            [0x01, 0x01, 0x02, 0x03],
            [0x03, 0x01, 0x01, 0x02],
        ],
        AesMode::Decryption => [
            [0x0e, 0x0b, 0x0d, 0x09],
            [0x09, 0x0e, 0x0b, 0x0d],
            [0x0d, 0x09, 0x0e, 0x0b],
            [0x0b, 0x0d, 0x09, 0x0e],
        ],
    };

    for col in 0..4 {
        let col_word = block
            .iter()
            .zip(0..AES_BLOCK_SIZE)
            .filter_map(|(&x, i)| if i % 4 == col { Some(x) } else { None })
            .collect::<Vec<u8>>();
        for row in 0..4 {
            let mut word = 0;
            for i in 0..4 {
                word ^= GF_MUL_TABLE[mix_col_mat[row][i]][col_word[i] as usize] as Word
            }
            block[row * 4 + col] = word as Byte;
        }
    }
}

/// transpose_block transposes the given block in place.
///
/// # Arguments
///
/// `block` - The block to transpose
///
/// # Panics
///
/// This function will panic if the block is smaller than 16 bytes.
///
fn transpose_block(block: &mut [u8]) {
    let mut src_block = [0u8; AES_BLOCK_SIZE];
    src_block.copy_from_slice(block);
    for row in 0..4 {
        for col in 0..4 {
            block[row * 4 + col] = src_block[col * 4 + row];
        }
    }
}

/// bytes_to_word converts the given bytes to a word.
///
/// # Arguments
///
/// `bytes` - The bytes to convert to a word
///
/// # Returns
///
/// The word
///
/// # Panics
///
/// This function will panic if 'bytes' are not 4 bytes long.
///
fn bytes_to_word(bytes: &[Byte]) -> Word {
    assert!(bytes.len() == AES_WORD_SIZE);
    let mut word = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        word |= (byte as Word) << (8 * i);
    }
    word
}

/// word_to_bytes converts the given word to an AESWord byte array.
///
/// # Arguments
///
/// `word` - The word to convert to an AESWord byte array.
///
/// # Returns
///
/// The AESWord byte array
///
fn word_to_bytes(word: Word) -> AesWord {
    let mut bytes = [0; AES_WORD_SIZE];
    for (i, byte) in bytes.iter_mut().enumerate() {
        let bits_shift = 8 * i;
        *byte = ((word & (0xff << bits_shift)) >> bits_shift) as Byte;
    }
    bytes
}

/// rot_word rotates the given word to the left.
///
/// # Arguments
///
/// `word` - The word to rotate
///
/// # Returns
///
/// The rotated word
///
/// # Panics
///
/// This function will not panic.
///
fn rot_word(word: Word) -> Word {
    let mut bytes = word_to_bytes(word);
    let init = bytes[0];
    bytes[0] = bytes[1];
    bytes[1] = bytes[2];
    bytes[2] = bytes[3];
    bytes[3] = init;
    bytes_to_word(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_128() {
        let plain: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let cipher: [u8; 16] = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];
        let encrypted = aes_encrypt(&plain, AesKey::AesKey128(key));
        assert_eq!(cipher, encrypted[..]);
        let decrypted = aes_decrypt(&encrypted, AesKey::AesKey128(key));
        assert_eq!(plain, decrypted[..]);
    }

    #[test]
    fn test_aes_192() {
        let plain: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let key: [u8; 24] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ];
        let cipher: [u8; 16] = [
            0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d,
            0x71, 0x91,
        ];
        let encrypted = aes_encrypt(&plain, AesKey::AesKey192(key));
        assert_eq!(cipher, encrypted[..]);
        let decrypted = aes_decrypt(&encrypted, AesKey::AesKey192(key));
        assert_eq!(plain, decrypted[..]);
    }

    #[test]
    fn test_aes_256() {
        let plain: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let cipher: [u8; 16] = [
            0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49,
            0x60, 0x89,
        ];
        let encrypted = aes_encrypt(&plain, AesKey::AesKey256(key));
        assert_eq!(cipher, encrypted[..]);
        let decrypted = aes_decrypt(&encrypted, AesKey::AesKey256(key));
        assert_eq!(plain, decrypted[..]);
    }

    #[test]
    fn test_str() {
        let str = "Hello, cipher world!";
        let plain = str.as_bytes();
        let key = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let encrypted = aes_encrypt(plain, AesKey::AesKey128(key));
        let decrypted = aes_decrypt(&encrypted, AesKey::AesKey128(key));
        assert_eq!(
            str,
            String::from_utf8(decrypted).unwrap().trim_end_matches("\0")
        );
    }
}
