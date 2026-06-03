//! Digit-slice conversions and normalization helpers.

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
pub const fn min_len(digits: &[Digit]) -> usize {
    let mut len = digits.len();
    while len > 0 && digits[len - 1].const_eq(Digit::ZERO) {
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
pub const fn min_len_signed(digits: &[Digit]) -> usize {
    assert!(!digits.is_empty());
    let mut len = digits.len();
    while len > 1
        && digits[len - 1].const_eq(sign_extension(digits[len - 2].cast_signed().is_negative()))
    {
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

/// Given a non-empty little-endian slice of two's complement bytes, returns the minimum
/// number of bytes needed to represent the value: the length with redundant
/// most-significant sign-extension bytes removed, but always at least 1.
///
/// This is the byte analogue of [`min_len_signed`].
///
/// # Panics
///
/// Panics if `bytes` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::min_len_bytes_signed;
/// // A redundant zero sign byte above a non-negative byte is dropped.
/// assert_eq!(min_len_bytes_signed(&[5, 0]), 1);
/// // -1 is all-ones; the redundant 0xff sign bytes are dropped.
/// assert_eq!(min_len_bytes_signed(&[0xff, 0xff]), 1);
/// // 0xc8 alone is negative, so a leading zero byte is needed to stay positive (200).
/// assert_eq!(min_len_bytes_signed(&[0xc8, 0x00]), 2);
/// ```
#[inline]
pub fn min_len_bytes_signed(bytes: &[u8]) -> usize {
    assert!(!bytes.is_empty());
    let mut len = bytes.len();
    while len > 1
        && bytes[len - 1] == sign_extension_byte(bytes[len - 2].cast_signed().is_negative())
    {
        len -= 1;
    }
    len
}

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

/// Writes the little-endian byte representation of the unsigned value held in `digits` into
/// `bytes`, zero-extending to fill `bytes`.
///
/// # Panics
///
/// Panics if `bytes.len() < digits.len() * Digit::BYTES`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, to_bytes};
/// let mut bytes = vec![0xffu8; 20];
/// to_bytes(&[Digit::from(0x0102u16)], &mut bytes);
/// assert_eq!(&bytes[..2], &[0x02, 0x01]);
/// assert!(bytes[2..].iter().all(|&b| b == 0));
/// ```
pub fn to_bytes(digits: &[Digit], bytes: &mut [u8]) {
    to_bytes_fill(digits, bytes, 0);
}

/// Writes the little-endian two's complement byte representation of the signed value held in
/// `digits` into `bytes`, sign-extending to fill `bytes`.
///
/// `bytes.len()` must be at least `digits.len() * Digit::BYTES`.
///
/// # Panics
///
/// Panics if `digits` is empty or `bytes.len() < digits.len() * Digit::BYTES`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, to_bytes_signed};
/// let mut bytes = vec![0u8; Digit::BYTES + 1];
/// to_bytes_signed(&[Digit::MAX], &mut bytes);
/// assert!(bytes.iter().all(|&b| b == 0xff));
/// ```
pub fn to_bytes_signed(digits: &[Digit], bytes: &mut [u8]) {
    to_bytes_fill(digits, bytes, sign_extension_byte(is_negative(digits)));
}

/// Writes `digits` little-endian into the low bytes of `bytes` and fills the rest with
/// `fill` (the sign-extension byte).
///
/// # Panics
///
/// Panics if `bytes.len() < digits.len() * Digit::BYTES`.
fn to_bytes_fill(digits: &[Digit], bytes: &mut [u8], fill: u8) {
    let (low, high) = bytes.split_at_mut(digits.len() * Digit::BYTES);
    let (chunks, _) = low.as_chunks_mut::<{ Digit::BYTES }>();
    for (chunk, &digit) in chunks.iter_mut().zip(digits) {
        *chunk = digit.to_le_bytes();
    }
    high.fill(fill);
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
pub const fn from_bytes(bytes: &[u8], digits: &mut [Digit]) {
    assert!(digits.len() == bytes.len().div_ceil(Digit::BYTES));
    let (chunks, rem) = bytes.as_chunks::<{ Digit::BYTES }>();
    let mut i = 0;
    while i < chunks.len() {
        digits[i] = Digit::from_le_bytes(chunks[i]);
        i += 1;
    }
    if !rem.is_empty() {
        let mut arr = [0u8; Digit::BYTES];
        let (dest, _) = arr.split_at_mut(rem.len());
        dest.copy_from_slice(rem);
        digits[i] = Digit::from_le_bytes(arr);
        i += 1;
    }
    assert!(i == digits.len());
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

/// Fills `digits` from the little-endian two's complement `bytes`.
///
/// `digits.len()` must equal `bytes.len().div_ceil(Digit::BYTES)`.
///
/// # Panics
///
/// Panics if `bytes` is empty: a signed value needs at least one byte to carry its sign.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, from_bytes_signed};
/// let mut digits = [Digit::ZERO; 1];
/// from_bytes_signed(&[1, 2], &mut digits);
/// assert_eq!(digits, [Digit::from(0x0201u16)]);
/// from_bytes_signed(&[0xff], &mut digits);
/// assert_eq!(digits, [Digit::MAX]);
/// ```
pub const fn from_bytes_signed(bytes: &[u8], digits: &mut [Digit]) {
    assert!(!bytes.is_empty());
    assert!(digits.len() == bytes.len().div_ceil(Digit::BYTES));

    let (chunks, rem) = bytes.as_chunks::<{ Digit::BYTES }>();
    let mut i = 0;
    while i < chunks.len() {
        digits[i] = Digit::from_le_bytes(chunks[i]);
        i += 1;
    }
    if !rem.is_empty() {
        let fill = sign_extension_byte(bytes.last().unwrap().cast_signed().is_negative());
        let mut arr = [fill; Digit::BYTES];
        let (dest, _) = arr.split_at_mut(rem.len());
        dest.copy_from_slice(rem);
        digits[i] = Digit::from_le_bytes(arr);
        i += 1;
    }
    assert!(i == digits.len());
}

/// Fills `digits` from the big-endian two's complement `bytes`, sign-extending the
/// most-significant digit.
///
/// `digits.len()` must equal `bytes.len().div_ceil(Digit::BYTES)`.
///
/// # Panics
///
/// Panics if `bytes` is empty: a signed value needs at least one byte to carry its sign.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, from_be_bytes_signed};
/// let mut digits = [Digit::ZERO; 1];
/// from_be_bytes_signed(&[1, 2], &mut digits);
/// assert_eq!(digits, [Digit::from(0x0102u16)]);
/// // 0xff is -1 in two's complement; it sign-extends to fill the digit with ones.
/// from_be_bytes_signed(&[0xff], &mut digits);
/// assert_eq!(digits, [Digit::MAX]);
/// ```
pub fn from_be_bytes_signed(bytes: &[u8], digits: &mut [Digit]) {
    assert!(!bytes.is_empty());
    assert_eq!(digits.len(), bytes.len().div_ceil(Digit::BYTES));
    let mut digit_iter = digits.iter_mut();
    let (rem, chunks) = bytes.as_rchunks::<{ Digit::BYTES }>();
    for &chunk in chunks.iter().rev() {
        *digit_iter.next().unwrap() = Digit::from_be_bytes(chunk);
    }
    if !rem.is_empty() {
        let fill = sign_extension_byte(bytes[0].cast_signed().is_negative());
        let mut arr = [fill; Digit::BYTES];
        arr[Digit::BYTES - rem.len()..].copy_from_slice(rem);
        *digit_iter.next().unwrap() = Digit::from_be_bytes(arr);
    }
    assert!(digit_iter.next().is_none());
}

/// The sign-extension byte for a value of the given sign: all-ones if negative, zero
/// otherwise.
#[inline]
const fn sign_extension_byte(is_negative: bool) -> u8 {
    if is_negative { u8::MAX } else { 0 }
}
