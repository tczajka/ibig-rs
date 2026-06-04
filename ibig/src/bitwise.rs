//! Bitwise operators for [`UBig`].

use core::ops::{BitAnd, BitAndAssign};

use crate::UBig;
use crate::ops::{CommutativeBinaryOpDigits, impl_binary_operator};
use crate::repr::Digits;
use ibig_core::Digit;

/// Bitwise AND operation.
enum BitAndOperation {}

impl CommutativeBinaryOpDigits<UBig> for BitAndOperation {
    #[inline]
    fn apply_digit_digit(lhs: Digit, rhs: Digit) -> UBig {
        UBig::from_digit(lhs & rhs)
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: Digit) -> UBig {
        UBig::from_digit(lhs[0] & rhs)
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> UBig {
        let n = lhs.len().min(rhs.len());
        let mut digits = Digits::from_slice(&lhs[..n]);
        ibig_core::and_same_len_in_place(&mut digits, &rhs[..n]);
        UBig::from_digits(digits)
    }

    #[inline]
    fn apply_val_digit(lhs: Digits, rhs: Digit) -> UBig {
        Self::apply_ref_digit(&lhs, rhs)
    }

    #[inline]
    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> UBig {
        let n = lhs.len().min(rhs.len());
        lhs.truncate(n);
        ibig_core::and_same_len_in_place(&mut lhs, &rhs[..n]);
        UBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> UBig {
        // Reuse storage from shorter operand.
        if lhs.len() <= rhs.len() {
            Self::apply_val_ref(lhs, &rhs)
        } else {
            Self::apply_val_ref(rhs, &lhs)
        }
    }
}

impl_binary_operator!(
    UBig,
    BitAnd::bitand,
    BitAndAssign::bitand_assign,
    BitAndOperation
);
