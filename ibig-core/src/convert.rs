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

/// Given a little-endian slice of bytes, returns the minimum number of bytes needed to
/// represent the value: the length with the most-significant zero bytes removed.
///
/// This is 0 for the value zero (an empty slice, or a slice of only zero bytes). It is the
/// byte analogue of [`min_len`].
///
/// # Examples
///
/// ```
/// # use ibig_core::min_len_bytes;
/// assert_eq!(min_len_bytes(&[]), 0);
/// assert_eq!(min_len_bytes(&[0, 0]), 0);
/// assert_eq!(min_len_bytes(&[5, 0]), 1);
/// assert_eq!(min_len_bytes(&[0, 1]), 2);
/// ```
#[inline]
pub fn min_len_bytes(bytes: &[u8]) -> usize {
    let mut len = bytes.len();
    while len > 0 && bytes[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Writes the little-endian byte representation of the unsigned value held in `digits` into
/// `bytes`, one digit per `Digit::BYTES` bytes.
///
/// `bytes.len()` must equal `digits.len() * Digit::BYTES`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, to_bytes};
/// let mut bytes = vec![0u8; Digit::BYTES];
/// to_bytes(&[Digit::from(0x0102u16)], &mut bytes);
/// assert_eq!(&bytes[..2], &[0x02, 0x01]);
/// assert!(bytes[2..].iter().all(|&b| b == 0));
/// ```
pub fn to_bytes(digits: &[Digit], bytes: &mut [u8]) {
    assert_eq!(bytes.len(), digits.len() * Digit::BYTES);
    let (chunks, _) = bytes.as_chunks_mut::<{ Digit::BYTES }>();
    for (chunk, &digit) in chunks.iter_mut().zip(digits) {
        *chunk = digit.to_le_bytes();
    }
}

/// Fills `digits` from the little-endian `bytes`.
///
/// `digits.len()` must equal `bytes.len().div_ceil(Digit::BYTES)`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, from_bytes};
/// // Two bytes fit in a single digit at every digit width.
/// let mut digits = [Digit::ZERO];
/// from_bytes(&[0x02, 0x01], &mut digits);
/// assert_eq!(digits, [Digit::from(0x0102u16)]);
/// ```
pub fn from_bytes(bytes: &[u8], digits: &mut [Digit]) {
    assert_eq!(digits.len(), bytes.len().div_ceil(Digit::BYTES));
    let mut digit_iter = digits.iter_mut();
    let (chunks, rem) = bytes.as_chunks::<{ Digit::BYTES }>();
    for &chunk in chunks {
        *digit_iter.next().unwrap() = Digit::from_le_bytes(chunk);
    }
    if !rem.is_empty() {
        let mut arr = [0u8; Digit::BYTES];
        arr[..rem.len()].copy_from_slice(rem);
        *digit_iter.next().unwrap() = Digit::from_le_bytes(arr);
    }
    assert!(digit_iter.next().is_none());
}

/// Fills `digits` from the big-endian `bytes`.
///
/// `digits.len()` must equal `bytes.len().div_ceil(Digit::BYTES)`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, from_be_bytes};
/// // Two bytes fit in a single digit at every digit width.
/// let mut digits = [Digit::ZERO; 1];
/// from_be_bytes(&[0x01, 0x02], &mut digits);
/// assert_eq!(digits, [Digit::from(0x0102u16)]);
/// ```
pub fn from_be_bytes(bytes: &[u8], digits: &mut [Digit]) {
    assert_eq!(digits.len(), bytes.len().div_ceil(Digit::BYTES));
    let mut digit_iter = digits.iter_mut();
    let (rem, chunks) = bytes.as_rchunks::<{ Digit::BYTES }>();
    for &chunk in chunks.iter().rev() {
        *digit_iter.next().unwrap() = Digit::from_be_bytes(chunk);
    }
    if !rem.is_empty() {
        let mut arr = [0u8; Digit::BYTES];
        arr[Digit::BYTES - rem.len()..].copy_from_slice(rem);
        *digit_iter.next().unwrap() = Digit::from_be_bytes(arr);
    }
    assert!(digit_iter.next().is_none());
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
