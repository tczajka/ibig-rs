//! Sign and sign-extension of two's complement digit slices.

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

/// Sign-extends the two's complement value held in `digits[..len]` to fill the rest of
/// `digits` in place: every digit from index `len` onward is set to the value's sign
/// (all-ones if negative, zero otherwise).
///
/// # Panics
///
/// Panics if `len` is 0 or greater than `digits.len()`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, extend_signed};
/// // -1 occupies the low digit; extend it across the buffer.
/// let mut digits = [Digit::MAX, Digit::ZERO, Digit::ZERO];
/// extend_signed(&mut digits, 1);
/// assert_eq!(digits, [Digit::MAX, Digit::MAX, Digit::MAX]);
/// // A non-negative value extends with zeros.
/// let mut digits = [Digit::from(5u8), Digit::MAX];
/// extend_signed(&mut digits, 1);
/// assert_eq!(digits, [Digit::from(5u8), Digit::ZERO]);
/// ```
#[inline]
pub fn extend_signed(digits: &mut [Digit], len: usize) {
    let fill = sign_extension(is_negative(&digits[..len]));
    digits[len..].fill(fill);
}

/// The sign-extension digit for a value of the given sign: all-ones if negative, zero
/// otherwise.
#[inline]
pub(crate) const fn sign_extension(is_negative: bool) -> Digit {
    if is_negative { Digit::MAX } else { Digit::ZERO }
}

/// The sign-extension byte for a value of the given sign: all-ones if negative, zero
/// otherwise.
#[inline]
pub(crate) const fn sign_extension_byte(is_negative: bool) -> u8 {
    if is_negative { u8::MAX } else { 0 }
}
