//! Tests of the `UBig` internal representation.

use crate::UBig;
use crate::ubig::Repr::{Large, Small};
use alloc::vec;
use alloc::vec::Vec;
use ibig_core::Digit;

#[test]
fn from_digit() {
    // `from_digit` stores the value directly as `Small`, including zero.
    assert_eq!(
        UBig::from_digit(Digit::ZERO).into_repr(),
        Small(Digit::ZERO)
    );
    assert_eq!(
        UBig::from_digit(Digit::from(42u8)).into_repr(),
        Small(Digit::from(42u8))
    );
}

#[test]
fn from_digits_small() {
    // Empty becomes a single zero digit.
    assert_eq!(UBig::from_digits(vec![]).into_repr(), Small(Digit::ZERO));
    // A value fitting in one digit is stored as `Small`, dropping leading zeros.
    assert_eq!(
        UBig::from_digits(vec![Digit::from(7u8), Digit::ZERO]).into_repr(),
        Small(Digit::from(7u8))
    );
    assert_eq!(
        UBig::from_digits(vec![Digit::ZERO, Digit::ZERO]).into_repr(),
        Small(Digit::ZERO)
    );
}

#[test]
fn from_digits_large() {
    // A multi-digit value stays `Large`, with the leading zero removed.
    let n = UBig::from_digits(vec![Digit::from(1u8), Digit::from(2u8), Digit::ZERO]);
    assert_eq!(n.repr(), &Large(vec![Digit::from(1u8), Digit::from(2u8)]));
}

#[test]
fn from_digits_shrinks_capacity() {
    // A buffer more than 4x larger than its contents is compacted.
    let mut digits = Vec::with_capacity(100);
    digits.push(Digit::from(1u8));
    digits.push(Digit::from(2u8));
    match UBig::from_digits(digits).repr() {
        Large(v) => {
            assert_eq!(v.len(), 2);
            assert!(v.capacity() < 100);
        }
        Small(_) => panic!("expected Large"),
    }
}
