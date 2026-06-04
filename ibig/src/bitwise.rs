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
        Self::apply_digit_digit(lhs[0], rhs)
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

impl CommutativeBinaryOpDigits<IBig> for BitAndOperation {
    #[inline]
    fn apply_digit_digit(lhs: SignedDigit, rhs: SignedDigit) -> IBig {
        IBig::from_digit(lhs & rhs)
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: SignedDigit) -> IBig {
        if rhs.is_negative() {
            // High digits are preserved.
            Self::apply_val_digit(Digits::from_slice(lhs), rhs)
        } else {
            // High digits are zeroed.
            Self::apply_digit_digit(lhs[0].cast_signed(), rhs)
        }
    }

    #[inline]
    fn apply_val_digit(mut lhs: Digits, rhs: SignedDigit) -> IBig {
        if rhs.is_negative() {
            // High digits are preserved.
            lhs[0] &= rhs.cast_unsigned();
            IBig::from_digits(lhs)
        } else {
            // High digits are zeroed.
            Self::apply_digit_digit(lhs[0].cast_signed(), rhs)
        }
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> IBig {
        let (longer, shorter) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        if ibig_core::is_negative(shorter) {
            // High digits are preserved: clone longer operand.
            Self::apply_val_ref(Digits::from_slice(longer), shorter)
        } else {
            // High digits are zeroed: clone shorter operand.
            Self::apply_val_ref(Digits::from_slice(shorter), longer)
        }
    }

    #[inline]
    fn apply_val_ref(mut lhs: Digits, rhs: &[Digit]) -> IBig {
        if lhs.len() >= rhs.len() {
            if !ibig_core::is_negative(rhs) {
                // Zero out high digits.
                lhs.truncate(rhs.len());
            }
            ibig_core::bitand_same_len(&mut lhs[..rhs.len()], rhs);
        } else {
            let (rhs_low, rhs_high) = rhs.split_at(lhs.len());
            let lhs_negative = ibig_core::is_negative(&lhs);
            ibig_core::bitand_same_len(&mut lhs, rhs_low);
            if lhs_negative {
                // Include high digits from `rhs`.
                lhs.extend_from_slice(rhs_high);
            }
        }
        IBig::from_digits(lhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> IBig {
        let (longer, shorter) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        if ibig_core::is_negative(&shorter) {
            // High digits are preserved: reuse storage from longer operand.
            Self::apply_val_ref(longer, &shorter)
        } else {
            // High digits are zeroed: reuse storage from shorter operand.
            Self::apply_val_ref(shorter, &longer)
        }
    }
}

impl_binary_operator!(
    IBig,
    BitAnd::bitand,
    BitAndAssign::bitand_assign,
    BitAndOperation
);
