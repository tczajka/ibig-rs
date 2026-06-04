//! Integration tests for `UBig` and `IBig` bitwise operators.

use ibig::{IBig, UBig};

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

#[test]
fn bitand() {
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
