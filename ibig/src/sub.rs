//! Subtraction operators (`Sub`) for [`UBig`].

use crate::UBig;
use crate::ops::{BinaryOpDigits, DigitsRhs, impl_binary_operator};
use crate::repr::Digits;
use core::ops::{Sub, SubAssign};
use ibig_core::Digit;

/// Subtraction operation.
///
/// Panics when the result would be negative.
struct SubOperation;

impl BinaryOpDigits<UBig> for SubOperation {
    #[inline]
    fn apply_digit_digit(lhs: Digit, rhs: Digit) -> UBig {
        let (diff, borrow) = lhs.overflowing_sub(rhs);
        if borrow {
            UBig::panic_negative();
        }
        UBig::from_digit(diff)
    }

    fn apply_digit_ref(_lhs: Digit, _rhs: &[Digit]) -> UBig {
        // A multi-digit `rhs` is bigger than any single digit.
        UBig::panic_negative()
    }

    fn apply_digit_val(_lhs: Digit, _rhs: Digits) -> UBig {
        // A multi-digit `rhs` is bigger than any single digit.
        UBig::panic_negative()
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: Digit) -> UBig {
        Self::apply_val_digit(Digits::from_slice(lhs), rhs)
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> UBig {
        // Check the lengths before cloning a result that would only be discarded.
        if lhs.len() < rhs.len() {
            UBig::panic_negative();
        }
        Self::apply_val_ref(Digits::from_slice(lhs), rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &[Digit], rhs: Digits) -> UBig {
        Self::apply_ref_ref(lhs, &rhs)
    }

    #[inline]
    fn apply_val_digit(mut lhs: Digits, rhs: Digit) -> UBig {
        // `lhs` has at least two digits, so subtracting a single digit cannot underflow.
        let borrow = ibig_core::sub_digit(&mut lhs, rhs);
        assert!(!borrow);
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> UBig {
        if lhs.len() < rhs.len() || ibig_core::sub(&mut lhs, rhs) {
            UBig::panic_negative();
        }
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> UBig {
        // Reuse storage from `lhs`; the result is never longer than `lhs`.
        Self::apply_val_ref(lhs, &rhs)
    }
}

impl_binary_operator!(
    UBig,
    UBig,
    Sub::sub,
    SubAssign::sub_assign,
    DigitsRhs<SubOperation>
);
