use core::ops::{BitAnd, BitOr, BitXorAssign, Shl, Shr};

use crate::gift128::key_schedule::RoundKeys;
use crate::gift128::rotate::Rotate;
use crate::gift128::sbox::{inv_sbox, sbox, SboxTraits};
use crate::gift128::State;
use crate::swap_move_single;
use crate::swapmove::SwapMoveTraits;

pub const ROUNDS: usize = 40;
const ROUND_CONSTANTS: [u32; ROUNDS] = [
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

pub trait StateOperations {
    #[must_use]
    fn byte_ror_2(self) -> Self;

    #[must_use]
    fn byte_ror_4(self) -> Self;

    #[must_use]
    fn byte_ror_6(self) -> Self;

    #[must_use]
    fn half_ror_4(self) -> Self;

    #[must_use]
    fn half_ror_8(self) -> Self;

    #[must_use]
    fn half_ror_12(self) -> Self;

    #[must_use]
    fn nibble_ror_1(self) -> Self;

    #[must_use]
    fn nibble_ror_2(self) -> Self;

    #[must_use]
    fn nibble_ror_3(self) -> Self;
}

impl<T> StateOperations for T
    where T: Copy + Shr<u32, Output=Self> + BitAnd<u32, Output=Self> + Shl<u32, Output=Self> + BitOr<Output=Self> {
    fn byte_ror_2(self) -> Self {
        ((self >> 2) & 0x3f3f3f3f) | ((self & 0x03030303) << 6)
    }

    fn byte_ror_4(self) -> Self {
        ((self >> 4) & 0x0f0f0f0f) | ((self & 0x0f0f0f0f) << 4)
    }

    fn byte_ror_6(self) -> Self {
        ((self >> 6) & 0x03030303) | ((self & 0x3f3f3f3f) << 2)
    }

    fn half_ror_4(self) -> Self {
        ((self >> 4) & 0x0fff0fff) | ((self & 0x000f000f) << 12)
    }

    fn half_ror_8(self) -> Self {
        ((self >> 8) & 0x00ff00ff) | ((self & 0x00ff00ff) << 8)
    }

    fn half_ror_12(self) -> Self {
        ((self >> 12) & 0x000f000f) | ((self & 0x0fff0fff) << 4)
    }

    fn nibble_ror_1(self) -> Self {
        ((self >> 1) & 0x77777777) | ((self & 0x11111111) << 3)
    }

    fn nibble_ror_2(self) -> Self {
        ((self >> 2) & 0x33333333) | ((self & 0x33333333) << 2)
    }

    fn nibble_ror_3(self) -> Self {
        ((self >> 3) & 0x11111111) | ((self & 0x77777777) << 1)
    }
}

pub trait RoundTraits: SboxTraits + StateOperations + SwapMoveTraits + BitXorAssign<u32> + Rotate {}

impl<T> RoundTraits for T
    where T: SboxTraits + StateOperations + SwapMoveTraits + BitXorAssign<u32> + Rotate {}

// TODO: possible to fix size on slices?
#[must_use]
pub(super) fn quintuple_round<T: RoundTraits>(state: State<T>, round_keys: &[u32], round_constants: &[u32]) -> State<T> {
    let State(mut s0, mut s1, mut s2, mut s3) = state;
    State(s0, s1, s2, s3) = sbox(State(s0, s1, s2, s3));
    s3 = s3.nibble_ror_1();
    s1 = s1.nibble_ror_2();
    s2 = s2.nibble_ror_3();
    s1 ^= round_keys[0];
    s2 ^= round_keys[1];
    s0 ^= round_constants[0];
    State(s3, s1, s2, s0) = sbox(State(s3, s1, s2, s0));
    s0 = s0.half_ror_4();
    s1 = s1.half_ror_8();
    s2 = s2.half_ror_12();
    s1 ^= round_keys[2];
    s2 ^= round_keys[3];
    s3 ^= round_constants[1];
    State(s0, s1, s2, s3) = sbox(State(s0, s1, s2, s3));
    s3 = s3.rotate_right(16);
    s2 = s2.rotate_right(16);
    s1 = swap_move_single(s1, 0x55555555, 1);
    s2 = swap_move_single(s2, 0x00005555, 1);
    s3 = swap_move_single(s3, 0x55550000, 1);
    s1 ^= round_keys[4];
    s2 ^= round_keys[5];
    s0 ^= round_constants[2];
    State(s3, s1, s2, s0) = sbox(State(s3, s1, s2, s0));
    s0 = s0.byte_ror_6();
    s1 = s1.byte_ror_4();
    s2 = s2.byte_ror_2();
    s1 ^= round_keys[6];
    s2 ^= round_keys[7];
    s3 ^= round_constants[3];
    State(s0, s1, s2, s3) = sbox(State(s0, s1, s2, s3));
    s3 = s3.rotate_right(24);
    s1 = s1.rotate_right(16);
    s2 = s2.rotate_right(8);
    s1 ^= round_keys[8];
    s2 ^= round_keys[9];
    s0 ^= round_constants[4];
    s0 ^= s3;
    s3 ^= s0;
    s0 ^= s3;

    State(s0, s1, s2, s3)
}

// TODO: possible to fix size on slices?
#[must_use]
fn inv_quintuple_round<T: RoundTraits>(state: State<T>, round_keys: &[u32], round_constants: &[u32]) -> State<T> {
    let State(mut s0, mut s1, mut s2, mut s3) = state;
    s0 ^= s3;
    s3 ^= s0;
    s0 ^= s3;
    s1 ^= round_keys[8];
    s2 ^= round_keys[9];
    s0 ^= round_constants[4];
    s3 = s3.rotate_right(8);
    s1 = s1.rotate_right(16);
    s2 = s2.rotate_right(24);
    State(s3, s1, s2, s0) = inv_sbox(State(s3, s1, s2, s0));
    s1 ^= round_keys[6];
    s2 ^= round_keys[7];
    s3 ^= round_constants[3];
    s0 = s0.byte_ror_2();
    s1 = s1.byte_ror_4();
    s2 = s2.byte_ror_6();
    State(s0, s1, s2, s3) = inv_sbox(State(s0, s1, s2, s3));
    s1 ^= round_keys[4];
    s2 ^= round_keys[5];
    s0 ^= round_constants[2];
    s3 = swap_move_single(s3, 0x55550000, 1);
    s1 = swap_move_single(s1, 0x55555555, 1);
    s2 = swap_move_single(s2, 0x00005555, 1);
    s3 = s3.rotate_right(16);
    s2 = s2.rotate_right(16);
    State(s3, s1, s2, s0) = inv_sbox(State(s3, s1, s2, s0));
    s1 ^= round_keys[2];
    s2 ^= round_keys[3];
    s3 ^= round_constants[1];
    s0 = s0.half_ror_12();
    s1 = s1.half_ror_8();
    s2 = s2.half_ror_4();
    State(s0, s1, s2, s3) = inv_sbox(State(s0, s1, s2, s3));
    s1 ^= round_keys[0];
    s2 ^= round_keys[1];
    s0 ^= round_constants[0];
    s3 = s3.nibble_ror_3();
    s1 = s1.nibble_ror_2();
    s2 = s2.nibble_ror_1();
    State(s3, s1, s2, s0) = inv_sbox(State(s3, s1, s2, s0));

    State(s0, s1, s2, s3)
}

pub(super) fn rounds<T: RoundTraits>(mut state: State<T>, round_keys: &RoundKeys) -> State<T> {
    for i in (0..ROUNDS).step_by(5) {
        state = quintuple_round(
            state,
            &round_keys[i * 2..i * 2 + 10],
            &ROUND_CONSTANTS[i..i + 5],
        );
    }

    state
}

pub(super) fn inv_rounds<T: RoundTraits>(mut state: State<T>, round_keys: &RoundKeys) -> State<T> {
    for i in (0..ROUNDS).step_by(5).rev() {
        state = inv_quintuple_round(
            state,
            &round_keys[i * 2..i * 2 + 10],
            &ROUND_CONSTANTS[i..i + 5],
        );
    }

    state
}
