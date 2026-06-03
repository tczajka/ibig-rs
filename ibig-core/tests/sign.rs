//! Integration tests for sign operations.

use ibig_core::{Digit, is_negative};

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
