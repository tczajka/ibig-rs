//! Addition.

use crate::sub::sub_1;
use crate::{Digit, SignedDigit, sign_extension};

/// Adds `rhs` to `lhs` in place, returning the carry out of the most-significant digit.
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
/// # use ibig_core::{Digit, add};
/// let mut a = [Digit::MAX, Digit::from(2u8)];
/// let carry = add(&mut a, &[Digit::from(1u8)]);
/// assert_eq!(a, [Digit::ZERO, Digit::from(3u8)]);
/// assert!(!carry);
/// ```
#[inline]
pub fn add(lhs: &mut [Digit], rhs: &[Digit]) -> bool {
    let (low, high) = lhs.split_at_mut(rhs.len());
    let carry = add_same_len(low, rhs);
    add_carry(high, carry)
}

/// Adds `rhs` to `lhs` in place, returning the carry out of the most-significant digit.
/// The slices must have the same length.
///
/// # Panics
///
/// Panics if `lhs` and `rhs` have different lengths.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, add_same_len};
/// let mut a = [Digit::MAX, Digit::MAX];
/// let carry = add_same_len(&mut a, &[Digit::from(1u8), Digit::ZERO]);
/// assert_eq!(a, [Digit::ZERO, Digit::ZERO]);
/// assert!(carry);
/// ```
pub fn add_same_len(lhs: &mut [Digit], rhs: &[Digit]) -> bool {
    assert_eq!(lhs.len(), rhs.len());
    let mut carry = false;
    for (l, r) in lhs.iter_mut().zip(rhs) {
        let (sum, new_carry) = l.carrying_add(*r, carry);
        *l = sum;
        carry = new_carry;
    }
    carry
}

/// Adds a single digit to the non-empty `lhs` in place, returning the carry out of the
/// most-significant digit.
///
/// # Panics
///
/// Panics if `lhs` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, add_digit};
/// let mut a = [Digit::MAX, Digit::from(7u8)];
/// let carry = add_digit(&mut a, Digit::from(1u8));
/// assert_eq!(a, [Digit::ZERO, Digit::from(8u8)]);
/// assert!(!carry);
/// ```
#[inline]
pub fn add_digit(lhs: &mut [Digit], rhs: Digit) -> bool {
    let (low, high) = lhs.split_first_mut().expect("lhs is empty");
    let (sum, carry) = low.overflowing_add(rhs);
    *low = sum;
    add_carry(high, carry)
}

/// Adds a carry (0 or 1) to `lhs` in place, returning the carry out of the most-significant
/// digit.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, add_carry};
/// let mut a = [Digit::MAX, Digit::ZERO];
/// let carry = add_carry(&mut a, true);
/// assert_eq!(a, [Digit::ZERO, Digit::from(1u8)]);
/// assert!(!carry);
///
/// // Without an incoming carry nothing changes.
/// let mut a = [Digit::MAX];
/// assert!(!add_carry(&mut a, false));
/// ```
#[inline]
pub fn add_carry(lhs: &mut [Digit], carry: bool) -> bool {
    carry && add_1(lhs)
}

/// Adds 1 to `lhs` in place, returning the carry out of the most-significant digit (`true`
/// exactly when `lhs` is all ones).
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, add_1};
/// let mut a = [Digit::MAX, Digit::ZERO];
/// let carry = add_1(&mut a);
/// assert_eq!(a, [Digit::ZERO, Digit::from(1u8)]);
/// assert!(!carry);
/// ```
pub fn add_1(lhs: &mut [Digit]) -> bool {
    for d in lhs.iter_mut() {
        let (sum, overflow) = d.overflowing_add(Digit::from(1u8));
        *d = sum;
        if !overflow {
            return false;
        }
    }
    true
}

/// Adds the signed `rhs` to the signed `lhs` in place, returning a sign digit (0 or -1) that
/// should be appended to `lhs`.
///
/// `rhs` must be non-empty and not longer than `lhs`.
///
/// # Panics
///
/// Panics if `rhs` is empty or longer than `lhs`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, SignedDigit, add_signed};
/// // -1 + -1 == -2
/// let mut a = [Digit::MAX];
/// let high = add_signed(&mut a, &[Digit::MAX]);
/// assert_eq!(a, [Digit::MAX - Digit::from(1u8)]);
/// assert_eq!(high, SignedDigit::from(-1i8));
/// ```
pub fn add_signed(lhs: &mut [Digit], rhs: &[Digit]) -> SignedDigit {
    let lhs_extension = sign_extension(lhs.last().expect("lhs is empty").cast_signed());
    let rhs_extension = sign_extension(rhs.last().expect("rhs is empty").cast_signed());
    let (low, high) = lhs.split_at_mut(rhs.len());
    let low_carry = SignedDigit::from(add_same_len(low, rhs)) + rhs_extension;
    let high_carry = if low_carry > SignedDigit::ZERO {
        SignedDigit::from(add_1(high))
    } else if low_carry < SignedDigit::ZERO {
        -SignedDigit::from(sub_1(high))
    } else {
        SignedDigit::ZERO
    };
    high_carry + lhs_extension
}
