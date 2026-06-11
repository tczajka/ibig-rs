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
use ibig_core::{BitIndex, Digit, SignedDigit};

/// Left shift.
enum ShlOperation {}

/// Right shift.
enum ShrOperation {}

/// Panics if shifting a value of `len` digits left by `whole` whole digits is certain to exceed
/// [`MAX_DIGITS`] (the result has at least `whole + len` digits).
#[inline]
fn check_shl_len(whole: usize, len: usize) {
    // `len <= MAX_DIGITS` already holds for any valid value, so the subtraction can't underflow.
    if whole > MAX_DIGITS - len {
        number_too_large();
    }
}

impl BinaryOp<UBig, usize> for ShlOperation {
    #[inline]
    fn apply_ref_ref(lhs: &UBig, rhs: &usize) -> UBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &UBig, rhs: usize) -> UBig {
        match lhs.as_digits() {
            Small(d) => shl_ubig_digit(d, rhs),
            Large(digits) => shl_ubig_fresh(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: UBig, rhs: &usize) -> UBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: UBig, rhs: usize) -> UBig {
        match lhs.into_digits() {
            Small(d) => shl_ubig_digit(d, rhs),
            Large(mut buffer) => {
                let index = BitIndex::from(rhs);
                let whole = index.digit_index();
                check_shl_len(whole, buffer.len());
                // Shift the bits in place, then prepend the whole zero digits.
                let overflow = ibig_core::shl_small(&mut buffer, index.bit_index());
                buffer.push(overflow);
                if whole != 0 {
                    buffer.insert_many(0, repeat_n(Digit::ZERO, whole));
                }
                UBig::from_digits(buffer)
            }
        }
    }
}

impl_binary_operator!(UBig, usize, Shl::shl, ShlAssign::shl_assign, ShlOperation);

/// Shifts a single-digit [`UBig`] left by `rhs` bits. The shifted digit spans at most two digits,
/// sitting above `rhs / DIGIT_BITS` prepended zero digits.
fn shl_ubig_digit(d: Digit, rhs: usize) -> UBig {
    // Shifting zero is zero (and avoids allocating `rhs / DIGIT_BITS` zero digits).
    if d == Digit::ZERO {
        return UBig::ZERO;
    }
    let index = BitIndex::from(rhs);
    let whole = index.digit_index();
    let small = index.bit_index(); // < Digit::BITS, the only shift `overflowing_shl` accepts
    check_shl_len(whole, 1);
    let mut buffer = Digits::with_capacity(whole + 2);
    buffer.resize(whole, Digit::ZERO);
    if small == 0 {
        buffer.push(d);
    } else {
        buffer.push(d.overflowing_shl(small).0); // low digit
        buffer.push(d >> (Digit::BITS - small)); // high digit
    }
    UBig::from_digits(buffer)
}

/// Shifts `digits` left by `rhs` bits into a freshly allocated buffer.
fn shl_ubig_fresh(digits: &[Digit], rhs: usize) -> UBig {
    let index = BitIndex::from(rhs);
    let whole = index.digit_index();
    check_shl_len(whole, digits.len());
    let mut buffer = Digits::with_capacity(whole + digits.len() + 1);
    buffer.resize(whole, Digit::ZERO);
    buffer.extend_from_slice(digits);
    let overflow = ibig_core::shl_small(&mut buffer[whole..], index.bit_index());
    buffer.push(overflow);
    UBig::from_digits(buffer)
}

impl BinaryOp<IBig, usize> for ShlOperation {
    #[inline]
    fn apply_ref_ref(lhs: &IBig, rhs: &usize) -> IBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &IBig, rhs: usize) -> IBig {
        match lhs.as_digits() {
            Small(d) => shl_ibig_digit(d, rhs),
            Large(digits) => shl_ibig_fresh(digits, rhs),
        }
    }

    #[inline]
    fn apply_val_ref(lhs: IBig, rhs: &usize) -> IBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: IBig, rhs: usize) -> IBig {
        match lhs.into_digits() {
            Small(d) => shl_ibig_digit(d, rhs),
            Large(mut buffer) => {
                let index = BitIndex::from(rhs);
                let whole = index.digit_index();
                check_shl_len(whole, buffer.len());
                // Shift the bits in place (sign-extending the overflow), then prepend zeros.
                let overflow = ibig_core::shl_small_signed(&mut buffer, index.bit_index());
                buffer.push(overflow);
                if whole != 0 {
                    buffer.insert_many(0, repeat_n(Digit::ZERO, whole));
                }
                IBig::from_digits(buffer)
            }
        }
    }
}

impl_binary_operator!(IBig, usize, Shl::shl, ShlAssign::shl_assign, ShlOperation);

/// Shifts a single-digit [`IBig`] left by `rhs` bits. The shifted digit spans at most two digits,
/// sitting above `rhs / DIGIT_BITS` prepended zero digits.
fn shl_ibig_digit(d: SignedDigit, rhs: usize) -> IBig {
    if d == SignedDigit::ZERO {
        return IBig::ZERO;
    }
    let index = BitIndex::from(rhs);
    let whole = index.digit_index();
    let small = index.bit_index(); // < SignedDigit::BITS, the only shift `overflowing_shl` accepts
    check_shl_len(whole, 1);
    let mut buffer = Digits::with_capacity(whole + 2);
    buffer.resize(whole, Digit::ZERO);
    if small == 0 {
        buffer.push(d.cast_unsigned());
    } else {
        buffer.push(d.overflowing_shl(small).0.cast_unsigned()); // low digit
        // The high digit is the sign-extended overflow (arithmetic shift).
        buffer.push((d >> (SignedDigit::BITS - small)).cast_unsigned());
    }
    IBig::from_digits(buffer)
}

/// Shifts the signed two's complement `digits` left by `rhs` bits into a freshly allocated
/// buffer.
fn shl_ibig_fresh(digits: &[Digit], rhs: usize) -> IBig {
    let index = BitIndex::from(rhs);
    let whole = index.digit_index();
    check_shl_len(whole, digits.len());
    let mut buffer = Digits::with_capacity(whole + digits.len() + 1);
    buffer.resize(whole, Digit::ZERO);
    buffer.extend_from_slice(digits);
    // The sign-extended overflow keeps the top digit two's-complement correct.
    let overflow = ibig_core::shl_small_signed(&mut buffer[whole..], index.bit_index());
    buffer.push(overflow);
    IBig::from_digits(buffer)
}

impl BinaryOp<UBig, usize> for ShrOperation {
    #[inline]
    fn apply_ref_ref(lhs: &UBig, rhs: &usize) -> UBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &UBig, rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        let whole = index.digit_index();
        match lhs.as_digits() {
            Small(d) if whole == 0 => UBig::from_digit(d >> index.bit_index()),
            Small(_) => UBig::ZERO,
            Large(digits) if whole >= digits.len() => UBig::ZERO,
            Large(digits) => {
                // Copy only the surviving high digits.
                let mut buffer = Digits::from_slice(&digits[whole..]);
                ibig_core::shr_small(&mut buffer, index.bit_index());
                UBig::from_digits(buffer)
            }
        }
    }

    #[inline]
    fn apply_val_ref(lhs: UBig, rhs: &usize) -> UBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: UBig, rhs: usize) -> UBig {
        let index = BitIndex::from(rhs);
        let whole = index.digit_index();
        match lhs.into_digits() {
            Small(d) if whole == 0 => UBig::from_digit(d >> index.bit_index()),
            Small(_) => UBig::ZERO,
            Large(buffer) if whole >= buffer.len() => UBig::ZERO,
            Large(mut buffer) => {
                buffer.drain(..whole);
                ibig_core::shr_small(&mut buffer, index.bit_index());
                UBig::from_digits(buffer)
            }
        }
    }
}

impl_binary_operator!(UBig, usize, Shr::shr, ShrAssign::shr_assign, ShrOperation);

impl BinaryOp<IBig, usize> for ShrOperation {
    #[inline]
    fn apply_ref_ref(lhs: &IBig, rhs: &usize) -> IBig {
        Self::apply_ref_val(lhs, *rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &IBig, rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        let whole = index.digit_index();
        match lhs.as_digits() {
            Small(d) => {
                // Beyond `BITS - 1`, an arithmetic shift of one digit saturates to the sign.
                let small = rhs.min((SignedDigit::BITS - 1) as usize) as u32;
                IBig::from_digit(d >> small)
            }
            Large(digits) if whole >= digits.len() => shr_ibig_sign(digits),
            Large(digits) => {
                // Copy only the surviving high digits.
                let mut buffer = Digits::from_slice(&digits[whole..]);
                ibig_core::shr_small_signed(&mut buffer, index.bit_index());
                IBig::from_digits(buffer)
            }
        }
    }

    #[inline]
    fn apply_val_ref(lhs: IBig, rhs: &usize) -> IBig {
        Self::apply_val_val(lhs, *rhs)
    }

    #[inline]
    fn apply_val_val(lhs: IBig, rhs: usize) -> IBig {
        let index = BitIndex::from(rhs);
        let whole = index.digit_index();
        match lhs.into_digits() {
            Small(d) => {
                // Beyond `BITS - 1`, an arithmetic shift of one digit saturates to the sign.
                let small = rhs.min((SignedDigit::BITS - 1) as usize) as u32;
                IBig::from_digit(d >> small)
            }
            Large(buffer) if whole >= buffer.len() => shr_ibig_sign(&buffer),
            Large(mut buffer) => {
                buffer.drain(..whole);
                ibig_core::shr_small_signed(&mut buffer, index.bit_index());
                IBig::from_digits(buffer)
            }
        }
    }
}

impl_binary_operator!(IBig, usize, Shr::shr, ShrAssign::shr_assign, ShrOperation);

/// The result of right-shifting the two's complement `digits` past all of its digits: the sign,
/// `0` or `-1`.
#[inline]
fn shr_ibig_sign(digits: &[Digit]) -> IBig {
    let fill = ibig_core::sign_extension(*digits.last().unwrap());
    IBig::from_digit(fill.cast_signed())
}
