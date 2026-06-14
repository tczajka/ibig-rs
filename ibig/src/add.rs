//! Addition.

use crate::ops::{CommutativeBinaryOpDigits, DigitsRhs, impl_binary_operator};
use crate::repr::{
    AsDigits,
    AsDigitsResult::{Large, Small},
    Digits,
};
use crate::sign::push_sign;
use crate::{IBig, UBig};
use core::ops::{Add, AddAssign};
use ibig_core::{Digit, SignedDigit, sign_extension, sign_extension_sdigit};

impl UBig {
    /// Adds the signed `rhs` to `self`, returning `None` if the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{IBig, UBig};
    /// assert_eq!(UBig::from(5u8).checked_add_signed(&IBig::from(3)), Some(UBig::from(8u8)));
    /// assert_eq!(UBig::from(5u8).checked_add_signed(&IBig::from(-3)), Some(UBig::from(2u8)));
    /// assert_eq!(UBig::from(5u8).checked_add_signed(&IBig::from(-8)), None);
    /// ```
    #[inline]
    pub fn checked_add_signed(&self, rhs: &IBig) -> Option<UBig> {
        match (self.as_digits(), rhs.as_digits()) {
            (Small(a), Small(b)) => UBig::checked_add_signed_digit_sdigit(a, b),
            (Small(a), Large(rhs)) => UBig::checked_add_signed_digit_ref(a, rhs),
            (Large(lhs), Small(b)) => UBig::checked_add_signed_ref_sdigit(lhs, b),
            (Large(lhs), Large(rhs)) => UBig::checked_add_signed_ref_ref(lhs, rhs),
        }
    }

    /// Adds the signed `rhs` to `self`.
    ///
    /// # Panics
    ///
    /// Panics if the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{IBig, UBig};
    /// assert_eq!(UBig::from(5u8).strict_add_signed(&IBig::from(-3)), UBig::from(2u8));
    /// ```
    #[inline]
    pub fn strict_add_signed(&self, rhs: &IBig) -> UBig {
        self.checked_add_signed(rhs)
            .unwrap_or_else(|| UBig::panic_negative())
    }

    /// Adds the signed `rhs` to `self`, saturating at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{IBig, UBig};
    /// assert_eq!(UBig::from(5u8).saturating_add_signed(&IBig::from(-3)), UBig::from(2u8));
    /// assert_eq!(UBig::from(5u8).saturating_add_signed(&IBig::from(-8)), UBig::ZERO);
    /// ```
    #[inline]
    pub fn saturating_add_signed(&self, rhs: &IBig) -> UBig {
        self.checked_add_signed(rhs).unwrap_or(UBig::ZERO)
    }

    /// `checked_add_signed` for a single unsigned digit `a` and a single signed digit `b`.
    #[inline]
    fn checked_add_signed_digit_sdigit(a: Digit, b: SignedDigit) -> Option<UBig> {
        let (sum, overflow) = a.overflowing_add_signed(b);
        if !overflow {
            Some(UBig::from_digit(sum))
        } else if b.is_negative() {
            // The result is negative.
            None
        } else {
            // The sum overflowed a single digit into a most-significant `1`.
            Some(UBig::from_two_digits(sum, Digit::from(1u8)))
        }
    }

    /// `checked_add_signed` for a single unsigned digit `lhs` and a signed slice `rhs`.
    fn checked_add_signed_digit_ref(lhs: Digit, rhs: &[Digit]) -> Option<UBig> {
        // `rhs` (at least two digits) is longer than the single digit. If it is at least two
        // digits longer and negative, its magnitude exceeds `lhs`, so the result is negative.
        if rhs.len() >= 3 && ibig_core::is_negative(rhs) {
            return None;
        }
        // Clone the signed `rhs` (the longer operand) and add the unsigned digit `lhs`.
        let mut digits = Digits::with_capacity(rhs.len() + 1);
        digits.extend_from_slice(rhs);
        let scarry = ibig_core::add_signed_digit(&mut digits, lhs);
        Self::checked_add_signed_finish(digits, scarry)
    }

    /// `checked_add_signed` for an unsigned slice `lhs` and a single signed digit `rhs`.
    fn checked_add_signed_ref_sdigit(lhs: &[Digit], rhs: SignedDigit) -> Option<UBig> {
        // Clone the unsigned `lhs` (the longer operand) and add the signed digit `rhs`.
        let mut digits = Digits::with_capacity(lhs.len() + 1);
        digits.extend_from_slice(lhs);
        let scarry = ibig_core::add_unsigned_sdigit(&mut digits, rhs);
        Self::checked_add_signed_finish(digits, scarry)
    }

    /// `checked_add_signed` for an unsigned `lhs` and a signed `rhs`, both as digit slices.
    fn checked_add_signed_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> Option<UBig> {
        let mut digits;
        let scarry;
        if lhs.len() >= rhs.len() {
            // Clone the unsigned `lhs` (the longer operand) and add the signed `rhs`.
            digits = Digits::with_capacity(lhs.len() + 1);
            digits.extend_from_slice(lhs);
            scarry = ibig_core::add_unsigned_signed(&mut digits, rhs);
        } else {
            // `rhs` is longer. If it is at least two digits longer and negative, its magnitude
            // exceeds any `lhs` of `lhs.len()` digits, so the result is certainly negative.
            if rhs.len() >= lhs.len() + 2 && ibig_core::is_negative(rhs) {
                return None;
            }
            // Clone the signed `rhs` (the longer operand) and add the unsigned `lhs`.
            digits = Digits::with_capacity(rhs.len() + 1);
            digits.extend_from_slice(rhs);
            scarry = ibig_core::add_signed_unsigned(&mut digits, lhs);
        }
        Self::checked_add_signed_finish(digits, scarry)
    }

    /// Finishes a `checked_add_signed`: `scarry` is the most-significant digit of the result
    /// (the sum of an unsigned and a signed value), negative exactly when the result is.
    #[inline]
    fn checked_add_signed_finish(mut digits: Digits, scarry: SignedDigit) -> Option<UBig> {
        if scarry.is_negative() {
            None
        } else {
            // `scarry` is 0 or 1; appending it gives the (non-negative) magnitude.
            digits.push(scarry.cast_unsigned());
            Some(UBig::from_digits(digits))
        }
    }
}

/// Addition operation.
struct AddOperation;

impl CommutativeBinaryOpDigits<UBig> for AddOperation {
    #[inline]
    fn apply_digit_digit(lhs: Digit, rhs: Digit) -> UBig {
        let (sum, carry) = lhs.overflowing_add(rhs);
        UBig::from_two_digits(sum, carry.into())
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: Digit) -> UBig {
        // Clone with room for a possible carry digit.
        let mut digits = Digits::with_capacity(lhs.len() + 1);
        digits.extend_from_slice(lhs);
        Self::apply_val_digit(digits, rhs)
    }

    #[inline]
    fn apply_val_digit(mut lhs: Digits, rhs: Digit) -> UBig {
        let carry = ibig_core::add_unsigned_digit(&mut lhs, rhs);
        push_carry(&mut lhs, carry);
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> UBig {
        // Clone the longer operand, with room for a possible carry digit, and add
        // the shorter one to it.
        let (longer, shorter) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        let mut digits = Digits::with_capacity(longer.len() + 1);
        digits.extend_from_slice(longer);
        Self::apply_val_ref(digits, shorter)
    }

    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> UBig {
        let carry = if lhs.len() >= rhs.len() {
            ibig_core::add_unsigned_unsigned(&mut lhs, rhs)
        } else {
            // Add the overlapping low digits, then append the high digits of `rhs` and
            // propagate the carry through them. Reserve for the appended digits and a
            // possible carry digit.
            let lhs_len = lhs.len();
            lhs.reserve(rhs.len() - lhs_len + 1);
            let (rhs_low, rhs_high) = rhs.split_at(lhs_len);
            let low_carry = ibig_core::add_unsigned_unsigned_same_len(&mut lhs, rhs_low);
            lhs.extend_from_slice(rhs_high);
            ibig_core::add_unsigned_carry(&mut lhs[lhs_len..], low_carry)
        };
        push_carry(&mut lhs, carry);
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> UBig {
        // Reuse storage from the longer operand.
        if lhs.len() >= rhs.len() {
            Self::apply_val_ref(lhs, &rhs)
        } else {
            Self::apply_val_ref(rhs, &lhs)
        }
    }
}

impl_binary_operator!(
    UBig,
    UBig,
    Add::add,
    AddAssign::add_assign,
    DigitsRhs<AddOperation>
);

impl CommutativeBinaryOpDigits<IBig> for AddOperation {
    #[inline]
    fn apply_digit_digit(lhs: SignedDigit, rhs: SignedDigit) -> IBig {
        let (sum, overflow) = lhs.overflowing_add(rhs);
        if overflow {
            // On overflow `lhs` and `rhs` share a sign, which is the sign of the two-digit result.
            IBig::from_two_digits(sum.cast_unsigned(), sign_extension_sdigit(lhs))
        } else {
            IBig::from_digit(sum)
        }
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: SignedDigit) -> IBig {
        // Clone with room for a possible sign digit.
        let mut digits = Digits::with_capacity(lhs.len() + 1);
        digits.extend_from_slice(lhs);
        Self::apply_val_digit(digits, rhs)
    }

    #[inline]
    fn apply_val_digit(mut lhs: Digits, rhs: SignedDigit) -> IBig {
        let high = ibig_core::add_signed_sdigit(&mut lhs, rhs);
        push_sign(&mut lhs, high);
        IBig::from_digits(lhs)
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> IBig {
        // Clone the longer operand, with room for a possible sign digit, and add the
        // shorter one to it.
        let (longer, shorter) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        let mut digits = Digits::with_capacity(longer.len() + 1);
        digits.extend_from_slice(longer);
        Self::apply_val_ref(digits, shorter)
    }

    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> IBig {
        let lhs_len = lhs.len();
        if lhs_len < rhs.len() {
            // Sign-extend `lhs` to the length of `rhs`. Reserve for the extension digits
            // and a possible sign digit.
            lhs.reserve(rhs.len() - lhs_len + 1);
            let fill = sign_extension(&lhs).cast_unsigned();
            lhs.resize(rhs.len(), fill);
        }
        let high = ibig_core::add_signed_signed(&mut lhs, rhs);
        push_sign(&mut lhs, high);
        IBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> IBig {
        // Reuse storage from the longer operand.
        if lhs.len() >= rhs.len() {
            Self::apply_val_ref(lhs, &rhs)
        } else {
            Self::apply_val_ref(rhs, &lhs)
        }
    }
}

impl_binary_operator!(
    IBig,
    IBig,
    Add::add,
    AddAssign::add_assign,
    DigitsRhs<AddOperation>
);

/// Appends the carry out of an addition as a most-significant `1` digit.
#[inline]
fn push_carry(digits: &mut Digits, carry: bool) {
    if carry {
        digits.push(Digit::from(1u8));
    }
}
