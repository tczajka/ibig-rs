//! Integration tests for bitwise operations between digit slices.

use ibig_core::{Digit, bitand_same_len, not};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_bitand_same_len() {
    let mut a = [digit(0b1100), Digit::MAX, digit(0)];
    bitand_same_len(&mut a, &[digit(0b1010), digit(0b1111), Digit::MAX]);
    assert_eq!(a, [digit(0b1000), digit(0b1111), digit(0)]);

    // Empty slices are allowed.
    let mut a: [Digit; 0] = [];
    bitand_same_len(&mut a, &[]);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn test_bitand_same_len_mismatched() {
    bitand_same_len(&mut [Digit::ZERO], &[Digit::ZERO, Digit::ZERO]);
}

#[test]
fn test_not() {
    let mut a = [digit(0b1100), Digit::ZERO, Digit::MAX];
    not(&mut a);
    assert_eq!(a, [!digit(0b1100), Digit::MAX, Digit::ZERO]);

    // Empty slice is allowed.
    let mut empty: [Digit; 0] = [];
    not(&mut empty);
    assert_eq!(empty, []);
}
