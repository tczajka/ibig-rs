//! Integration tests for bitwise operations between digit slices.

use ibig_core::{Digit, and_same_len_in_place};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_and_same_len_in_place() {
    let mut a = [digit(0b1100), Digit::MAX, digit(0)];
    and_same_len_in_place(&mut a, &[digit(0b1010), digit(0b1111), Digit::MAX]);
    assert_eq!(a, [digit(0b1000), digit(0b1111), digit(0)]);

    // Empty slices are allowed.
    let mut a: [Digit; 0] = [];
    and_same_len_in_place(&mut a, &[]);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn test_and_same_len_in_place_mismatched() {
    and_same_len_in_place(&mut [Digit::ZERO], &[Digit::ZERO, Digit::ZERO]);
}
