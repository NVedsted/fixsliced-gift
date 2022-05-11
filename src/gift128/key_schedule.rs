use core::ops::{BitAnd, BitOr, Shl, Shr};

use crate::gift128::Key;
use crate::gift128::masking::BinaryMask;
use crate::gift128::rotate::Rotate;
use crate::gift128::rounds::ROUNDS;
use crate::swapmove::{swap_move_single, SwapMoveTraits};

// TODO: use tuples
pub type RoundKeys<T> = [T; ROUNDS * 2];

#[must_use]
fn key_update<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + Shl<usize, Output=T> + BitAnd<u32, Output=T> + BitOr<Output=T> {
    ((round_key >> 12) & 0x0000000f)
        | ((round_key & 0x00000fff) << 4)
        | ((round_key >> 2) & 0x3fff0000)
        | ((round_key & 0x00030000) << 14)
}

#[must_use]
fn rearrange_round_key0<T: SwapMoveTraits>(mut round_key: T) -> T {
    round_key = swap_move_single(round_key, 0x00550055, 9);
    round_key = swap_move_single(round_key, 0x000f000f, 12);
    round_key = swap_move_single(round_key, 0x00003333, 18);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key1<T: SwapMoveTraits>(mut round_key: T) -> T {
    round_key = swap_move_single(round_key, 0x11111111, 3);
    round_key = swap_move_single(round_key, 0x03030303, 6);
    round_key = swap_move_single(round_key, 0x000f000f, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key2<T: SwapMoveTraits>(mut round_key: T) -> T {
    round_key = swap_move_single(round_key, 0x0000aaaa, 15);
    round_key = swap_move_single(round_key, 0x00003333, 18);
    round_key = swap_move_single(round_key, 0x0000f0f0, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key3<T: SwapMoveTraits>(mut round_key: T) -> T {
    round_key = swap_move_single(round_key, 0x0a0a0a0a, 3);
    round_key = swap_move_single(round_key, 0x00cc00cc, 6);
    round_key = swap_move_single(round_key, 0x0000f0f0, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn key_triple_update_0<T>(round_key: T) -> T
    where T: Copy + BitAnd<u32, Output=T> + BitOr<Output=T> + Rotate {
    (round_key & 0x33333333).rotate_right(24) | (round_key & 0xcccccccc).rotate_right(16)
}

#[must_use]
fn key_double_update_1<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 4) & 0x0f000f00)
        | ((round_key & 0x0f000f00) << 4)
        | ((round_key >> 6) & 0x00030003)
        | ((round_key & 0x003f003f) << 2)
}

#[must_use]
fn key_triple_update_1<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 6) & 0x03000300)
        | ((round_key & 0x3f003f00) << 2)
        | ((round_key >> 5) & 0x00070007)
        | ((round_key & 0x001f001f) << 3)
}

#[must_use]
fn key_double_update_2<T>(round_key: T) -> T
    where T: Copy + BitAnd<u32, Output=T> + BitOr<Output=T> + Rotate {
    (round_key & 0xaaaaaaaa).rotate_right(24) | (round_key & 0x55555555).rotate_right(16)
}

#[must_use]
fn key_triple_update_2<T>(round_key: T) -> T
    where T: Copy + BitAnd<u32, Output=T> + BitOr<Output=T> + Rotate {
    (round_key & 0x55555555).rotate_right(24) | (round_key & 0xaaaaaaaa).rotate_right(20)
}

#[must_use]
fn key_double_update_3<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 2) & 0x03030303)
        | ((round_key & 0x03030303) << 2)
        | ((round_key >> 1) & 0x70707070)
        | ((round_key & 0x10101010) << 3)
}

#[must_use]
fn key_triple_update_3<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 18) & 0x00003030)
        | ((round_key & 0x01010101) << 3)
        | ((round_key >> 14) & 0x0000c0c0)
        | ((round_key & 0x0000e0e0) << 15)
        | ((round_key >> 1) & 0x07070707)
        | ((round_key & 0x00001010) << 19)
}

#[must_use]
fn key_double_update_4<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 4) & 0x0fff0000)
        | ((round_key & 0x000f0000) << 12)
        | ((round_key >> 8) & 0x000000ff)
        | ((round_key & 0x000000ff) << 8)
}

#[must_use]
fn key_triple_update_4<T>(round_key: T) -> T
    where T: Copy + Shr<usize, Output=T> + BitAnd<u32, Output=T> + Shl<usize, Output=T> + BitOr<Output=T> {
    ((round_key >> 6) & 0x03ff0000)
        | ((round_key & 0x003f0000) << 10)
        | ((round_key >> 4) & 0x00000fff)
        | ((round_key & 0x0000000f) << 12)
}

fn fill_round_keys<T>(round_keys: &mut [T; ROUNDS * 2])
    where T: SwapMoveTraits + BitOr<Output=T> + Rotate {
    for i in (0..16).step_by(2) {
        round_keys[i + 4] = round_keys[i + 1];
        round_keys[i + 5] = key_update(round_keys[i]);
    }

    for i in (0..20).step_by(10) {
        round_keys[i] = rearrange_round_key0(round_keys[i]);
        round_keys[i + 1] = rearrange_round_key0(round_keys[i + 1]);
        round_keys[i + 2] = rearrange_round_key1(round_keys[i + 2]);
        round_keys[i + 3] = rearrange_round_key1(round_keys[i + 3]);
        round_keys[i + 4] = rearrange_round_key2(round_keys[i + 4]);
        round_keys[i + 5] = rearrange_round_key2(round_keys[i + 5]);
        round_keys[i + 6] = rearrange_round_key3(round_keys[i + 6]);
        round_keys[i + 7] = rearrange_round_key3(round_keys[i + 7]);
    }

    for i in (20..80).step_by(10) {
        round_keys[i] = round_keys[i - 19];
        round_keys[i + 1] = key_triple_update_0(round_keys[i - 20]);
        round_keys[i + 2] = key_double_update_1(round_keys[i - 17]);
        round_keys[i + 3] = key_triple_update_1(round_keys[i - 18]);
        round_keys[i + 4] = key_double_update_2(round_keys[i - 15]);
        round_keys[i + 5] = key_triple_update_2(round_keys[i - 16]);
        round_keys[i + 6] = key_double_update_3(round_keys[i - 13]);
        round_keys[i + 7] = key_triple_update_3(round_keys[i - 14]);
        round_keys[i + 8] = key_double_update_4(round_keys[i - 11]);
        round_keys[i + 9] = key_triple_update_4(round_keys[i - 12]);
        round_keys[i] = swap_move_single(round_keys[i], 0x00003333, 16);
        round_keys[i] = swap_move_single(round_keys[i], 0x55554444, 1);
        round_keys[i + 1] = swap_move_single(round_keys[i + 1], 0x55551100, 1);
    }
}

#[must_use]
pub fn precompute_round_keys(key: &Key) -> RoundKeys<u32> {
    let mut round_keys = [0; ROUNDS * 2];
    round_keys[0] = u32::from_le_bytes([key[12], key[13], key[14], key[15]]).swap_bytes();
    round_keys[1] = u32::from_le_bytes([key[4], key[5], key[6], key[7]]).swap_bytes();
    round_keys[2] = u32::from_le_bytes([key[8], key[9], key[10], key[11]]).swap_bytes();
    round_keys[3] = u32::from_le_bytes([key[0], key[1], key[2], key[3]]).swap_bytes();

    fill_round_keys(&mut round_keys);

    round_keys
}

#[must_use]
pub fn precompute_masked_round_keys(key: &Key) -> RoundKeys<BinaryMask<u32>> {
    let mut round_keys = [BinaryMask::make_shares(0, 0); ROUNDS * 2];
    // TODO: figure out how to supply random
    let masks = (0x1d54f08eu32, 0x550aaf8cu32, 0xb3d27d46u32, 0x4aafa1b4u32);
    // TODO: swapping bytes likely leaks!
    round_keys[0] = BinaryMask::make_shares(u32::from_le_bytes([key[12], key[13], key[14], key[15]]).swap_bytes(), masks.0);
    round_keys[1] = BinaryMask::make_shares(u32::from_le_bytes([key[4], key[5], key[6], key[7]]).swap_bytes(), masks.1);
    round_keys[2] = BinaryMask::make_shares(u32::from_le_bytes([key[8], key[9], key[10], key[11]]).swap_bytes(), masks.2);
    round_keys[3] = BinaryMask::make_shares(u32::from_le_bytes([key[0], key[1], key[2], key[3]]).swap_bytes(), masks.3);

    fill_round_keys(&mut round_keys);

    round_keys
}

#[cfg(test)]
mod tests {
    use crate::gift128::key_schedule::{precompute_masked_round_keys, precompute_round_keys};

    #[test]
    fn test_masked_round_keys() {
        let key = [
            0xd0, 0xf5, 0xc5, 0x9a, 0x77, 0x00, 0xd3, 0xe7,
            0x99, 0x02, 0x8f, 0xa9, 0xf9, 0x0a, 0xd8, 0x37,
        ];
        let round_keys = precompute_round_keys(&key);
        let masked_rounds_keys = precompute_masked_round_keys(&key);

        for (masked, expected) in masked_rounds_keys.into_iter().zip(round_keys) {
            assert_eq!(masked.recover_shares(), expected);
        }
    }
}
