//! Integration tests for digit-slice normalization helpers.

use ibig_core::{Digit, min_len, min_len_signed};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_min_len() {
    // Zero needs no digits: empty and all-zero slices collapse to 0.
    assert_eq!(min_len(&[]), 0);
    assert_eq!(min_len(&[digit(0)]), 0);
    assert_eq!(min_len(&[digit(0), digit(0), digit(0)]), 0);
    // A nonzero value keeps up to its most-significant nonzero digit.
    assert_eq!(min_len(&[digit(5)]), 1);
    assert_eq!(min_len(&[digit(5), digit(0)]), 1);
    assert_eq!(min_len(&[digit(1), digit(2), digit(0)]), 2);
    // A zero low digit below a nonzero digit is kept.
    assert_eq!(min_len(&[digit(0), digit(1)]), 2);
    // Nothing to strip.
    assert_eq!(min_len(&[digit(1), digit(2), digit(3)]), 3);
}

#[test]
fn test_min_len_signed() {
    // Zero needs one digit.
    assert_eq!(min_len_signed(&[digit(0)]), 1);
    assert_eq!(min_len_signed(&[digit(0), digit(0)]), 1);
    // A redundant zero sign-extension above a non-negative digit is stripped.
    assert_eq!(min_len_signed(&[digit(5), digit(0)]), 1);
    assert_eq!(min_len_signed(&[digit(1), digit(2), digit(0), digit(0)]), 2);
    // -1 is all-ones; the redundant all-ones sign-extension digits are stripped.
    assert_eq!(min_len_signed(&[Digit::MAX]), 1);
    assert_eq!(min_len_signed(&[Digit::MAX, Digit::MAX, Digit::MAX]), 1);
    // A positive value whose top significant digit has its high bit set needs a leading
    // zero digit, which must be kept.
    assert_eq!(min_len_signed(&[Digit::MAX, digit(0)]), 2);
    // A negative value whose top significant digit has its high bit clear needs a leading
    // all-ones digit, which must be kept.
    assert_eq!(min_len_signed(&[digit(0), Digit::MAX]), 2);
    assert_eq!(min_len_signed(&[digit(5), Digit::MAX]), 2);
}

#[test]
#[should_panic]
fn test_min_len_signed_empty() {
    min_len_signed(&[]);
}
