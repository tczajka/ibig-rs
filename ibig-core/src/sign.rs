//! Sign and sign-extension of two's complement digit and byte slices.

use crate::Digit;

/// Returns `true` if the non-empty two's complement `digits` represent a negative value (the
/// most-significant digit's sign bit is set).
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
    assert!(
        len > 0 && len <= digits.len(),
        "len must be in 1..=digits.len()"
    );
    let fill = sign_extension(digits[len - 1]);
    digits[len..].fill(fill);
}

/// Sign-extends the two's complement value held in `bytes[..len]` to fill the rest of `bytes`
/// in place: every byte from index `len` onward is set to the value's sign (all-ones if
/// negative, zero otherwise).
///
/// # Panics
///
/// Panics if `len` is 0 or greater than `bytes.len()`.
///
/// # Examples
///
/// ```
/// # use ibig_core::extend_signed_bytes;
/// // -1 occupies the low byte; extend it across the buffer.
/// let mut bytes = [0xffu8, 0, 0];
/// extend_signed_bytes(&mut bytes, 1);
/// assert_eq!(bytes, [0xff, 0xff, 0xff]);
/// // A non-negative value extends with zeros.
/// let mut bytes = [5u8, 0xff];
/// extend_signed_bytes(&mut bytes, 1);
/// assert_eq!(bytes, [5, 0]);
/// ```
#[inline]
pub fn extend_signed_bytes(bytes: &mut [u8], len: usize) {
    assert!(
        len > 0 && len <= bytes.len(),
        "len must be in 1..=bytes.len()"
    );
    let fill = sign_extension_byte(bytes[len - 1]);
    bytes[len..].fill(fill);
}

/// The sign-extension digit for a two's complement value whose most-significant digit is
/// `high`: all-ones (`Digit::MAX`) if `high` is negative (its sign bit is set), zero otherwise.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sign_extension};
/// assert_eq!(sign_extension(Digit::MAX), Digit::MAX); // a negative top digit
/// assert_eq!(sign_extension(Digit::from(5u8)), Digit::ZERO); // a non-negative top digit
/// ```
#[inline]
pub const fn sign_extension(high: Digit) -> Digit {
    // Smear the sign bit across the whole digit: arithmetic-shifting it down to every bit
    // yields all-ones for a negative `high` and all-zeros otherwise.
    high.cast_signed()
        .checked_shr(Digit::BITS - 1)
        .unwrap()
        .cast_unsigned()
}

/// The sign-extension byte for a two's complement value whose most-significant byte is `high`:
/// all-ones if `high` is negative (its sign bit is set), zero otherwise.
#[inline]
pub(crate) const fn sign_extension_byte(high: u8) -> u8 {
    (high.cast_signed() >> (u8::BITS - 1)).cast_unsigned()
}
