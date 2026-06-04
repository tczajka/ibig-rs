//! Bitwise operations between digit slices.

use crate::Digit;

/// Computes the bitwise AND of two equal-length little-endian digit slices, storing the
/// result in `a`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, and_same_len_in_place};
/// let mut a = [Digit::from(0b1100u8), Digit::MAX];
/// and_same_len_in_place(&mut a, &[Digit::from(0b1010u8), Digit::ZERO]);
/// assert_eq!(a, [Digit::from(0b1000u8), Digit::ZERO]);
/// ```
pub fn and_same_len_in_place(a: &mut [Digit], b: &[Digit]) {
    assert_eq!(a.len(), b.len());
    for (x, &y) in a.iter_mut().zip(b) {
        *x &= y;
    }
}
