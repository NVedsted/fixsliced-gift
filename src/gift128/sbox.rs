use core::ops::{BitAnd, BitOr, BitXorAssign};

use crate::gift128::State;

pub trait SboxTraits: BitXorAssign + Copy + BitAnd<Output=Self> + BitXorAssign<u32> + BitOr<Output=Self> {}

impl<T> SboxTraits for T
    where T: BitXorAssign + Copy + BitAnd<Output=T> + BitXorAssign<u32> + BitOr<Output=T> {}

#[must_use]
#[inline]
pub(super) fn sbox<T: SboxTraits>(state: State<T>) -> State<T> {
    let State(mut s0, mut s1, mut s2, mut s3) = state;
    s1 ^= s0 & s2;
    s0 ^= s1 & s3;
    s2 ^= s0 | s1;
    s3 ^= s2;
    s1 ^= s3;
    s3 ^= 0xffffffff;
    s2 ^= s0 & s1;
    State(s0, s1, s2, s3)
}

#[must_use]
#[inline]
pub(super) fn inv_sbox<T: SboxTraits>(state: State<T>) -> State<T> {
    let State(mut s0, mut s1, mut s2, mut s3) = state;
    s2 ^= s3 & s1;
    s0 ^= 0xffffffff;
    s1 ^= s0;
    s0 ^= s2;
    s2 ^= s3 | s1;
    s3 ^= s1 & s0;
    s1 ^= s3 & s2;
    State(s0, s1, s2, s3)
}

#[cfg(test)]
mod tests {
    use crate::gift128::sbox::{inv_sbox, sbox};
    use crate::gift128::State;

    #[test]
    fn test_masked_sbox() {
        let initial_state = State(0xd576370du32, 0x8b52c000u32, 0x2bafc0ccu32, 0xa3487987u32);
        let masks = (0x1d54f08eu32, 0x550aaf8cu32, 0xb3d27d46u32, 0x4aafa1b4u32);

        let masked_state = initial_state.make_shares(masks);
        let masked_output = sbox(masked_state);

        let output = masked_output.recover_shares();
        assert_eq!(output, sbox(initial_state));
    }

    #[test]
    fn test_masked_inv_sbox() {
        let initial_state = State(0xd576370du32, 0x8b52c000u32, 0x2bafc0ccu32, 0xa3487987u32);
        let masks = (0x1d54f08eu32, 0x550aaf8cu32, 0xb3d27d46u32, 0x4aafa1b4u32);

        let masked_state = initial_state.make_shares(masks);
        let masked_output = inv_sbox(masked_state);

        let output = masked_output.recover_shares();
        assert_eq!(output, inv_sbox(initial_state));
    }
}
