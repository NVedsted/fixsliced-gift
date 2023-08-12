pub trait Rotate {
    #[must_use]
    fn rotate_right(self, rhs: u32) -> Self;
}

impl Rotate for u32 {
    #[inline]
    fn rotate_right(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
}

pub trait SwapBytes {
    fn swap_bytes(self) -> Self;
}

impl SwapBytes for u32 {
    #[inline]
    fn swap_bytes(self) -> Self {
        self.swap_bytes()
    }
}
