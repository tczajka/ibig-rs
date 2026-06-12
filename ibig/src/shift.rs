//! Bit shift operators (`Shl`, `Shr`) for [`UBig`] and [`IBig`] by a `usize` amount.

use crate::ops::{BinaryOp, impl_binary_operator};
use crate::repr::{
    AsDigits,
    AsDigitsResult::{Large, Small},
    Digits, MAX_DIGITS, number_too_large,
};
use crate::{IBig, UBig};
use core::iter::repeat_n;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use ibig_core::{BitIndex, DIGIT_BITS_USIZE, Digit, SignedDigit};

/// Left shift.
enum ShlOperation {}

impl BinaryOp<UBig, usize> for ShlOperation {
    #[inline]
    fn apply_ref_ref(lhs: &UBig, rhs: &usize) -> UBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &UBig, rhs: usize) -> UBig {
        match lhs.as_digits() {
            Small(digit) => UBig::shl_digit(digit, rhs),
            Large(digits) => UBig::shl_ref(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: UBig, rhs: &usize) -> UBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: UBig, rhs: usize) -> UBig {
        match lhs.into_digits() {
            Small(digit) => UBig::shl_digit(digit, rhs),
            Large(digits) => UBig::shl_val(digits, rhs),
        }
    }
}

impl_binary_operator!(UBig, usize, Shl::shl, ShlAssign::shl_assign, ShlOperation);

impl UBig {
    /// Shifts a single-digit [`UBig`] left by `rhs` bits. The shifted digit spans at most two
    /// digits, sitting above `rhs / DIGIT_BITS` prepended zero digits.
    #[inline]
    fn shl_digit(digit: Digit, rhs: usize) -> UBig {
        // Shifting zero is zero (and avoids allocating `rhs / DIGIT_BITS` zero digits).
        if digit == Digit::ZERO {
            return UBig::ZERO;
        }
        let index = BitIndex::from(rhs);
        let (low, high) = ibig_core::shl_small_digit(digit, index.bit_index());
        // Fast path: the result is still a single digit.
        if index.digit_index() == 0 && high == Digit::ZERO {
            return UBig::from_digit(low);
        }
        if index.digit_index() >= MAX_DIGITS {
            number_too_large();
        }
        let mut digits = Digits::with_capacity(index.digit_index() + 2);
        digits.resize(index.digit_index(), Digit::ZERO);
        digits.push(low);
        digits.push(high);
        UBig::from_digits(digits)
    }

    /// Shifts `digits` left by `rhs` bits into a freshly allocated buffer.
    fn shl_ref(digits: &[Digit], rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() > MAX_DIGITS - digits.len() {
            number_too_large();
        }
        let mut new_digits = Digits::with_capacity(index.digit_index() + digits.len() + 1);
        new_digits.resize(index.digit_index(), Digit::ZERO);
        new_digits.extend_from_slice(digits);
        let overflow =
            ibig_core::shl_small(&mut new_digits[index.digit_index()..], index.bit_index());
        new_digits.push(overflow);
        UBig::from_digits(new_digits)
    }

    /// Shifts the owned `digits` left by `rhs` bits, reusing the buffer.
    fn shl_val(mut digits: Digits, rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() > MAX_DIGITS - digits.len() {
            number_too_large();
        }
        // Shift the bits in place, then prepend the whole zero digits.
        let overflow = ibig_core::shl_small(&mut digits, index.bit_index());
        digits.insert_many(0, repeat_n(Digit::ZERO, index.digit_index()));
        digits.push(overflow);
        UBig::from_digits(digits)
    }
}

impl BinaryOp<IBig, usize> for ShlOperation {
    #[inline]
    fn apply_ref_ref(lhs: &IBig, rhs: &usize) -> IBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &IBig, rhs: usize) -> IBig {
        match lhs.as_digits() {
            Small(d) => IBig::shl_digit(d, rhs),
            Large(digits) => IBig::shl_ref(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: IBig, rhs: &usize) -> IBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: IBig, rhs: usize) -> IBig {
        match lhs.into_digits() {
            Small(d) => IBig::shl_digit(d, rhs),
            Large(buffer) => IBig::shl_val(buffer, rhs),
        }
    }
}

impl_binary_operator!(IBig, usize, Shl::shl, ShlAssign::shl_assign, ShlOperation);

impl IBig {
    /// Shifts a single-digit [`IBig`] left by `rhs` bits. The shifted digit spans at most two
    /// digits, sitting above `rhs / DIGIT_BITS` prepended zero digits.
    #[inline]
    fn shl_digit(digit: SignedDigit, rhs: usize) -> IBig {
        if digit == SignedDigit::ZERO {
            return IBig::ZERO;
        }
        let index = BitIndex::from(rhs);
        let (low, high) = ibig_core::shl_small_signed_digit(digit, index.bit_index());
        // Fast path: the result still fits in a single digit.
        if index.digit_index() == 0 && high == ibig_core::sign_extension(low.cast_signed()) {
            return IBig::from_digit(low.cast_signed());
        }
        if index.digit_index() >= MAX_DIGITS {
            number_too_large();
        }
        let mut digits = Digits::with_capacity(index.digit_index() + 2);
        digits.resize(index.digit_index(), Digit::ZERO);
        digits.push(low);
        digits.push(high.cast_unsigned());
        IBig::from_digits(digits)
    }

    /// Shifts the signed two's complement `digits` left by `rhs` bits into a freshly allocated
    /// buffer.
    fn shl_ref(digits: &[Digit], rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() > MAX_DIGITS - digits.len() {
            number_too_large();
        }
        let mut new_digits = Digits::with_capacity(index.digit_index() + digits.len() + 1);
        new_digits.resize(index.digit_index(), Digit::ZERO);
        new_digits.extend_from_slice(digits);
        let overflow =
            ibig_core::shl_small_signed(&mut new_digits[index.digit_index()..], index.bit_index());
        new_digits.push(overflow.cast_unsigned());
        IBig::from_digits(new_digits)
    }

    /// Shifts the owned signed two's complement `digits` left by `rhs` bits, reusing the buffer.
    fn shl_val(mut digits: Digits, rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() > MAX_DIGITS - digits.len() {
            number_too_large();
        }
        // Shift the bits in place (sign-extending the overflow), then prepend zeros.
        let overflow = ibig_core::shl_small_signed(&mut digits, index.bit_index());
        digits.insert_many(0, repeat_n(Digit::ZERO, index.digit_index()));
        digits.push(overflow.cast_unsigned());
        IBig::from_digits(digits)
    }
}

/// Right shift.
enum ShrOperation {}

impl BinaryOp<UBig, usize> for ShrOperation {
    #[inline]
    fn apply_ref_ref(lhs: &UBig, rhs: &usize) -> UBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &UBig, rhs: usize) -> UBig {
        match lhs.as_digits() {
            Small(d) => UBig::shr_digit(d, rhs),
            Large(digits) => UBig::shr_ref(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: UBig, rhs: &usize) -> UBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: UBig, rhs: usize) -> UBig {
        match lhs.into_digits() {
            Small(d) => UBig::shr_digit(d, rhs),
            Large(buffer) => UBig::shr_val(buffer, rhs),
        }
    }
}

impl_binary_operator!(UBig, usize, Shr::shr, ShrAssign::shr_assign, ShrOperation);

impl UBig {
    /// Shifts a single-digit [`UBig`] right by `rhs` bits.
    #[inline]
    fn shr_digit(d: Digit, rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() != 0 {
            return UBig::ZERO;
        }
        UBig::from_digit(d >> index.bit_index())
    }

    /// Shifts `digits` right by `rhs` bits into a freshly allocated buffer.
    fn shr_ref(digits: &[Digit], rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() >= digits.len() {
            return UBig::ZERO;
        }
        let mut new_digits = Digits::from_slice(&digits[index.digit_index()..]);
        ibig_core::shr_small(&mut new_digits, index.bit_index());
        UBig::from_digits(new_digits)
    }

    /// Shifts the owned `digits` right by `rhs` bits, reusing the buffer.
    fn shr_val(mut digits: Digits, rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() >= digits.len() {
            return UBig::ZERO;
        }
        digits.drain(..index.digit_index());
        ibig_core::shr_small(&mut digits, index.bit_index());
        UBig::from_digits(digits)
    }
}

impl BinaryOp<IBig, usize> for ShrOperation {
    #[inline]
    fn apply_ref_ref(lhs: &IBig, rhs: &usize) -> IBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &IBig, rhs: usize) -> IBig {
        match lhs.as_digits() {
            Small(d) => IBig::shr_digit(d, rhs),
            Large(digits) => IBig::shr_ref(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: IBig, rhs: &usize) -> IBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: IBig, rhs: usize) -> IBig {
        match lhs.into_digits() {
            Small(d) => IBig::shr_digit(d, rhs),
            Large(buffer) => IBig::shr_val(buffer, rhs),
        }
    }
}

impl_binary_operator!(IBig, usize, Shr::shr, ShrAssign::shr_assign, ShrOperation);

impl IBig {
    /// Shifts a single-digit [`IBig`] right by `rhs` bits (an arithmetic shift).
    #[inline]
    fn shr_digit(digit: SignedDigit, rhs: usize) -> IBig {
        let small: u32 = rhs.min(DIGIT_BITS_USIZE - 1).try_into().unwrap();
        IBig::from_digit(digit >> small)
    }

    /// Shifts the signed two's complement `digits` right by `rhs` bits (an arithmetic shift)
    /// into a freshly allocated buffer.
    fn shr_ref(digits: &[Digit], rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() >= digits.len() {
            return IBig::shr_whole(digits);
        }
        // Copy only the surviving high digits.
        let mut new_digits = Digits::from_slice(&digits[index.digit_index()..]);
        ibig_core::shr_small_signed(&mut new_digits, index.bit_index());
        IBig::from_digits(new_digits)
    }

    /// Shifts the owned signed two's complement `digits` right by `rhs` bits (an arithmetic
    /// shift), reusing the buffer.
    fn shr_val(mut digits: Digits, rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        if index.digit_index() >= digits.len() {
            return IBig::shr_whole(&digits);
        }
        digits.drain(..index.digit_index());
        ibig_core::shr_small_signed(&mut digits, index.bit_index());
        IBig::from_digits(digits)
    }

    /// The result of right-shifting the two's complement `digits` past all of its digits: the
    /// sign, `0` or `-1`.
    #[inline]
    fn shr_whole(digits: &[Digit]) -> IBig {
        IBig::from_digit(ibig_core::sign_extension(
            digits.last().unwrap().cast_signed(),
        ))
    }
}
