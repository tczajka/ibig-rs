//! Helpers for normalizing digit-slice representations.

use crate::Digit;

/// Given a little-endian slice of unsigned digits, returns the minimum number of digits
/// needed to represent the value.
///
/// This is the length with the most-significant zero digits removed. It is 0 for the value
/// zero (an empty slice, or a slice of only zero digits).
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, min_len};
/// assert_eq!(min_len(&[Digit::from(5u8), Digit::from(2u8)]), 2);
/// assert_eq!(min_len(&[Digit::from(5u8), Digit::ZERO]), 1);
/// assert_eq!(min_len(&[Digit::ZERO, Digit::ZERO]), 0);
/// ```
#[inline]
pub fn min_len(digits: &[Digit]) -> usize {
    let mut len = digits.len();
    while len > 0 && digits[len - 1] == Digit::ZERO {
        len -= 1;
    }
    len
}

/// Given a non-empty little-endian slice of two's complement digits, returns the minimum
/// number of digits needed to represent the value.
///
/// This is the length with redundant most-significant sign-extension digits removed (a top
/// digit that merely repeats the sign of the digit below it), but always at least 1 (a
/// signed value always needs at least one digit to carry its sign).
///
/// # Panics
///
/// Panics if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, min_len_signed};
/// // 5 is positive in a single digit, so the redundant zero sign-extension is dropped.
/// assert_eq!(min_len_signed(&[Digit::from(5u8), Digit::ZERO]), 1);
/// // -1 is all-ones; the redundant all-ones sign-extension digits are dropped.
/// assert_eq!(min_len_signed(&[Digit::MAX, Digit::MAX]), 1);
/// // The top digit's high bit being set means the leading zero digit is needed to keep
/// // this value positive, so it is not dropped.
/// assert_eq!(min_len_signed(&[Digit::MAX, Digit::ZERO]), 2);
/// ```
#[inline]
pub fn min_len_signed(digits: &[Digit]) -> usize {
    assert!(!digits.is_empty());
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
