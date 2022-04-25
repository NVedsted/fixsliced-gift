use crate::gift128::sbox::{inv_sbox, sbox};
use crate::gift128::State;
use crate::swap_move_single;

// TODO: make private again
pub const ROUND_CONSTANTS: [u32; 40] = [
    0x10000008, 0x80018000, 0x54000002, 0x01010181,
    0x8000001f, 0x10888880, 0x6001e000, 0x51500002,
    0x03030180, 0x8000002f, 0x10088880, 0x60016000,
    0x41500002, 0x03030080, 0x80000027, 0x10008880,
    0x4001e000, 0x11500002, 0x03020180, 0x8000002b,
    0x10080880, 0x60014000, 0x01400002, 0x02020080,
    0x80000021, 0x10000080, 0x0001c000, 0x51000002,
    0x03010180, 0x8000002e, 0x10088800, 0x60012000,
    0x40500002, 0x01030080, 0x80000006, 0x10008808,
    0xc001a000, 0x14500002, 0x01020181, 0x8000001a
];

#[must_use]
fn byte_ror_2(x: u32) -> u32 {
    ((x >> 2) & 0x3f3f3f3f) | ((x & 0x03030303) << 6)
}

#[must_use]
fn byte_ror_4(x: u32) -> u32 {
    ((x >> 4) & 0x0f0f0f0f) | ((x & 0x0f0f0f0f) << 4)
}

#[must_use]
fn byte_ror_6(x: u32) -> u32 {
    ((x >> 6) & 0x03030303) | ((x & 0x3f3f3f3f) << 2)
}

#[must_use]
fn half_ror_4(x: u32) -> u32 {
    ((x >> 4) & 0x0fff0fff) | ((x & 0x000f000f) << 12)
}

#[must_use]
fn half_ror_8(x: u32) -> u32 {
    ((x >> 8) & 0x00ff00ff) | ((x & 0x00ff00ff) << 8)
}

#[must_use]
fn half_ror_12(x: u32) -> u32 {
    ((x >> 12) & 0x000f000f) | ((x & 0x0fff0fff) << 4)
}

#[must_use]
fn nibble_ror_1(x: u32) -> u32 {
    ((x >> 1) & 0x77777777) | ((x & 0x11111111) << 3)
}

#[must_use]
fn nibble_ror_2(x: u32) -> u32 {
    ((x >> 2) & 0x33333333) | ((x & 0x33333333) << 2)
}

#[must_use]
fn nibble_ror_3(x: u32) -> u32 {
    ((x >> 3) & 0x11111111) | ((x & 0x77777777) << 1)
}

// TODO: possible to fix size on slices?
#[must_use]
pub fn quintuple_round(state: State, round_keys: &[u32], round_constants: &[u32]) -> State {
    let (mut s0, mut s1, mut s2, mut s3) = state;
    (s0, s1, s2, s3) = sbox((s0, s1, s2, s3));
    s3 = nibble_ror_1(s3);
    s1 = nibble_ror_2(s1);
    s2 = nibble_ror_3(s2);
    s1 ^= round_keys[0];
    s2 ^= round_keys[1];
    s0 ^= round_constants[0];
    (s3, s1, s2, s0) = sbox((s3, s1, s2, s0));
    s0 = half_ror_4(s0);
    s1 = half_ror_8(s1);
    s2 = half_ror_12(s2);
    s1 ^= round_keys[2];
    s2 ^= round_keys[3];
    s3 ^= round_constants[1];
    (s0, s1, s2, s3) = sbox((s0, s1, s2, s3));
    s3 = s3.rotate_right(16);
    s2 = s2.rotate_right(16);
    s1 = swap_move_single(s1, 0x55555555, 1);
    s2 = swap_move_single(s2, 0x00005555, 1);
    s3 = swap_move_single(s3, 0x55550000, 1);
    s1 ^= round_keys[4];
    s2 ^= round_keys[5];
    s0 ^= round_constants[2];
    (s3, s1, s2, s0) = sbox((s3, s1, s2, s0));
    s0 = byte_ror_6(s0);
    s1 = byte_ror_4(s1);
    s2 = byte_ror_2(s2);
    s1 ^= round_keys[6];
    s2 ^= round_keys[7];
    s3 ^= round_constants[3];
    (s0, s1, s2, s3) = sbox((s0, s1, s2, s3));
    s3 = s3.rotate_right(24);
    s1 = s1.rotate_right(16);
    s2 = s2.rotate_right(8);
    s1 ^= round_keys[8];
    s2 ^= round_keys[9];
    s0 ^= round_constants[4];
    s0 ^= s3;
    s3 ^= s0;
    s0 ^= s3;

    (s0, s1, s2, s3)
}

// TODO: possible to fix size on slices?
#[must_use]
pub fn inv_quintuple_round(state: State, round_keys: &[u32], round_constants: &[u32]) -> State {
    let (mut s0, mut s1, mut s2, mut s3) = state;
    s0 ^= s3;
    s3 ^= s0;
    s0 ^= s3;
    s1 ^= round_keys[8];
    s2 ^= round_keys[9];
    s0 ^= round_constants[4];
    s3 = s3.rotate_right(8);
    s1 = s1.rotate_right(16);
    s2 = s2.rotate_right(24);
    (s3, s1, s2, s0) = inv_sbox((s3, s1, s2, s0));
    s1 ^= round_keys[6];
    s2 ^= round_keys[7];
    s3 ^= round_constants[3];
    s0 = byte_ror_2(s0);
    s1 = byte_ror_4(s1);
    s2 = byte_ror_6(s2);
    (s0, s1, s2, s3) = inv_sbox((s0, s1, s2, s3));
    s1 ^= round_keys[4];
    s2 ^= round_keys[5];
    s0 ^= round_constants[2];
    s3 = swap_move_single(s3, 0x55550000, 1);
    s1 = swap_move_single(s1, 0x55555555, 1);
    s2 = swap_move_single(s2, 0x00005555, 1);
    s3 = s3.rotate_right(16);
    s2 = s2.rotate_right(16);
    (s3, s1, s2, s0) = inv_sbox((s3, s1, s2, s0));
    s1 ^= round_keys[2];
    s2 ^= round_keys[3];
    s3 ^= round_constants[1];
    s0 = half_ror_12(s0);
    s1 = half_ror_8(s1);
    s2 = half_ror_4(s2);
    (s0, s1, s2, s3) = inv_sbox((s0, s1, s2, s3));
    s1 ^= round_keys[0];
    s2 ^= round_keys[1];
    s0 ^= round_constants[0];
    s3 = nibble_ror_3(s3);
    s1 = nibble_ror_2(s1);
    s2 = nibble_ror_1(s2);
    (s3, s1, s2, s0) = inv_sbox((s3, s1, s2, s0));

    (s0, s1, s2, s3)
}
