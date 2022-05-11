use core::ops::{BitAnd, BitXor, Shl, Shr};

pub trait SwapMoveTraits: Shr<usize, Output=Self> + BitXor<Output=Self> + BitAnd<u32, Output=Self> + Copy + Shl<usize, Output=Self> {}

impl<T> SwapMoveTraits for T
    where T: Shr<usize, Output=T> + BitXor<Output=T> + BitAnd<u32, Output=T> + Copy + Shl<usize, Output=T> {}

#[must_use]
pub fn swap_move<T: SwapMoveTraits>(a: T, b: T, mask: u32, n: usize) -> (T, T) {
    let tmp = (b ^ (a >> n)) & mask;
    (a ^ (tmp << n), b ^ tmp)
}

#[must_use]
pub fn swap_move_single<T: SwapMoveTraits>(a: T, mask: u32, n: usize) -> T {
    let tmp = (a ^ (a >> n)) & mask;
    a ^ tmp ^ (tmp << n)
}
