//! Tests of the `IBig` internal representation.

use crate::{Digits, IBig};
use ibig_core::{Digit, SignedDigit};
use smallvec::{SmallVec, smallvec};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

fn signed(n: i8) -> SignedDigit {
    SignedDigit::from(n)
}

#[test]
fn from_digit() {
    // A single signed digit is stored as its two's complement bit pattern, inline.
    assert_eq!(IBig::from_digit(signed(0)).as_digits(), &[digit(0)]);
    assert_eq!(IBig::from_digit(signed(42)).as_digits(), &[digit(42)]);
    // -1 is all-ones in two's complement.
    assert_eq!(IBig::from_digit(signed(-1)).as_digits(), &[Digit::MAX]);
    assert!(!IBig::from_digit(signed(-1)).into_digits().spilled());
}

#[test]
fn from_digits_normalizes() {
    assert_eq!(
        IBig::from_digits(smallvec![digit(0), digit(0)]).as_digits(),
        &[digit(0)]
    );
    // A redundant zero sign-extension above a non-negative digit is stripped.
    assert_eq!(
        IBig::from_digits(smallvec![digit(5), digit(0), digit(0)]).as_digits(),
        &[digit(5)]
    );
    // For a negative value the sign-extension digits are all-ones, and are stripped.
    assert_eq!(
        IBig::from_digits(smallvec![Digit::MAX, Digit::MAX, Digit::MAX]).as_digits(),
        &[Digit::MAX]
    );
}

#[test]
#[should_panic]
fn from_digits_panics_on_empty() {
    IBig::from_digits(smallvec![]);
}

#[test]
fn from_digits_keeps_needed_sign_digit() {
    // The unsigned value 2^W - 1 (all-ones in one digit) is negative as a single two's
    // complement digit, so representing it as a positive number needs a leading zero digit
    // that must not be stripped.
    assert_eq!(
        IBig::from_digits(smallvec![Digit::MAX, digit(0)]).as_digits(),
        &[Digit::MAX, digit(0)]
    );
    // Likewise a leading all-ones digit is needed below a non-negative digit to stay
    // negative.
    assert_eq!(
        IBig::from_digits(smallvec![digit(0), Digit::MAX]).as_digits(),
        &[digit(0), Digit::MAX]
    );
}

#[test]
fn from_digits_inlines_small() {
    // A spilled buffer that normalizes to few digits is moved back inline, including a
    // value that collapses to a single digit.
    let mut digits: Digits = SmallVec::with_capacity(100);
    digits.push(digit(5));
    digits.push(digit(0));
    assert!(digits.spilled());
    let n = IBig::from_digits(digits);
    assert_eq!(n.try_to_digit(), Some(signed(5)));
    assert!(!n.into_digits().spilled());
}

#[test]
fn from_digits_shrinks_capacity() {
    // A heap buffer far larger than its contents is compacted but stays on the heap.
    let mut digits: Digits = SmallVec::with_capacity(100);
    for i in 1..=5u8 {
        digits.push(digit(i));
    }
    let buf = IBig::from_digits(digits).into_digits();
    assert_eq!(buf.len(), 5);
    assert!(buf.spilled());
    assert!(buf.capacity() < 100);
}

#[test]
fn try_to_digit() {
    assert_eq!(IBig::from_digit(signed(0)).try_to_digit(), Some(signed(0)));
    assert_eq!(IBig::from_digit(signed(9)).try_to_digit(), Some(signed(9)));
    assert_eq!(
        IBig::from_digit(signed(-1)).try_to_digit(),
        Some(signed(-1))
    );
    // A value needing two digits does not fit in a single signed digit.
    assert_eq!(
        IBig::from_digits(smallvec![Digit::MAX, digit(0)]).try_to_digit(),
        None
    );
}
