use crate::gift128::Key;
use crate::gift128::rounds::ROUNDS;
use crate::swapmove::swap_move_single;

// TODO: use tuples
pub type RoundKeys = [u32; ROUNDS * 2];

#[must_use]
fn key_update(round_key: u32) -> u32 {
    ((round_key >> 12) & 0x0000000f)
        | ((round_key & 0x00000fff) << 4)
        | ((round_key >> 2) & 0x3fff0000)
        | ((round_key & 0x00030000) << 14)
}

#[must_use]
fn rearrange_round_key0(mut round_key: u32) -> u32 {
    round_key = swap_move_single(round_key, 0x00550055, 9);
    round_key = swap_move_single(round_key, 0x000f000f, 12);
    round_key = swap_move_single(round_key, 0x00003333, 18);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key1(mut round_key: u32) -> u32 {
    round_key = swap_move_single(round_key, 0x11111111, 3);
    round_key = swap_move_single(round_key, 0x03030303, 6);
    round_key = swap_move_single(round_key, 0x000f000f, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key2(mut round_key: u32) -> u32 {
    round_key = swap_move_single(round_key, 0x0000aaaa, 15);
    round_key = swap_move_single(round_key, 0x00003333, 18);
    round_key = swap_move_single(round_key, 0x0000f0f0, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn rearrange_round_key3(mut round_key: u32) -> u32 {
    round_key = swap_move_single(round_key, 0x0a0a0a0a, 3);
    round_key = swap_move_single(round_key, 0x00cc00cc, 6);
    round_key = swap_move_single(round_key, 0x0000f0f0, 12);
    swap_move_single(round_key, 0x000000ff, 24)
}

#[must_use]
fn key_triple_update_0(round_key: u32) -> u32 {
    (round_key & 0x33333333).rotate_right(24) | (round_key & 0xcccccccc).rotate_right(16)
}

#[must_use]
fn key_double_update_1(round_key: u32) -> u32 {
    ((round_key >> 4) & 0x0f000f00)
        | ((round_key & 0x0f000f00) << 4)
        | ((round_key >> 6) & 0x00030003)
        | ((round_key & 0x003f003f) << 2)
}

#[must_use]
fn key_triple_update_1(round_key: u32) -> u32 {
    ((round_key >> 6) & 0x03000300)
        | ((round_key & 0x3f003f00) << 2)
        | ((round_key >> 5) & 0x00070007)
        | ((round_key & 0x001f001f) << 3)
}

#[must_use]
fn key_double_update_2(round_key: u32) -> u32 {
    (round_key & 0xaaaaaaaa).rotate_right(24) | (round_key & 0x55555555).rotate_right(16)
}

#[must_use]
fn key_triple_update_2(round_key: u32) -> u32 {
    (round_key & 0x55555555).rotate_right(24) | (round_key & 0xaaaaaaaa).rotate_right(20)
}

#[must_use]
fn key_double_update_3(round_key: u32) -> u32 {
    ((round_key >> 2) & 0x03030303)
        | ((round_key & 0x03030303) << 2)
        | ((round_key >> 1) & 0x70707070)
        | ((round_key & 0x10101010) << 3)
}

#[must_use]
fn key_triple_update_3(round_key: u32) -> u32 {
    ((round_key >> 18) & 0x00003030)
        | ((round_key & 0x01010101) << 3)
        | ((round_key >> 14) & 0x0000c0c0)
        | ((round_key & 0x0000e0e0) << 15)
        | ((round_key >> 1) & 0x07070707)
        | ((round_key & 0x00001010) << 19)
}

#[must_use]
fn key_double_update_4(round_key: u32) -> u32 {
    ((round_key >> 4) & 0x0fff0000)
        | ((round_key & 0x000f0000) << 12)
        | ((round_key >> 8) & 0x000000ff)
        | ((round_key & 0x000000ff) << 8)
}

#[must_use]
fn key_triple_update_4(round_key: u32) -> u32 {
    ((round_key >> 6) & 0x03ff0000)
        | ((round_key & 0x003f0000) << 10)
        | ((round_key >> 4) & 0x00000fff)
        | ((round_key & 0x0000000f) << 12)
}

#[must_use]
pub fn precompute_round_keys(key: &Key) -> RoundKeys {
    let mut round_keys = [0; ROUNDS * 2];
    round_keys[0] = u32::from_le_bytes([key[12], key[13], key[14], key[15]]).swap_bytes();
    round_keys[1] = u32::from_le_bytes([key[4], key[5], key[6], key[7]]).swap_bytes();
    round_keys[2] = u32::from_le_bytes([key[8], key[9], key[10], key[11]]).swap_bytes();
    round_keys[3] = u32::from_le_bytes([key[0], key[1], key[2], key[3]]).swap_bytes();

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

    round_keys
}
