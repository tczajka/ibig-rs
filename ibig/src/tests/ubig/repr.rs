//! Tests of the `UBig` internal representation.

use crate::{Digits, UBig};
use ibig_core::Digit;
use smallvec::{SmallVec, smallvec};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn from_digit() {
    // Every value, including zero, is a single inline digit.
    assert_eq!(UBig::from_digit(Digit::ZERO).as_digits(), &[Digit::ZERO]);
    assert_eq!(UBig::from_digit(digit(42)).as_digits(), &[digit(42)]);
    assert!(!UBig::from_digit(digit(42)).into_digits().spilled());
}

#[test]
fn from_digits_normalizes() {
    // Empty and all-zero buffers normalize to the single digit `[0]`.
    assert_eq!(UBig::from_digits(smallvec![]).as_digits(), &[Digit::ZERO]);
    assert_eq!(
        UBig::from_digits(smallvec![Digit::ZERO, Digit::ZERO]).as_digits(),
        &[Digit::ZERO]
    );
    // Most-significant zero digits are stripped.
    assert_eq!(
        UBig::from_digits(smallvec![digit(7), Digit::ZERO]).as_digits(),
        &[digit(7)]
    );
    assert_eq!(
        UBig::from_digits(smallvec![digit(1), digit(2), Digit::ZERO]).as_digits(),
        &[digit(1), digit(2)]
    );
}

#[test]
fn from_digits_inlines_small() {
    // A spilled buffer that normalizes to few digits is moved back inline, including
    // a value that collapses to a single digit.
    let mut digits: Digits = SmallVec::with_capacity(100);
    digits.push(digit(5));
    digits.push(Digit::ZERO);
    assert!(digits.spilled());
    let n = UBig::from_digits(digits);
    assert_eq!(n.try_to_digit(), Some(digit(5)));
    assert!(!n.into_digits().spilled());
}

#[test]
fn from_digits_shrinks_capacity() {
    // A heap buffer far larger than its contents is compacted but stays on the heap.
    let mut digits: Digits = SmallVec::with_capacity(100);
    for i in 1..=5u8 {
        digits.push(digit(i));
    }
    let buf = UBig::from_digits(digits).into_digits();
    assert_eq!(buf.len(), 5);
    assert!(buf.spilled());
    assert!(buf.capacity() < 100);
}

#[test]
fn try_to_digit() {
    assert_eq!(
        UBig::from_digit(Digit::ZERO).try_to_digit(),
        Some(Digit::ZERO)
    );
    assert_eq!(UBig::from_digit(digit(9)).try_to_digit(), Some(digit(9)));
    assert_eq!(
        UBig::from_digits(smallvec![digit(1), digit(2)]).try_to_digit(),
        None
    );
}
