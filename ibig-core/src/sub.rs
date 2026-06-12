//! Subtraction.

use crate::Digit;

/// Subtracts `rhs` from `lhs` in place, returning the borrow out of the most-significant digit.
///
/// `rhs` must not be longer than `lhs`.
///
/// # Panics
///
/// Panics if `rhs` is longer than `lhs`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sub};
/// let mut a = [Digit::ZERO, Digit::from(3u8)];
/// let borrow = sub(&mut a, &[Digit::from(1u8)]);
/// assert_eq!(a, [Digit::MAX, Digit::from(2u8)]);
/// assert!(!borrow);
/// ```
#[inline]
pub fn sub(lhs: &mut [Digit], rhs: &[Digit]) -> bool {
    let (low, high) = lhs.split_at_mut(rhs.len());
    let borrow = sub_same_len(low, rhs);
    sub_borrow(high, borrow)
}

/// Subtracts `rhs` from `lhs` in place, returning the borrow out of the most-significant digit.
/// The slices must have the same length.
///
/// # Panics
///
/// Panics if `lhs` and `rhs` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sub_same_len};
/// let mut a = [Digit::ZERO, Digit::ZERO];
/// let borrow = sub_same_len(&mut a, &[Digit::from(1u8), Digit::ZERO]);
/// assert_eq!(a, [Digit::MAX, Digit::MAX]);
/// assert!(borrow);
/// ```
pub fn sub_same_len(lhs: &mut [Digit], rhs: &[Digit]) -> bool {
    assert_eq!(lhs.len(), rhs.len());
    let mut borrow = false;
    for (l, r) in lhs.iter_mut().zip(rhs) {
        let (diff, new_borrow) = l.borrowing_sub(*r, borrow);
        *l = diff;
        borrow = new_borrow;
    }
    borrow
}

/// Subtracts a single digit from the non-empty `lhs` in place, returning the borrow out of the
/// most-significant digit.
///
/// # Panics
///
/// Panics if `lhs` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sub_digit};
/// let mut a = [Digit::ZERO, Digit::from(8u8)];
/// let borrow = sub_digit(&mut a, Digit::from(1u8));
/// assert_eq!(a, [Digit::MAX, Digit::from(7u8)]);
/// assert!(!borrow);
/// ```
#[inline]
pub fn sub_digit(lhs: &mut [Digit], rhs: Digit) -> bool {
    let (low, high) = lhs.split_first_mut().expect("lhs is empty");
    let (diff, borrow) = low.overflowing_sub(rhs);
    *low = diff;
    sub_borrow(high, borrow)
}

/// Subtracts a borrow (0 or 1) from `lhs` in place, returning the borrow out of the
/// most-significant digit.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sub_borrow};
/// let mut a = [Digit::ZERO, Digit::from(1u8)];
/// let borrow = sub_borrow(&mut a, true);
/// assert_eq!(a, [Digit::MAX, Digit::ZERO]);
/// assert!(!borrow);
///
/// // Without an incoming borrow nothing changes.
/// let mut a = [Digit::ZERO];
/// assert!(!sub_borrow(&mut a, false));
/// ```
#[inline]
pub fn sub_borrow(lhs: &mut [Digit], borrow: bool) -> bool {
    borrow && sub_1(lhs)
}

/// Subtracts 1 from `lhs` in place, returning the borrow out of the most-significant digit
/// (`true` exactly when `lhs` is all zeros).
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, sub_1};
/// let mut a = [Digit::ZERO, Digit::from(1u8)];
/// let borrow = sub_1(&mut a);
/// assert_eq!(a, [Digit::MAX, Digit::ZERO]);
/// assert!(!borrow);
/// ```
pub fn sub_1(lhs: &mut [Digit]) -> bool {
    for d in lhs.iter_mut() {
        let (diff, overflow) = d.overflowing_sub(Digit::from(1u8));
        *d = diff;
        if !overflow {
            return false;
        }
    }
    true
}
