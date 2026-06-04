//! Integration tests for bit operations.

use ibig_core::{
    Digit, bit, bit_signed, bit_width, is_power_of_two, trailing_ones, trailing_zeros,
};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

const BITS: usize = Digit::BITS as usize;

#[test]
fn test_bit_width() {
    // Zero in any form needs no bits.
    assert_eq!(bit_width(&[]), 0);
    assert_eq!(bit_width(&[digit(0)]), 0);
    assert_eq!(bit_width(&[digit(0), digit(0)]), 0);
    // Small values.
    assert_eq!(bit_width(&[digit(1)]), 1);
    assert_eq!(bit_width(&[digit(0b101)]), 3);
    assert_eq!(bit_width(&[Digit::MAX]), BITS);
    // Most-significant zero digits don't count.
    assert_eq!(bit_width(&[digit(5), digit(0)]), 3);
    // A set bit in a higher digit, above a zero low digit.
    assert_eq!(bit_width(&[digit(0), digit(1)]), BITS + 1);
    assert_eq!(bit_width(&[digit(0), Digit::MAX]), 2 * BITS);
    assert_eq!(bit_width(&[Digit::MAX, Digit::MAX]), 2 * BITS);
}

#[test]
fn test_bit() {
    // 0b101 = 5.
    let d = [digit(0b101)];
    assert!(bit(&d, 0));
    assert!(!bit(&d, 1));
    assert!(bit(&d, 2));
    assert!(!bit(&d, 3));
    // The value is zero-extended above its bits.
    assert!(!bit(&d, BITS - 1));
    assert!(!bit(&d, BITS));
    assert!(!bit(&d, 1000));
    // An empty slice is the value zero: every bit is clear.
    assert!(!bit(&[], 0));
    assert!(!bit(&[], 1000));
    // Bits across a digit boundary: low digit all ones, high digit 0b10.
    let d2 = [Digit::MAX, digit(0b10)];
    assert!(bit(&d2, 0));
    assert!(bit(&d2, BITS - 1));
    assert!(!bit(&d2, BITS)); // bit 0 of 0b10
    assert!(bit(&d2, BITS + 1)); // bit 1 of 0b10
    assert!(!bit(&d2, BITS + 2));
}

#[test]
fn test_bit_signed() {
    // -1 is all ones, including every sign-extended position.
    assert!(bit_signed(&[Digit::MAX], 0));
    assert!(bit_signed(&[Digit::MAX], BITS - 1));
    assert!(bit_signed(&[Digit::MAX], BITS));
    assert!(bit_signed(&[Digit::MAX], 1000));
    // 0b101 is non-negative: the sign bit and everything above it are zero.
    let p = [digit(0b101)];
    assert!(bit_signed(&p, 0));
    assert!(!bit_signed(&p, 1));
    assert!(bit_signed(&p, 2));
    assert!(!bit_signed(&p, BITS - 1));
    assert!(!bit_signed(&p, BITS));
    assert!(!bit_signed(&p, 1000));
    // A negative two-digit value [0, MAX]: low digit zero, high digit all ones, sign negative.
    let neg = [digit(0), Digit::MAX];
    assert!(!bit_signed(&neg, 0));
    assert!(!bit_signed(&neg, BITS - 1));
    assert!(bit_signed(&neg, BITS));
    assert!(bit_signed(&neg, 2 * BITS - 1));
    assert!(bit_signed(&neg, 2 * BITS)); // sign-extended
    assert!(bit_signed(&neg, 1000));
}

#[test]
#[should_panic]
fn test_bit_signed_empty() {
    bit_signed(&[], 0);
}

#[test]
fn test_trailing_zeros() {
    // Within the low digit.
    assert_eq!(trailing_zeros(&[digit(0b1100)]), 2);
    assert_eq!(trailing_zeros(&[digit(1)]), 0);
    assert_eq!(trailing_zeros(&[Digit::MAX]), 0);
    // A zero low digit contributes a full digit, then count into the next.
    assert_eq!(trailing_zeros(&[digit(0), digit(0b100)]), BITS + 2);
    assert_eq!(trailing_zeros(&[digit(0), digit(0), digit(1)]), 2 * BITS);
    // All zeros (the value zero): the full width.
    assert_eq!(trailing_zeros(&[]), 0);
    assert_eq!(trailing_zeros(&[digit(0), digit(0)]), 2 * BITS);
}

#[test]
fn test_trailing_ones() {
    // Within the low digit.
    assert_eq!(trailing_ones(&[digit(0b1011)]), 2);
    assert_eq!(trailing_ones(&[digit(0)]), 0);
    assert_eq!(trailing_ones(&[Digit::MAX]), BITS);
    // An all-ones low digit contributes a full digit, then count into the next.
    assert_eq!(trailing_ones(&[Digit::MAX, digit(0b1011)]), BITS + 2);
    assert_eq!(trailing_ones(&[Digit::MAX, Digit::MAX, digit(0)]), 2 * BITS);
    // All ones: the full width.
    assert_eq!(trailing_ones(&[]), 0);
    assert_eq!(trailing_ones(&[Digit::MAX, Digit::MAX]), 2 * BITS);
}

#[test]
fn test_is_power_of_two() {
    // Zero and all-zero slices are not powers of two.
    assert!(!is_power_of_two(&[]));
    assert!(!is_power_of_two(&[digit(0)]));
    assert!(!is_power_of_two(&[digit(0), digit(0)]));
    // Single-digit powers of two and non-powers.
    assert!(is_power_of_two(&[digit(1)]));
    assert!(is_power_of_two(&[digit(8)]));
    assert!(!is_power_of_two(&[digit(6)]));
    assert!(!is_power_of_two(&[Digit::MAX]));
    // The high bit of a digit is a power of two.
    assert!(is_power_of_two(&[Digit::MAX / digit(2) + digit(1)]));
    // A single set bit in a higher digit (lower digits zero).
    assert!(is_power_of_two(&[digit(0), digit(0), digit(4)]));
    // More than one set bit, across digits or within one.
    assert!(!is_power_of_two(&[digit(1), digit(1)]));
    assert!(!is_power_of_two(&[digit(0), digit(3)]));
    assert!(!is_power_of_two(&[digit(1), digit(0), digit(4)]));
}
