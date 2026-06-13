//! Addition operators (`Add`) for [`UBig`] and [`IBig`].

use crate::ops::{CommutativeBinaryOpDigits, DigitsRhs, impl_binary_operator};
use crate::repr::Digits;
use crate::{IBig, UBig};
use core::ops::{Add, AddAssign};
use ibig_core::{Digit, SignedDigit, sign_extension};

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
        let (low, high) = ibig_core::add_sdigit_sdigit(lhs, rhs);
        IBig::from_two_digits(low, high)
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
            let fill = sign_extension(lhs[lhs_len - 1].cast_signed()).cast_unsigned();
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

/// Appends the sign digit of a signed addition, unless it is a redundant sign-extension of
/// the digit below it.
#[inline]
fn push_sign(digits: &mut Digits, high: SignedDigit) {
    if high != sign_extension(digits.last().unwrap().cast_signed()) {
        digits.push(high.cast_unsigned());
    }
}
