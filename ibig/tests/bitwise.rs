//! Integration tests for `UBig` and `IBig` bitwise operators.

use ibig::proptest::ibig_up_to_bits;
use ibig::{IBig, UBig};
use proptest::prelude::*;

#[test]
fn not() {
    // `!x == -x - 1`, so for any primitive `v`, `!IBig::from(v) == IBig::from(!v)`.
    let cases = [
        // Single-digit fast path (both signs, and the extremes that flip into each other).
        0i128,
        1,
        -1,
        2,
        -2,
        5,
        -6,
        0xff,
        -0x100,
        i64::MAX as i128,
        i64::MIN as i128,
        // Multi-digit (spans multiple digits at every word size).
        1 << 100,
        -1 << 100,
        (1 << 100) + 12345,
        i128::MIN,
        i128::MAX,
    ];

    for v in cases {
        let x = IBig::from(v);
        let expected = IBig::from(!v);
        assert_eq!(!&x, expected);
        assert_eq!(!x, expected);
    }
}

proptest! {
    // Bitwise NOT is an involution: `!!x == x`.
    #[test]
    fn ibig_not_not(x in ibig_up_to_bits(1000)) {
        prop_assert_eq!(!!&x, x);
    }
}

#[test]
fn ubig_bitand() {
    // `(a, b, a & b)` as little-endian byte slices.
    let cases: &[(&[u8], &[u8], &[u8])] = &[
        // Within a byte.
        (&[0b1100], &[0b1010], &[0b1000]),
        (&[0xff], &[0x0f], &[0x0f]),
        // AND with zero.
        (&[0xff, 0xff], &[], &[]),
        // Different lengths: the longer operand's high part drops out.
        (&[0xff, 0xff, 0xff], &[0x0f], &[0x0f]),
        // Multi-byte, spanning digits at every word size.
        (
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[0xff; 10],
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ),
        // Single digit and multi-digit with no common bits: the result shrinks to zero.
        (&[0, 0, 0, 0, 0, 0, 0, 0, 1], &[1], &[]),
        // Both multi-digit, overlapping bits.
        (&[0xff; 18], &[0x0f; 30], &[0x0f; 18]),
        // Both multi-digit, no overlapping bits.
        (&[0xf0; 18], &[0x0f; 30], &[]),
    ];

    for &(a_bytes, b_bytes, expected_bytes) in cases {
        let a = UBig::from_le_bytes(a_bytes);
        let b = UBig::from_le_bytes(b_bytes);
        let expected = UBig::from_le_bytes(expected_bytes);

        // Both orders (AND is commutative), each in all value/reference combinations.
        for (x, y) in [(&a, &b), (&b, &a)] {
            assert_eq!(x.clone() & y.clone(), expected);
            assert_eq!(x.clone() & y, expected);
            assert_eq!(x & y.clone(), expected);
            assert_eq!(x & y, expected);
        }
    }
}

#[test]
fn ibig_bitand() {
    // Two's complement AND: `IBig::from(a) & IBig::from(b) == IBig::from(a & b)`. Covers every
    // sign combination and both relative lengths (single/multi-digit at each word size).
    let cases: &[(i128, i128)] = &[
        (0, 0),
        (0b1100, 0b1010),
        (-1, 5), // -1 is all ones: `a & -1 == a`
        (-1, -1),
        (-2, -3), // negative & negative stays negative
        (5, -8),  // positive & negative
        (i64::MAX as i128, i64::MIN as i128),
        (1 << 100, -1),            // multi-digit positive & all-ones
        (1 << 100, (1 << 64) - 1), // multi-digit, different lengths
        (-(1 << 100), 0xff),       // multi-digit negative & small positive
        (-(1 << 100), -(1 << 64)), // multi-digit negative & negative
        (i128::MIN, i128::MAX),
        ((1 << 100) | 0xdead, (1 << 100) | 0xbeef),
    ];

    for &(a, b) in cases {
        let expected = IBig::from(a & b);
        let (x, y) = (IBig::from(a), IBig::from(b));
        // Both orders (AND is commutative), each in all value/reference combinations.
        for (p, q) in [(&x, &y), (&y, &x)] {
            assert_eq!(p.clone() & q.clone(), expected);
            assert_eq!(p.clone() & q, expected);
            assert_eq!(p & q.clone(), expected);
            assert_eq!(p & q, expected);

            let mut z = p.clone();
            z &= q.clone();
            assert_eq!(z, expected);
            let mut w = p.clone();
            w &= q;
            assert_eq!(w, expected);
        }
    }
}

#[test]
fn ibig_bitand_unequal_lengths() {
    // Regression: a negative operand with strictly fewer digits than the other, in the owned
    // position of a mixed owned/borrowed `&`. These values exceed `i128` (built from bytes) so
    // the digit counts genuinely differ at every word size.
    //
    // `-(2^70)` has bits 0..69 clear and all higher bits set, so:
    //   -(2^70) & 2^200  == 2^200      (only bit 200 survives)
    //   -(2^70) & -(2^200) == -(2^200) (bits below 200 cancel)
    let neg_short = IBig::from(-(1i128 << 70)); // negative, few digits

    let mut pos_bytes = [0u8; 26];
    pos_bytes[25] = 1; // +2^200, positive (top bit clear)
    let big_pos = IBig::from_le_bytes(&pos_bytes);

    let mut neg_bytes = [0u8; 26];
    neg_bytes[25] = 0xff; // -(2^200), negative (top bit set)
    let big_neg = IBig::from_le_bytes(&neg_bytes);

    for (b, expected) in [(&big_pos, &big_pos), (&big_neg, &big_neg)] {
        let expected = expected.clone();
        let a = &neg_short;
        // Both orders, all value/reference combinations, plus both assigning forms.
        for (p, q) in [(a, b), (b, a)] {
            assert_eq!(p.clone() & q.clone(), expected);
            assert_eq!(p.clone() & q, expected);
            assert_eq!(p & q.clone(), expected);
            assert_eq!(p & q, expected);

            let mut z = p.clone();
            z &= q.clone();
            assert_eq!(z, expected);
            let mut w = p.clone();
            w &= q;
            assert_eq!(w, expected);
        }
    }
}
