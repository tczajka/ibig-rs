//! Subtraction.

use crate::UBig;
use crate::ops::{BinaryOpDigits, DigitsRhs, impl_binary_operator};
use crate::repr::{
    AsDigits,
    AsDigitsResult::{Large, Small},
    Digits,
};
use core::ops::{Sub, SubAssign};
use ibig_core::Digit;

impl UBig {
    /// Subtracts `rhs` from `self`, returning `None` if the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(5u8).checked_sub(&UBig::from(3u8)), Some(UBig::from(2u8)));
    /// assert_eq!(UBig::from(3u8).checked_sub(&UBig::from(5u8)), None);
    /// ```
    #[inline]
    pub fn checked_sub(&self, rhs: &UBig) -> Option<UBig> {
        match (self.as_digits(), rhs.as_digits()) {
            (Small(a), Small(b)) => a.checked_sub(b).map(UBig::from_digit),
            // A multi-digit `rhs` is bigger than any single digit.
            (Small(_), Large(_)) => None,
            // A multi-digit `lhs` minus a single digit never underflows.
            (Large(lhs), Small(b)) => Some(SubOperation::apply_ref_digit(lhs, b)),
            (Large(lhs), Large(rhs)) => checked_sub_large(lhs, rhs),
        }
    }

    /// Subtracts `rhs` from `self`, saturating at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(5u8).saturating_sub(&UBig::from(3u8)), UBig::from(2u8));
    /// assert_eq!(UBig::from(3u8).saturating_sub(&UBig::from(5u8)), UBig::ZERO);
    /// ```
    #[inline]
    pub fn saturating_sub(&self, rhs: &UBig) -> UBig {
        self.checked_sub(rhs).unwrap_or(UBig::ZERO)
    }
}

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
        let borrow = ibig_core::sub_unsigned_digit(&mut lhs, rhs);
        assert!(!borrow);
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> UBig {
        if lhs.len() < rhs.len() || ibig_core::sub_unsigned_unsigned(&mut lhs, rhs) {
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

/// Checked subtraction of multi-digit values.
#[inline]
fn checked_sub_large(lhs: &[Digit], rhs: &[Digit]) -> Option<UBig> {
    // A shorter `lhs` is necessarily smaller, and `ibig_core::sub_unsigned_unsigned` requires
    // `rhs` to not be longer than `lhs`.
    if lhs.len() < rhs.len() {
        return None;
    }
    let mut digits = Digits::from_slice(lhs);
    if ibig_core::sub_unsigned_unsigned(&mut digits, rhs) {
        return None;
    }
    Some(UBig::from_digits(digits))
}
