pub trait Rotate {
    #[must_use]
    fn rotate_right(self, rhs: u32) -> Self;
}

impl Rotate for u32 {
    fn rotate_right(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
}
