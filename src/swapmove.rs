#[must_use]
pub fn swap_move(a: u32, b: u32, mask: u32, n: usize) -> (u32, u32) {
    let tmp = (b ^ (a >> n)) & mask;
    (a ^ (tmp << n), b ^ tmp)
}

#[must_use]
pub fn swap_move_single(a: u32, mask: u32, n: usize) -> u32 {
    let tmp = (a ^ (a >> n)) & mask;
    a ^ tmp ^ (tmp << n)
}
