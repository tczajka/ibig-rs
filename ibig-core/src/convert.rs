//! Helpers for normalizing digit-slice representations.

use crate::Digit;

/// Given a non-empty little-endian slice of unsigned digits, returns the minimum number of
/// digits needed to represent the value.
///
/// This is the length with the most-significant zero digits removed, but always at least 1
/// (the value zero needs one digit).
#[inline]
pub fn min_len(digits: &[Digit]) -> usize {
    let mut len = digits.len();
    while len > 1 && digits[len - 1] == Digit::ZERO {
        len -= 1;
    }
    len
}

/// Given a non-empty little-endian slice of two's complement digits, returns the minimum
/// number of digits needed to represent the value.
///
/// This is the length with redundant most-significant sign-extension digits removed (a top
/// digit that merely repeats the sign of the digit below it), but always at least 1.
#[inline]
pub fn min_len_signed(digits: &[Digit]) -> usize {
    let mut len = digits.len();
    while len > 1 && digits[len - 1] == sign_extension(digits[len - 2]) {
        len -= 1;
    }
    len
}

/// The digit that sign-extends `digit`: all-ones if `digit` is negative, zero otherwise.
#[inline]
fn sign_extension(digit: Digit) -> Digit {
    if digit.cast_signed().is_negative() {
        Digit::MAX
    } else {
        Digit::ZERO
    }
}
