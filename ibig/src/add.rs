//! Addition operators (`Add`) for [`UBig`].

use crate::UBig;
use crate::ops::{CommutativeBinaryOpDigits, DigitsRhs, impl_binary_operator};
use crate::repr::Digits;
use core::ops::{Add, AddAssign};
use ibig_core::Digit;

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
        let carry = ibig_core::add_digit(&mut lhs, rhs);
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
            ibig_core::add(&mut lhs, rhs)
        } else {
            // Add the overlapping low digits, then append the high digits of `rhs` and
            // propagate the carry through them. Reserve for the appended digits and a
            // possible carry digit.
            let lhs_len = lhs.len();
            lhs.reserve(rhs.len() - lhs_len + 1);
            let (rhs_low, rhs_high) = rhs.split_at(lhs_len);
            let low_carry = ibig_core::add_same_len(&mut lhs, rhs_low);
            lhs.extend_from_slice(rhs_high);
            ibig_core::add_carry(&mut lhs[lhs_len..], low_carry)
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

/// Appends the carry out of an addition as a most-significant `1` digit.
#[inline]
fn push_carry(digits: &mut Digits, carry: bool) {
    if carry {
        digits.push(Digit::from(1u8));
    }
}
