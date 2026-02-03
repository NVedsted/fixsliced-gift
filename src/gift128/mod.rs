use crate::gift128::key_schedule::{
    precompute_masked_round_keys, precompute_round_keys, RoundKeys,
};
use crate::gift128::masking::BinaryMask;
use crate::gift128::packing::{
    bitsliced_pack, bitsliced_unpack, masked_pack, masked_unpack, pack, unpack,
};
use crate::gift128::rounds::{inv_rounds, rounds};

pub mod key_schedule;
mod masking;
mod packing;
mod rounds;
mod sbox;
mod traits;

const KEY_SIZE: usize = 16;
const BLOCK_SIZE: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
struct State<T>(T, T, T, T);

pub type Block = [u8; BLOCK_SIZE];
pub type MaskedBlock = [BinaryMask<u8>; BLOCK_SIZE];
pub type Key = [u8; KEY_SIZE];
pub type MaskedKey = [BinaryMask<u8>; KEY_SIZE];

pub fn encrypt(plaintext: &[u8], key: &Key, ciphertext: &mut [u8]) {
    if !plaintext.len().is_multiple_of(BLOCK_SIZE) {
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
    if !ciphertext.len().is_multiple_of(16) {
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

#[must_use]
pub fn mask_block(block: &Block, masks: &[u8; BLOCK_SIZE]) -> MaskedBlock {
    let mut masked_block = [Default::default(); BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        masked_block[i] = BinaryMask::make_shares(block[i], masks[i]);
    }
    masked_block
}

#[must_use]
pub fn unmask_block(masked_block: &MaskedBlock) -> Block {
    let mut block = [0; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        block[i] = masked_block[i].recover_shares();
    }
    block
}

pub fn encrypt_masked(
    plaintext: &[BinaryMask<u8>],
    key: &MaskedKey,
    ciphertext: &mut [BinaryMask<u8>],
) {
    if !plaintext.len().is_multiple_of(BLOCK_SIZE) {
        panic!("plaintext size is not a multiple of 16");
    }

    if plaintext.len() != ciphertext.len() {
        panic!("ciphertext size differs from plaintext size");
    }

    let round_keys = precompute_masked_round_keys(key);

    for (i, chunk) in plaintext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let plaintext_block = chunk.try_into().expect("invalid chunk length");
        let state = rounds(masked_pack(plaintext_block), &round_keys);
        ciphertext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&masked_unpack(state));
    }
}

pub fn decrypt_masked(
    ciphertext: &[BinaryMask<u8>],
    key: &MaskedKey,
    plaintext: &mut [BinaryMask<u8>],
) {
    if !ciphertext.len().is_multiple_of(16) {
        panic!("ciphertext size is not a multiple of 16");
    }

    if ciphertext.len() != plaintext.len() {
        panic!("plaintext size differs from ciphertext size");
    }

    let round_keys = precompute_masked_round_keys(key);

    for (i, chunk) in ciphertext.chunks(BLOCK_SIZE).enumerate() {
        // TODO: annoying runtime check
        let ciphertext_block = chunk.try_into().expect("invalid chunk length");
        let state = inv_rounds(masked_pack(ciphertext_block), &round_keys);
        plaintext[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&masked_unpack(state));
    }
}

#[must_use]
pub fn bitsliced_encrypt_block(plaintext: &Block, round_keys: &RoundKeys<u32>) -> Block {
    let initial_state = bitsliced_pack(plaintext);
    let final_state = rounds(initial_state, round_keys);
    bitsliced_unpack(final_state)
}

#[must_use]
pub fn bitsliced_decrypt_block(ciphertext: &Block, round_keys: &RoundKeys<u32>) -> Block {
    let initial_state = bitsliced_pack(ciphertext);
    let final_state = inv_rounds(initial_state, round_keys);
    bitsliced_unpack(final_state)
}

#[must_use]
pub fn bitsliced_masked_encrypt_block(
    plaintext: &Block,
    mask: (u32, u32, u32, u32),
    round_keys: &RoundKeys<BinaryMask<u32>>,
) -> Block {
    let initial_state = bitsliced_pack(plaintext);
    let initial_masked_state = initial_state.make_shares(mask);
    let final_masked_state = rounds(initial_masked_state, round_keys);
    let final_state = final_masked_state.recover_shares();
    bitsliced_unpack(final_state)
}

#[must_use]
pub fn bitsliced_masked_decrypt_block(
    ciphertext: &Block,
    mask: (u32, u32, u32, u32),
    round_keys: &RoundKeys<BinaryMask<u32>>,
) -> Block {
    let initial_state = bitsliced_pack(ciphertext);
    let initial_masked_state = initial_state.make_shares(mask);
    let final_masked_state = inv_rounds(initial_masked_state, round_keys);
    let final_state = final_masked_state.recover_shares();
    bitsliced_unpack(final_state)
}

#[cfg(test)]
mod tests {
    use crate::gift128::key_schedule::mask_key;
    use crate::gift128::*;

    struct TestTriple {
        key: Key,
        plaintext: Block,
        ciphertext: Block,
    }

    const CASES: [TestTriple; 3] = [
        TestTriple {
            key: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            plaintext: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            ciphertext: [
                0xcd, 0x0b, 0xd7, 0x38, 0x38, 0x8a, 0xd3, 0xf6, 0x68, 0xb1, 0x5a, 0x36, 0xce, 0xb6,
                0xff, 0x92,
            ],
        },
        TestTriple {
            key: [
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
                0x32, 0x10,
            ],
            plaintext: [
                0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
                0x32, 0x10,
            ],
            ciphertext: [
                0x84, 0x22, 0x24, 0x1a, 0x6d, 0xbf, 0x5a, 0x93, 0x46, 0xaf, 0x46, 0x84, 0x09, 0xee,
                0x01, 0x52,
            ],
        },
        TestTriple {
            key: [
                0xd0, 0xf5, 0xc5, 0x9a, 0x77, 0x00, 0xd3, 0xe7, 0x99, 0x02, 0x8f, 0xa9, 0xf9, 0x0a,
                0xd8, 0x37,
            ],
            plaintext: [
                0xe3, 0x9c, 0x14, 0x1f, 0xa5, 0x7d, 0xba, 0x43, 0xf0, 0x8a, 0x85, 0xb6, 0xa9, 0x1f,
                0x86, 0xc1,
            ],
            ciphertext: [
                0x13, 0xed, 0xe6, 0x7c, 0xbd, 0xcc, 0x3d, 0xbf, 0x40, 0x0a, 0x62, 0xd6, 0x97, 0x72,
                0x65, 0xea,
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
    fn test_masked_encrypt() {
        let plaintext_masks = [
            0x1d, 0x54, 0xf0, 0x8e, 0x55, 0x0a, 0xaf, 0x8c, 0xb3, 0xd2, 0x7d, 0x46, 0x4a, 0xaf,
            0xa1, 0xb4,
        ];
        let key_masks = [
            0x1d, 0x54, 0xf0, 0xae, 0x54, 0x0a, 0xaf, 0x8c, 0xb3, 0xd7, 0x7d, 0x46, 0x4a, 0xac,
            0xa1, 0xb4,
        ];
        for case in &CASES {
            let masked_key = mask_key(&case.key, &key_masks);
            let mut masked_ciphertext = [Default::default(); BLOCK_SIZE];
            let masked_plaintext = mask_block(&case.plaintext, &plaintext_masks);
            encrypt_masked(&masked_plaintext, &masked_key, &mut masked_ciphertext);
            let ciphertext = unmask_block(&masked_ciphertext);
            assert_eq!(ciphertext, case.ciphertext);
        }
    }

    #[test]
    fn test_masked_decrypt() {
        let ciphertext_masks = [
            0x1d, 0x54, 0xf0, 0x8e, 0x55, 0x0a, 0xaf, 0x8c, 0xb3, 0xd2, 0x7d, 0x46, 0x4a, 0xaf,
            0xa1, 0xb4,
        ];
        let key_masks = [
            0x1d, 0x54, 0xf0, 0xae, 0x54, 0x0a, 0xaf, 0x8c, 0xb3, 0xd7, 0x7d, 0x46, 0x4a, 0xac,
            0xa1, 0xb4,
        ];
        for case in &CASES {
            let masked_key = mask_key(&case.key, &key_masks);
            let masked_ciphertext = mask_block(&case.ciphertext, &ciphertext_masks);
            let mut masked_plaintext = [Default::default(); BLOCK_SIZE];
            decrypt_masked(&masked_ciphertext, &masked_key, &mut masked_plaintext);
            let plaintext = unmask_block(&masked_plaintext);
            assert_eq!(plaintext, case.plaintext);
        }
    }

    #[test]
    fn test_bitsliced_block_encrypt_decrypt() {
        for case in &CASES {
            let round_keys = precompute_round_keys(&case.key);
            let ciphertext = bitsliced_encrypt_block(&case.plaintext, &round_keys);
            let plaintext = bitsliced_decrypt_block(&ciphertext, &round_keys);

            assert_eq!(case.plaintext, plaintext);
        }
    }

    #[test]
    fn test_masked_bitsliced_block_encrypt_decrypt() {
        let encrypt_masks = (0x1d54f08eu32, 0x550aaf8cu32, 0xb3d27d46u32, 0x4aafa1b4u32);
        let decrypt_masks = (0x1d56608eu32, 0x550aaf8cu32, 0xb3d27d98u32, 0x4fffa1b4u32);
        let key_masks = [
            0x1d, 0x54, 0xf0, 0xae, 0x54, 0x0a, 0xaf, 0x8c, 0xb3, 0xd7, 0x7d, 0x46, 0x4a, 0xac,
            0xa1, 0xb4,
        ];
        for case in &CASES {
            let masked_key = mask_key(&case.key, &key_masks);
            let round_keys = precompute_masked_round_keys(&masked_key);
            let ciphertext =
                bitsliced_masked_encrypt_block(&case.plaintext, encrypt_masks, &round_keys);
            let plaintext = bitsliced_masked_decrypt_block(&ciphertext, decrypt_masks, &round_keys);

            assert_eq!(case.plaintext, plaintext);
        }
    }

    #[test]
    fn test_mask_block() {
        let block_masks = [
            0x1d, 0x54, 0xf0, 0xae, 0x54, 0x0a, 0xaf, 0x8c, 0xb3, 0xd7, 0x7d, 0x46, 0x4a, 0xac,
            0xa1, 0xb4,
        ];

        for case in &CASES {
            let masked_block = mask_block(&case.plaintext, &block_masks);
            let unmasked_block = unmask_block(&masked_block);
            assert_eq!(case.plaintext, unmasked_block);
        }
    }
}
