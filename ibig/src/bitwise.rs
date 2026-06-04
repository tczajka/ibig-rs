//! Bitwise operators for [`UBig`] and [`IBig`].

use crate::ops::{
    CommutativeBinaryOpDigits, UnaryOpDigits, impl_binary_operator, impl_unary_operator,
};
use crate::repr::Digits;
use crate::{IBig, UBig};
use core::ops::{BitAnd, BitAndAssign, Not};
use ibig_core::{Digit, SignedDigit};

/// Bitwise NOT operation.
enum NotOperation {}

impl UnaryOpDigits<IBig> for NotOperation {
    #[inline]
    fn apply_digit(operand: SignedDigit) -> IBig {
        IBig::from_digit(!operand)
    }

    #[inline]
    fn apply_ref(operand: &[Digit]) -> IBig {
        Self::apply_val(Digits::from_slice(operand))
    }

    #[inline]
    fn apply_val(mut operand: Digits) -> IBig {
        ibig_core::not(&mut operand);
        IBig::from_digits(operand)
    }
}

impl_unary_operator!(IBig, Not::not, NotOperation);

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
        ibig_core::bitand_same_len(&mut digits, &rhs[..n]);
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
        ibig_core::bitand_same_len(&mut lhs, &rhs[..n]);
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
