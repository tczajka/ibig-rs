//! Integration tests for sign operations.

use ibig_core::{Digit, extend_signed, is_negative};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_is_negative() {
    // The sign is the most-significant digit's high bit.
    assert!(!is_negative(&[digit(0)]));
    assert!(!is_negative(&[digit(5)]));
    assert!(is_negative(&[Digit::MAX])); // -1
    // A negative top digit makes the whole value negative regardless of lower digits.
    assert!(is_negative(&[digit(5), Digit::MAX]));
    // A non-negative top digit keeps the value non-negative, even across digits.
    assert!(!is_negative(&[Digit::MAX, digit(0)]));
}

#[test]
#[should_panic]
fn test_is_negative_empty() {
    is_negative(&[]);
}

#[test]
fn test_extend_signed() {
    // A negative value extends with all-ones.
    let mut digits = [Digit::MAX, digit(0), digit(0)];
    extend_signed(&mut digits, 1);
    assert_eq!(digits, [Digit::MAX, Digit::MAX, Digit::MAX]);
    // A non-negative value extends with zeros.
    let mut digits = [digit(5), Digit::MAX, Digit::MAX];
    extend_signed(&mut digits, 1);
    assert_eq!(digits, [digit(5), digit(0), digit(0)]);
    // The sign comes from the most-significant digit of the value part.
    let mut digits = [digit(1), Digit::MAX, digit(0)];
    extend_signed(&mut digits, 2);
    assert_eq!(digits, [digit(1), Digit::MAX, Digit::MAX]);
    // `len == digits.len()` leaves the buffer unchanged.
    let mut digits = [digit(7), digit(8)];
    extend_signed(&mut digits, 2);
    assert_eq!(digits, [digit(7), digit(8)]);
}

#[test]
#[should_panic]
fn test_extend_signed_empty() {
    extend_signed(&mut [Digit::ZERO], 0);
}
