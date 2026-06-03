//! Sign of a two's complement digit slice and the digit/byte that sign-extends it.

use crate::Digit;

/// Returns `true` if the non-empty little-endian slice of two's complement `digits`
/// represents a negative value (the most-significant digit's sign bit is set).
///
/// # Panics
///
/// Panics if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, is_negative};
/// assert!(is_negative(&[Digit::MAX])); // -1
/// assert!(!is_negative(&[Digit::from(5u8)])); // +5
/// assert!(!is_negative(&[Digit::MAX, Digit::ZERO])); // a positive multi-digit value
/// ```
#[inline]
pub const fn is_negative(digits: &[Digit]) -> bool {
    digits.last().unwrap().cast_signed().is_negative()
}

/// The sign-extension digit for a value of the given sign: all-ones if negative, zero
/// otherwise.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sign_extension};
/// assert_eq!(sign_extension(false), Digit::ZERO);
/// assert_eq!(sign_extension(true), Digit::MAX);
/// ```
#[inline]
pub const fn sign_extension(is_negative: bool) -> Digit {
    if is_negative { Digit::MAX } else { Digit::ZERO }
}

/// The sign-extension byte for a value of the given sign: all-ones if negative, zero
/// otherwise.
#[inline]
pub(crate) const fn sign_extension_byte(is_negative: bool) -> u8 {
    if is_negative { u8::MAX } else { 0 }
}
