//! Mathematical functions.

use crate::{arch::Word, primitive::PrimitiveUnsigned};

/// The length of an integer in bits.
/// 0 for 0.
pub(crate) fn bit_len<T>(x: T) -> u32
where
    T: PrimitiveUnsigned,
{
    T::BIT_SIZE - x.leading_zeros()
}

/// The length of an integer in bits.
/// 0 for 0.
pub(crate) const fn const_bit_len_word(x: Word) -> u32 {
    Word::BIT_SIZE - x.leading_zeros()
}

/// Ceiling of log_2(x).
/// x must be non-zero.
pub(crate) fn ceil_log_2<T>(x: T) -> u32
where
    T: PrimitiveUnsigned,
{
    debug_assert!(x != T::from(0u8));
    bit_len(x - T::from(1u8))
}

/// Ceiling of log_2(x).
/// x must be non-zero.
pub(crate) const fn const_ceil_log_2_word(x: Word) -> u32 {
    const_bit_len_word(x - 1)
}

/// Ceiling of a / b.
pub(crate) fn ceil_div<T>(a: T, b: T) -> T
where
    T: PrimitiveUnsigned,
{
    if a == T::from(0u8) {
        T::from(0u8)
    } else {
        (a - T::from(1u8)) / b + T::from(1u8)
    }
}

/// n ones: 2^n - 1
pub(crate) fn ones<T>(n: u32) -> T
where
    T: PrimitiveUnsigned,
{
    if n == 0 {
        T::from(0u8)
    } else {
        T::MAX >> (T::BIT_SIZE - n)
    }
}

/// n ones: 2^n - 1
pub(crate) const fn const_ones_word(n: u32) -> Word {
    if n == 0 {
        0
    } else {
        Word::MAX >> (Word::BIT_SIZE - n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_len() {
        assert_eq!(bit_len(0u32), 0);
        assert_eq!(bit_len(0b10011101u32), 8);
        assert_eq!(bit_len(0b10000000u32), 8);
        assert_eq!(bit_len(0b1111111u32), 7);
    }

    #[test]
    fn test_ceil_log_2() {
        assert_eq!(ceil_log_2(1u32), 0);
        assert_eq!(ceil_log_2(7u32), 3);
        assert_eq!(ceil_log_2(8u32), 3);
        assert_eq!(ceil_log_2(9u32), 4);
        assert_eq!(ceil_log_2(u32::MAX), 32);
    }

    #[test]
    fn test_ceil_div() {
        assert_eq!(ceil_div(0u32, 10u32), 0);
        assert_eq!(ceil_div(9u32, 10u32), 1);
        assert_eq!(ceil_div(10u32, 10u32), 1);
        assert_eq!(ceil_div(11u32, 10u32), 2);
    }

    #[test]
    fn test_ones() {
        assert_eq!(ones::<u32>(0), 0);
        assert_eq!(ones::<u32>(5), 0b11111);
        assert_eq!(ones::<u32>(32), u32::MAX);
    }
}
