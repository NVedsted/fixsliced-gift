use crate::gift128::key_schedule::precompute_round_keys;
use crate::gift128::packing::{bitsliced_pack, bitsliced_unpack, pack, unpack};
use crate::gift128::rounds::{inv_rounds, rounds};

mod key_schedule;
mod packing;
mod rounds;
mod sbox;

const KEY_SIZE: usize = 16;
const BLOCK_SIZE: usize = 16;

type State = (u32, u32, u32, u32);
type Block = [u8; BLOCK_SIZE];
pub type Key = [u8; KEY_SIZE];

pub fn encrypt(plaintext: &[u8], key: &Key, ciphertext: &mut [u8]) {
    if plaintext.len() % BLOCK_SIZE != 0 {
        panic!("plaintext size is not a multiple of 16");
    }

    if plaintext.len() != ciphertext.len() {
        panic!("ciphertext size differs from plaintext size");
    }

    let round_keys = precompute_round_keys(key);
    for (i, chunk) in plaintext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let plaintext_block = chunk.try_into().expect("invalid chunk length");

        let state = rounds(pack(plaintext_block), &round_keys);

        ciphertext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&unpack(state));
    }
}

pub fn decrypt(ciphertext: &[u8], key: &Key, plaintext: &mut [u8]) {
    if ciphertext.len() % 16 != 0 {
        panic!("ciphertext size is not a multiple of 16");
    }

    if ciphertext.len() != plaintext.len() {
        panic!("plaintext size differs from ciphertext size");
    }

    let round_keys = precompute_round_keys(key);

    for (i, chunk) in ciphertext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let ciphertext_block = chunk.try_into().expect("invalid chunk length");

        let state = inv_rounds(pack(ciphertext_block), &round_keys);

        plaintext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&unpack(state));
    }
}

pub fn bitsliced_encrypt(plaintext: &[u8], key: &Key, ciphertext: &mut [u8]) {
    if plaintext.len() % BLOCK_SIZE != 0 {
        panic!("plaintext size is not a multiple of 16");
    }

    if plaintext.len() != ciphertext.len() {
        panic!("ciphertext size differs from plaintext size");
    }

    let round_keys = precompute_round_keys(key);
    for (i, chunk) in plaintext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let ciphertext_block = chunk.try_into().expect("invalid chunk length");

        let state = rounds(bitsliced_pack(&ciphertext_block), &round_keys);

        ciphertext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&bitsliced_unpack(state));
    }
}

pub fn bitsliced_decrypt(ciphertext: &[u8], key: &Key, plaintext: &mut [u8]) {
    if ciphertext.len() % 16 != 0 {
        panic!("ciphertext size is not a multiple of 16");
    }

    if ciphertext.len() != plaintext.len() {
        panic!("plaintext size differs from ciphertext size");
    }

    let round_keys = precompute_round_keys(key);

    for (i, chunk) in ciphertext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let ciphertext_block = chunk.try_into().expect("invalid chunk length");

        let state = inv_rounds(bitsliced_pack(ciphertext_block), &round_keys);

        plaintext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&bitsliced_unpack(state));
    }
}

#[cfg(test)]
mod tests {
    use crate::gift128::*;

    struct TestTriple {
        key: Key,
        plaintext: Block,
        ciphertext: Block,
    }

    const CASES: [TestTriple; 3] = [
        TestTriple {
            key: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            plaintext: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            ciphertext: [
                0xcd, 0x0b, 0xd7, 0x38, 0x38, 0x8a, 0xd3, 0xf6,
                0x68, 0xb1, 0x5a, 0x36, 0xce, 0xb6, 0xff, 0x92,
            ],
        },
        TestTriple {
            key: [
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            ],
            plaintext: [
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            ],
            ciphertext: [
                0x84, 0x22, 0x24, 0x1a, 0x6d, 0xbf, 0x5a, 0x93,
                0x46, 0xaf, 0x46, 0x84, 0x09, 0xee, 0x01, 0x52,
            ],
        },
        TestTriple {
            key: [
                0xd0, 0xf5, 0xc5, 0x9a, 0x77, 0x00, 0xd3, 0xe7,
                0x99, 0x02, 0x8f, 0xa9, 0xf9, 0x0a, 0xd8, 0x37,
            ],
            plaintext: [
                0xe3, 0x9c, 0x14, 0x1f, 0xa5, 0x7d, 0xba, 0x43,
                0xf0, 0x8a, 0x85, 0xb6, 0xa9, 0x1f, 0x86, 0xc1,
            ],
            ciphertext: [
                0x13, 0xed, 0xe6, 0x7c, 0xbd, 0xcc, 0x3d, 0xbf,
                0x40, 0x0a, 0x62, 0xd6, 0x97, 0x72, 0x65, 0xea,
            ],
        },
    ];

    #[test]
    fn test_encrypt() {
        for case in &CASES {
            let mut ciphertext = [0; BLOCK_SIZE];
            encrypt(&case.plaintext, &case.key, &mut ciphertext);
            assert_eq!(ciphertext, case.ciphertext);
        }
    }

    #[test]
    fn test_decrypt() {
        for case in &CASES {
            let mut plaintext = [0; BLOCK_SIZE];
            decrypt(&case.ciphertext, &case.key, &mut plaintext);
            assert_eq!(plaintext, case.plaintext);
        }
    }

    #[test]
    fn test_bitsliced_encrypt_decrypt() {
        for case in &CASES {
            let mut ciphertext = [0; BLOCK_SIZE];
            bitsliced_encrypt(&case.plaintext, &case.key, &mut ciphertext);

            let mut plaintext = [0; BLOCK_SIZE];
            bitsliced_decrypt(&ciphertext, &case.key, &mut plaintext);

            assert_eq!(case.plaintext, plaintext);
        }
    }
}
