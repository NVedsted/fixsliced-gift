use crate::{swap_move, swap_move_single};
use crate::gift128::{Block, BLOCK_SIZE, State};

#[must_use]
pub fn pack(input: &Block) -> State {
    let mut s0 = ((input[6] as u32) << 24)
        | ((input[7] as u32) << 16)
        | ((input[14] as u32) << 8)
        | input[15] as u32;
    let mut s1 = ((input[4] as u32) << 24)
        | ((input[5] as u32) << 16)
        | ((input[12] as u32) << 8)
        | (input[13] as u32);
    let mut s2 = ((input[2] as u32) << 24)
        | ((input[3] as u32) << 16)
        | ((input[10] as u32) << 8)
        | (input[11] as u32);
    let mut s3 = ((input[0] as u32) << 24)
        | ((input[1] as u32) << 16)
        | ((input[8] as u32) << 8)
        | (input[9] as u32);

    s0 = swap_move_single(s0, 0x0a0a0a0a, 3);
    s0 = swap_move_single(s0, 0x00cc00cc, 6);
    s1 = swap_move_single(s1, 0x0a0a0a0a, 3);
    s1 = swap_move_single(s1, 0x00cc00cc, 6);
    s2 = swap_move_single(s2, 0x0a0a0a0a, 3);
    s2 = swap_move_single(s2, 0x00cc00cc, 6);
    s3 = swap_move_single(s3, 0x0a0a0a0a, 3);
    s3 = swap_move_single(s3, 0x00cc00cc, 6);

    (s0, s1) = swap_move(s0, s1, 0x000f000f, 4);
    (s0, s2) = swap_move(s0, s2, 0x000f000f, 8);
    (s0, s3) = swap_move(s0, s3, 0x000f000f, 12);
    (s1, s2) = swap_move(s1, s2, 0x00f000f0, 4);
    (s1, s3) = swap_move(s1, s3, 0x00f000f0, 8);
    (s2, s3) = swap_move(s2, s3, 0x0f000f00, 4);

    (s0, s1, s2, s3)
}

#[must_use]
pub fn unpack(state: State) -> Block {
    let (mut s0, mut s1, mut s2, mut s3) = state;

    // TODO: use macro for swap_move
    (s2, s3) = swap_move(s2, s3, 0x0f000f00, 4);
    (s1, s3) = swap_move(s1, s3, 0x00f000f0, 8);
    (s1, s2) = swap_move(s1, s2, 0x00f000f0, 4);
    (s0, s3) = swap_move(s0, s3, 0x000f000f, 12);
    (s0, s2) = swap_move(s0, s2, 0x000f000f, 8);
    (s0, s1) = swap_move(s0, s1, 0x000f000f, 4);

    s3 = swap_move_single(s3, 0x00cc00cc, 6);
    s3 = swap_move_single(s3, 0x0a0a0a0a, 3);
    s2 = swap_move_single(s2, 0x00cc00cc, 6);
    s2 = swap_move_single(s2, 0x0a0a0a0a, 3);
    s1 = swap_move_single(s1, 0x00cc00cc, 6);
    s1 = swap_move_single(s1, 0x0a0a0a0a, 3);
    s0 = swap_move_single(s0, 0x00cc00cc, 6);
    s0 = swap_move_single(s0, 0x0a0a0a0a, 3);
    [
        (s3 >> 24) as u8,
        ((s3 >> 16) & 0xff) as u8,
        (s2 >> 24) as u8,
        ((s2 >> 16) & 0xff) as u8,
        (s1 >> 24) as u8,
        ((s1 >> 16) & 0xff) as u8,
        (s0 >> 24) as u8,
        ((s0 >> 16) & 0xff) as u8,
        ((s3 >> 8) & 0xff) as u8,
        (s3 & 0xff) as u8,
        ((s2 >> 8) & 0xff) as u8,
        (s2 & 0xff) as u8,
        ((s1 >> 8) & 0xff) as u8,
        (s1 & 0xff) as u8,
        ((s0 >> 8) & 0xff) as u8,
        (s0 & 0xff) as u8,
    ]
}

pub fn bitsliced_pack(input: &Block) -> State {
    let s0 = u32::from_le_bytes([input[0], input[1], input[2], input[3]]).swap_bytes();
    let s1 = u32::from_le_bytes([input[4], input[5], input[6], input[7]]).swap_bytes();
    let s2 = u32::from_le_bytes([input[8], input[9], input[10], input[11]]).swap_bytes();
    let s3 = u32::from_le_bytes([input[12], input[13], input[14], input[15]]).swap_bytes();

    (s0, s1, s2, s3)
}

pub fn bitsliced_unpack(state: State) -> Block {
    let (s0, s1, s2, s3) = state;
    let mut block = [0; BLOCK_SIZE];
    block[..4].copy_from_slice(&s0.to_be_bytes());
    block[4..8].copy_from_slice(&s1.to_be_bytes());
    block[8..12].copy_from_slice(&s2.to_be_bytes());
    block[12..16].copy_from_slice(&s3.to_be_bytes());
    block
}

#[cfg(test)]
mod tests {
    use crate::gift128::packing::*;

    #[test]
    fn test_pack_unpack() {
        let input = [
            0xe3, 0x9c, 0x14, 0x1f, 0xa5, 0x7d, 0xba, 0x43,
            0xf0, 0x8a, 0x85, 0xb6, 0xa9, 0x1f, 0x86, 0xc1
        ];
        let s = pack(&input);
        let output = unpack(s);
        assert_eq!(input, output);
    }
}
