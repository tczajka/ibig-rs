//! Bitwise operations between digit slices.

use crate::Digit;

/// Negates (bitwise NOT) a little-endian digit slice in place, flipping every bit.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, not};
/// let mut a = [Digit::from(0b1100u8), Digit::ZERO];
/// not(&mut a);
/// assert_eq!(a, [!Digit::from(0b1100u8), Digit::MAX]);
/// ```
pub fn not(a: &mut [Digit]) {
    for x in a {
        *x = !*x;
    }
}

/// Computes the bitwise AND of two equal-length digit slices, storing the
/// result in `a`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bitand_same_len};
/// let mut a = [Digit::from(0b1100u8), Digit::MAX];
/// bitand_same_len(&mut a, &[Digit::from(0b1010u8), Digit::ZERO]);
/// assert_eq!(a, [Digit::from(0b1000u8), Digit::ZERO]);
/// ```
pub fn bitand_same_len(a: &mut [Digit], b: &[Digit]) {
    assert_eq!(a.len(), b.len());
    for (x, &y) in a.iter_mut().zip(b) {
        *x &= y;
    }
}

/// Computes the bitwise OR of two equal-length digit slices, storing the
/// result in `a`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bitor_same_len};
/// let mut a = [Digit::from(0b1100u8), Digit::ZERO];
/// bitor_same_len(&mut a, &[Digit::from(0b1010u8), Digit::MAX]);
/// assert_eq!(a, [Digit::from(0b1110u8), Digit::MAX]);
/// ```
pub fn bitor_same_len(a: &mut [Digit], b: &[Digit]) {
    assert_eq!(a.len(), b.len());
    for (x, &y) in a.iter_mut().zip(b) {
        *x |= y;
    }
}

/// Computes the bitwise XOR of two equal-length digit slices, storing the
/// result in `a`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bitxor_same_len};
/// let mut a = [Digit::from(0b1100u8), Digit::MAX];
/// bitxor_same_len(&mut a, &[Digit::from(0b1010u8), Digit::MAX]);
/// assert_eq!(a, [Digit::from(0b0110u8), Digit::ZERO]);
/// ```
pub fn bitxor_same_len(a: &mut [Digit], b: &[Digit]) {
    assert_eq!(a.len(), b.len());
    for (x, &y) in a.iter_mut().zip(b) {
        *x ^= y;
    }
}

/// Computes the bitwise AND-NOT (`a & !b`) of two equal-length digit slices, storing the
/// result in `a`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bitandnot_same_len};
/// let mut a = [Digit::from(0b1100u8), Digit::MAX];
/// bitandnot_same_len(&mut a, &[Digit::from(0b1010u8), Digit::from(0b11u8)]);
/// assert_eq!(a, [Digit::from(0b0100u8), !Digit::from(0b11u8)]);
/// ```
pub fn bitandnot_same_len(a: &mut [Digit], b: &[Digit]) {
    assert_eq!(a.len(), b.len());
    for (x, &y) in a.iter_mut().zip(b) {
        *x &= !y;
    }
}
