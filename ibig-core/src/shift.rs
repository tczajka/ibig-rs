//! Bit shifts.

use crate::{
    Digit,
    sign::{is_negative, sign_extension},
};

/// Shifts `digits` left by `shift` bits in place, returning the overflow.
///
/// `shift` must be less than [`Digit::BITS`]. The bits shifted out beyond the
/// most-significant digit are returned in the low `shift` bits of the overflow digit (its
/// high bits are zero).
///
/// # Panics
///
/// Panics if `shift >= Digit::BITS`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, shl_small};
/// let mut a = [Digit::from(0b101u8)];
/// let overflow = shl_small(&mut a, 2);
/// assert_eq!(a, [Digit::from(0b10100u8)]);
/// assert_eq!(overflow, Digit::ZERO);
///
/// // The top bit of the low digit is carried into the next digit.
/// let mut a = [Digit::MAX, Digit::ZERO];
/// let overflow = shl_small(&mut a, 1);
/// assert_eq!(a, [!Digit::from(1u8), Digit::from(1u8)]);
/// assert_eq!(overflow, Digit::ZERO);
/// ```
#[inline]
pub fn shl_small(digits: &mut [Digit], shift: u32) -> Digit {
    assert!(shift < Digit::BITS);
    if shift == 0 {
        return Digit::ZERO;
    }
    let mut carry = Digit::ZERO;
    for d in digits.iter_mut() {
        let new_carry = *d >> (Digit::BITS - shift);
        *d = (*d << shift) | carry;
        carry = new_carry;
    }
    carry
}

/// Shifts `digits` right by `shift` bits in place. The bits shifted out below the
/// least-significant digit are discarded.
///
/// `shift` must be less than [`Digit::BITS`].
///
/// # Panics
///
/// Panics if `shift >= Digit::BITS`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, shr_small};
/// let mut a = [Digit::from(0b10100u8)];
/// shr_small(&mut a, 2);
/// assert_eq!(a, [Digit::from(0b101u8)]);
///
/// // The high bits of the low digit are pulled down from the next digit.
/// let mut a = [Digit::ZERO, Digit::from(1u8)];
/// shr_small(&mut a, 1);
/// assert_eq!(a, [Digit::from(1u8) << (Digit::BITS - 1), Digit::ZERO]);
/// ```
#[inline]
pub fn shr_small(digits: &mut [Digit], shift: u32) {
    assert!(shift < Digit::BITS);
    if shift == 0 {
        return;
    }
    let mut carry = Digit::ZERO;
    for d in digits.iter_mut().rev() {
        let new_carry = *d << (Digit::BITS - shift);
        *d = (*d >> shift) | carry;
        carry = new_carry;
    }
}

/// Shifts the signed two's complement value in `digits` left by `shift` bits in place,
/// returning the sign-extended overflow.
///
/// `shift` must be less than [`Digit::BITS`]. Like [`shl_small`], the low `shift` bits of the
/// returned digit are the bits shifted out beyond the most-significant digit; unlike it, the
/// high `BITS - shift` bits are filled with the sign of the value. The `(len + 1)`-digit
/// two's complement number formed by the shifted digits followed by the overflow equals the
/// original value shifted left by `shift`.
///
/// # Panics
///
/// Panics if `shift >= Digit::BITS`, or if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, shl_small_signed};
/// let mut a = [Digit::MAX]; // -1
/// let overflow = shl_small_signed(&mut a, 1);
/// assert_eq!(a, [!Digit::from(1u8)]); // -2
/// assert_eq!(overflow, Digit::MAX); // sign-extension of -1
/// ```
#[inline]
pub fn shl_small_signed(digits: &mut [Digit], shift: u32) -> Digit {
    assert!(shift < Digit::BITS);
    let negative = is_negative(digits);
    let overflow = shl_small(digits, shift);
    // Fill the high `BITS - shift` bits of the overflow with the sign.
    overflow | (sign_extension(negative) << shift)
}

/// Shifts the signed two's complement value in `digits` right by `shift` bits in place (an
/// arithmetic shift, sign-extending). The bits shifted out below the least-significant digit
/// are discarded.
///
/// `shift` must be less than [`Digit::BITS`]. The vacated high bits of the most-significant
/// digit are filled with the sign of the value.
///
/// # Panics
///
/// Panics if `shift >= Digit::BITS`, or if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, shr_small_signed};
/// let mut a = [!Digit::from(3u8)]; // -4
/// shr_small_signed(&mut a, 1);
/// assert_eq!(a, [!Digit::from(1u8)]); // -2
/// ```
#[inline]
pub fn shr_small_signed(digits: &mut [Digit], shift: u32) {
    assert!(shift < Digit::BITS);
    if shift == 0 {
        return;
    }
    // The bits shifted into the top of the most-significant digit are the sign extension.
    let mut carry = sign_extension(is_negative(digits)) << (Digit::BITS - shift);
    for d in digits.iter_mut().rev() {
        let new_carry = *d << (Digit::BITS - shift);
        *d = (*d >> shift) | carry;
        carry = new_carry;
    }
}
