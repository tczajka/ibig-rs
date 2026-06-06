//! Integration tests for bit operations.

use ibig_core::{
    Digit, bit, bit_signed, bit_width, count_ones, is_power_of_two, next_power_of_two_in_place,
    trailing_ones, trailing_zeros,
};

const fn digit(n: u16) -> Digit {
    Digit::from_u16(n)
}

const BITS: usize = Digit::BITS as usize;

#[test]
fn test_bit_width() {
    let cases: &[(&[Digit], usize)] = &[
        // Zero in any form needs no bits.
        (&[], 0),
        (&[digit(0)], 0),
        (&[digit(0), digit(0)], 0),
        // Small values.
        (&[digit(1)], 1),
        (&[digit(0b101)], 3),
        (&[Digit::MAX], BITS),
        // Most-significant zero digits don't count.
        (&[digit(5), digit(0)], 3),
        // A set bit in a higher digit, above a zero low digit.
        (&[digit(0), digit(1)], BITS + 1),
        (&[digit(0), Digit::MAX], 2 * BITS),
        (&[Digit::MAX, Digit::MAX], 2 * BITS),
    ];
    for &(digits, expected) in cases {
        assert_eq!(bit_width(digits), expected);
    }
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
    let cases: &[(&[Digit], usize)] = &[
        // Within the low digit.
        (&[digit(0b1100)], 2),
        (&[digit(1)], 0),
        (&[Digit::MAX], 0),
        // A zero low digit contributes a full digit, then count into the next.
        (&[digit(0), digit(0b100)], BITS + 2),
        (&[digit(0), digit(0), digit(1)], 2 * BITS),
        // All zeros (the value zero): the full width.
        (&[], 0),
        (&[digit(0), digit(0)], 2 * BITS),
    ];
    for &(digits, expected) in cases {
        assert_eq!(trailing_zeros(digits), expected);
    }
}

#[test]
fn test_trailing_ones() {
    let cases: &[(&[Digit], usize)] = &[
        // Within the low digit.
        (&[digit(0b1011)], 2),
        (&[digit(0)], 0),
        (&[Digit::MAX], BITS),
        // An all-ones low digit contributes a full digit, then count into the next.
        (&[Digit::MAX, digit(0b1011)], BITS + 2),
        (&[Digit::MAX, Digit::MAX, digit(0)], 2 * BITS),
        // All ones (no zero bit): the full width.
        (&[], 0),
        (&[Digit::MAX, Digit::MAX], 2 * BITS),
    ];
    for &(digits, expected) in cases {
        assert_eq!(trailing_ones(digits), expected);
    }
}

#[test]
fn test_count_ones() {
    let cases: &[(&[Digit], usize)] = &[
        (&[], 0),
        (&[digit(0)], 0),
        (&[digit(0b101)], 2),
        (&[Digit::MAX], BITS),
        // Counts across every digit, including most-significant zeros.
        (&[digit(0b11), digit(0b1)], 3),
        (&[Digit::MAX, digit(0)], BITS),
        (&[Digit::MAX, Digit::MAX], 2 * BITS),
    ];
    for &(digits, expected) in cases {
        assert_eq!(count_ones(digits), expected);
    }
}

#[test]
fn test_is_power_of_two() {
    let cases: &[(&[Digit], bool)] = &[
        // Zero and all-zero slices are not powers of two.
        (&[], false),
        (&[digit(0)], false),
        (&[digit(0), digit(0)], false),
        // Single-digit powers of two and non-powers.
        (&[digit(1)], true),
        (&[digit(8)], true),
        (&[digit(6)], false),
        (&[Digit::MAX], false),
        // The high bit of a digit is a power of two.
        (&[Digit::MAX / digit(2) + digit(1)], true),
        // A single set bit in a higher digit (lower digits zero).
        (&[digit(0), digit(0), digit(4)], true),
        // More than one set bit, across digits or within one.
        (&[digit(1), digit(1)], false),
        (&[digit(0), digit(3)], false),
        (&[digit(1), digit(0), digit(4)], false),
    ];
    for &(digits, expected) in cases {
        assert_eq!(is_power_of_two(digits), expected);
    }
}

#[test]
fn test_next_power_of_two_in_place() {
    let high_bit = Digit::from_u8(1) << (Digit::BITS - 1);

    // Empty slice rounds up to one and overflows.
    assert!(next_power_of_two_in_place(&mut []));

    // Zero rounds up to one.
    let mut d = [digit(0), digit(0)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(1), digit(0)]);

    // Already a power of two: unchanged.
    let mut d = [digit(1)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(1)]);
    let mut d = [digit(8)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(8)]);

    // Rounds up to the next power of two.
    let mut d = [digit(3)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(4)]);
    let mut d = [digit(5)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(8)]);

    // Multi-digit: [1, 1] rounds up to [0, 2].
    let mut d = [digit(1), digit(1)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(2)]);

    // Multi-digit power of two: unchanged.
    let mut d = [digit(0), digit(0), digit(0), digit(8)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(0), digit(0), digit(8)]);

    let mut d = [digit(0), high_bit];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), high_bit]);

    // Multi-digit non-power of two.
    let mut d = [digit(0), digit(1), digit(0), digit(8)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(0), digit(0), digit(16)]);

    let mut d = [digit(0), digit(1), high_bit, digit(0)];
    assert!(!next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(0), digit(0), digit(1)]);

    // Overflow: all-ones has no fitting next power of two, so the value becomes zero.
    let mut d = [Digit::MAX];
    assert!(next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0)]);
    let mut d = [Digit::MAX, Digit::MAX];
    assert!(next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(0)]);

    // Overflow boundary: the top bit is set but the value is not a power of two.
    let mut d = [digit(1), high_bit];
    assert!(next_power_of_two_in_place(&mut d));
    assert_eq!(d, [digit(0), digit(0)]);
}
