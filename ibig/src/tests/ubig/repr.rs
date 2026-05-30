//! Tests of the `UBig` internal representation.

use crate::UBig;
use crate::ubig::Repr::{Large, Small};
use alloc::vec;
use ibig_core::Digit;

#[test]
fn test_from_digits_small() {
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
fn test_from_digits_large() {
    // A multi-digit value stays `Large`, with the leading zero removed.
    let n = UBig::from_digits(vec![Digit::from(1u8), Digit::from(2u8), Digit::ZERO]);
    assert_eq!(n.repr(), &Large(vec![Digit::from(1u8), Digit::from(2u8)]));
}
