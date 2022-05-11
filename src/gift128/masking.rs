use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};
use crate::gift128::rotate::Rotate;
use crate::gift128::State;

#[derive(Copy, Clone, Debug)]
pub struct BinaryMask<T>(T, T);

impl<T: BitXor<Output=T> + Copy> BinaryMask<T> {
    pub fn make_shares(v: T, m: T) -> Self {
        BinaryMask(v ^ m, m)
    }

    pub fn recover_shares(self) -> T {
        self.0 ^ self.1
    }
}

impl<T> BitAnd for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + BitOr<Output=T> + BitXor<Output=T> + Copy {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let z1 = (self.0 & rhs.0) ^ (self.0 | !rhs.1);
        let z2 = (self.1 & rhs.0) ^ (self.1 | !rhs.1);
        BinaryMask(z1, z2)
    }
}

impl<T> BitAnd<T> for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + Copy {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        let z1 = !(self.0 & rhs);
        let z2 = !(self.1 & rhs);
        BinaryMask(z1, z2)
    }
}

impl<T> BitAndAssign for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + BitOr<Output=T> + BitXor<Output=T> + Copy {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<T> BitOr for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + BitOr<Output=T> + BitXor<Output=T> + Copy {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let z1 = (self.0 & rhs.0) ^ (self.0 | rhs.1);
        let z2 = (self.1 | rhs.0) ^ (self.1 & rhs.1);
        BinaryMask(z1, z2)
    }
}

impl<T> BitOr<T> for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + BitOr<Output=T> + BitXor<Output=T> + Copy {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        let z1 = self.0 & !rhs;
        let z2 = self.1 | rhs;
        BinaryMask(z1, z2)
    }
}

impl<T> BitOrAssign for BinaryMask<T>
    where T: BitAnd<Output=T> + Not<Output=T> + BitOr<Output=T> + BitXor<Output=T> + Copy {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<T: Shl<R>, R: Copy> Shl<R> for BinaryMask<T> {
    type Output = BinaryMask<T::Output>;

    fn shl(self, rhs: R) -> Self::Output {
        BinaryMask(
            self.0 << rhs,
            self.1 << rhs,
        )
    }
}

impl<T: ShlAssign<R>, R: Copy> ShlAssign<R> for BinaryMask<T> {
    fn shl_assign(&mut self, rhs: R) {
        self.0 <<= rhs;
        self.1 <<= rhs;
    }
}

impl<T: Shr<R>, R: Copy> Shr<R> for BinaryMask<T> {
    type Output = BinaryMask<T::Output>;

    fn shr(self, rhs: R) -> Self::Output {
        BinaryMask(
            self.0 >> rhs,
            self.1 >> rhs,
        )
    }
}

impl<T: ShrAssign<R>, R: Copy> ShrAssign<R> for BinaryMask<T> {
    fn shr_assign(&mut self, rhs: R) {
        self.0 >>= rhs;
        self.1 >>= rhs;
    }
}

impl<T: BitXor> BitXor for BinaryMask<T> {
    type Output = BinaryMask<T::Output>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BinaryMask(
            self.0 ^ rhs.0,
            self.1 ^ rhs.1,
        )
    }
}

impl<T: BitXorAssign> BitXorAssign for BinaryMask<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
        self.1 ^= rhs.1;
    }
}

impl<T: BitXor<Output=T>> BitXor<T> for BinaryMask<T> {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        BinaryMask(
            self.0 ^ rhs,
            self.1,
        )
    }
}

impl<T: BitXorAssign> BitXorAssign<T> for BinaryMask<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        self.0 ^= rhs;
    }
}

impl<T: Rotate> Rotate for BinaryMask<T> {
    fn rotate_right(self, rhs: u32) -> Self {
        BinaryMask(
            self.0.rotate_right(rhs),
            self.1.rotate_right(rhs),
        )
    }
}

impl<T: BitXor<Output=T> + Copy> State<BinaryMask<T>> {
    pub fn make_shares(state: State<T>, masks: State<T>) -> Self {
        State(
            BinaryMask::make_shares(state.0, masks.0),
            BinaryMask::make_shares(state.1, masks.1),
            BinaryMask::make_shares(state.2, masks.2),
            BinaryMask::make_shares(state.3, masks.3),
        )
    }

    pub fn recover_shares(self) -> State<T> {
        State(
            self.0.recover_shares(),
            self.1.recover_shares(),
            self.2.recover_shares(),
            self.3.recover_shares(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::gift128::masking::BinaryMask;
    use crate::gift128::rotate::Rotate;
    use crate::gift128::rounds::StateOperations;

    #[test]
    fn test_and() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;
        let mask2 = 0xC2E993A4u32;

        let masked_value1 = BinaryMask::make_shares(value1, mask1);
        let masked_value2 = BinaryMask::make_shares(value2, mask2);

        let masked_result = masked_value1 & masked_value2;
        let result = masked_result.recover_shares();

        assert_eq!(result, value1 & value2);
    }

    #[test]
    fn test_and_with_constant() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;

        let masked_value = BinaryMask::make_shares(value1, mask1);

        let masked_result = masked_value & value2;
        let result = masked_result.recover_shares();

        assert_eq!(result, value1 & value2);
    }

    #[test]
    fn test_or() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;
        let mask2 = 0xC2E993A4u32;

        let masked_value1 = BinaryMask::make_shares(value1, mask1);
        let masked_value2 = BinaryMask::make_shares(value2, mask2);

        let masked_result = masked_value1 | masked_value2;
        let result = masked_result.recover_shares();

        assert_eq!(result, value1 | value2);
    }

    #[test]
    fn test_or_with_constant() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;

        let masked_value = BinaryMask::make_shares(value1, mask1);

        let masked_result = masked_value | value2;
        let result = masked_result.recover_shares();

        assert_eq!(result, value1 | value2);
    }

    #[test]
    fn test_shl() {
        let value = 0xD576370Du32;
        let mask = 0xB751F5EFu32;
        let masked_value = BinaryMask::make_shares(value, mask);

        let mut masked_result = masked_value << 1;
        masked_result <<= 1;
        let result = masked_result.recover_shares();

        assert_eq!(result, value << 2);
    }

    #[test]
    fn test_rotate() {
        let value = 0xD576370Du32;
        let mask = 0xB751F5EFu32;
        let masked_value = BinaryMask::make_shares(value, mask);

        let masked_result = masked_value.rotate_right(5);
        let result = masked_result.recover_shares();

        assert_eq!(result, value.rotate_right(5));
    }

    #[test]
    fn test_xor() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;
        let mask2 = 0xC2E993A4u32;

        let masked_value1 = BinaryMask::make_shares(value1, mask1);
        let masked_value2 = BinaryMask::make_shares(value2, mask2);

        let masked_result = masked_value1 ^ masked_value2;
        let result = masked_result.recover_shares();
        assert_eq!(result, value1 ^ value2);

        let mut masked_value1 = masked_value1;
        masked_value1 ^= masked_value2;
        let result = masked_value1.recover_shares();
        assert_eq!(result, value1 ^ value2);
    }

    #[test]
    fn test_xor_with_constant() {
        let value1 = 0xD576370Du32;
        let mask1 = 0xB751F5EFu32;
        let value2 = 0x6CC92B7Bu32;

        let masked_value1 = BinaryMask::make_shares(value1, mask1);

        let masked_result = masked_value1 ^ value2;
        let result = masked_result.recover_shares();
        assert_eq!(result, value1 ^ value2);

        let mut masked_value1 = masked_value1;
        masked_value1 ^= value2;
        let result = masked_value1.recover_shares();
        assert_eq!(result, value1 ^ value2);
    }

    #[test]
    fn test_state_operations() {
        let value = 0xD576370Du32;
        let mask = 0xB751F5EFu32;
        let masked_value = BinaryMask::make_shares(value, mask);

        assert_eq!(masked_value.byte_ror_2().recover_shares(), value.byte_ror_2());
        assert_eq!(masked_value.byte_ror_4().recover_shares(), value.byte_ror_4());
        assert_eq!(masked_value.byte_ror_6().recover_shares(), value.byte_ror_6());
        assert_eq!(masked_value.half_ror_4().recover_shares(), value.half_ror_4());
        assert_eq!(masked_value.half_ror_8().recover_shares(), value.half_ror_8());
        assert_eq!(masked_value.half_ror_12().recover_shares(), value.half_ror_12());
        assert_eq!(masked_value.nibble_ror_1().recover_shares(), value.nibble_ror_1());
        assert_eq!(masked_value.nibble_ror_2().recover_shares(), value.nibble_ror_2());
        assert_eq!(masked_value.nibble_ror_3().recover_shares(), value.nibble_ror_3());
    }
}