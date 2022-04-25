use crate::gift128::State;

#[must_use]
pub fn sbox(state: State) -> State {
    let (mut s0, mut s1, mut s2, mut s3) = state;
    s1 ^= s0 & s2;
    s0 ^= s1 & s3;
    s2 ^= s0 | s1;
    s3 ^= s2;
    s1 ^= s3;
    s3 ^= 0xffffffff;
    s2 ^= s0 & s1;
    (s0, s1, s2, s3)
}

#[must_use]
pub fn inv_sbox(state: State) -> State {
    let (mut s0, mut s1, mut s2, mut s3) = state;
    s2 ^= s3 & s1;
    s0 ^= 0xffffffff;
    s1 ^= s0;
    s0 ^= s2;
    s2 ^= s3 | s1;
    s3 ^= s1 & s0;
    s1 ^= s3 & s2;
    (s0, s1, s2, s3)
}
