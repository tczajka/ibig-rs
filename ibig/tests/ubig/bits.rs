//! Integration tests for `UBig` bit operations.

use ibig::UBig;

/// `2^k` as a `UBig`.
fn pow2(k: usize) -> UBig {
    let mut bytes = vec![0u8; k / 8 + 1];
    bytes[k / 8] = 1u8 << (k % 8);
    UBig::from_le_bytes(&bytes)
}

#[test]
fn bit_width() {
    assert_eq!(UBig::ZERO.bit_width(), 0);
    assert_eq!(UBig::from(1u8).bit_width(), 1);
    assert_eq!(UBig::from(0b101u8).bit_width(), 3);
    assert_eq!(UBig::from(u128::MAX).bit_width(), 128);
    assert_eq!(UBig::from(1u128 << 100).bit_width(), 101);
}

#[test]
fn ilog2() {
    assert_eq!(UBig::from(1u8).ilog2(), 0);
    assert_eq!(UBig::from(0b101u8).ilog2(), 2);
    assert_eq!(UBig::from(u128::MAX).ilog2(), 127);
    assert_eq!(UBig::from(1u128 << 100).ilog2(), 100);
}

#[test]
#[should_panic]
fn ilog2_zero() {
    UBig::ZERO.ilog2();
}

#[test]
fn bit() {
    // Single digit (fast path).
    let a = UBig::from(0b10010u8);
    assert!(!a.bit(0));
    assert!(a.bit(1));
    assert!(a.bit(4));
    assert!(!a.bit(5));
    assert!(!a.bit(1000));

    // Multi-digit (slow path): 2^100.
    let p = UBig::from(1u128 << 100);
    assert!(!p.bit(0));
    assert!(!p.bit(99));
    assert!(p.bit(100));
    assert!(!p.bit(101));
    assert!(!p.bit(1000));

    // Low bits set across digit boundaries.
    let m = UBig::from(u128::MAX);
    assert!(m.bit(0));
    assert!(m.bit(127));
    assert!(!m.bit(128));
}

#[test]
fn set_bit() {
    // Set and clear within a digit.
    let mut a = UBig::from(0b100u8);
    a.set_bit(0, true);
    assert_eq!(a, UBig::from(0b101u8));
    a.set_bit(2, false);
    assert_eq!(a, UBig::from(0b001u8));

    // Setting a high bit grows the value: 0 -> 2^100.
    let mut b = UBig::ZERO;
    b.set_bit(100, true);
    assert_eq!(b, UBig::from(1u128 << 100));
    assert!(b.bit(100));

    // Clearing the only set bit yields zero.
    let mut c = UBig::from(1u8);
    c.set_bit(0, false);
    assert_eq!(c, UBig::ZERO);

    // Clearing a bit far above the value is a no-op.
    let mut d = UBig::from(5u8);
    d.set_bit(1000, false);
    assert_eq!(d, UBig::from(5u8));

    // Re-setting an already-set bit is idempotent; clearing across digits works.
    let mut e = UBig::from(u128::MAX);
    e.set_bit(0, true);
    assert_eq!(e, UBig::from(u128::MAX));
    e.set_bit(0, false);
    assert_eq!(e, UBig::from(u128::MAX - 1));
}

#[test]
fn trailing_zeros() {
    assert_eq!(UBig::from(1u8).trailing_zeros(), 0);
    assert_eq!(UBig::from(0b101000u8).trailing_zeros(), 3);
    // Multi-digit: 2^100 has 100 trailing zeros.
    assert_eq!(UBig::from(1u128 << 100).trailing_zeros(), 100);
}

#[test]
#[should_panic]
fn trailing_zeros_zero() {
    UBig::ZERO.trailing_zeros();
}

#[test]
fn trailing_ones() {
    assert_eq!(UBig::ZERO.trailing_ones(), 0);
    assert_eq!(UBig::from(0b100111u8).trailing_ones(), 3);
    // All bits of u128::MAX set; the result is its width, and it never panics (finite).
    assert_eq!(UBig::from(u128::MAX).trailing_ones(), 128);
}

#[test]
fn is_power_of_two() {
    let cases = [
        (UBig::ZERO, false),
        (UBig::from(1u8), true),
        (UBig::from(8u8), true),
        (UBig::from(6u8), false),
        // Multi-digit.
        (UBig::from(1u128 << 100), true),
        (UBig::from(3u128 << 100), false),
        (UBig::from(u128::MAX), false),
    ];
    for (n, expected) in cases {
        assert_eq!(n.is_power_of_two(), expected);
    }
}

#[test]
fn next_power_of_two() {
    // (value, smallest power of two >= value).
    let mut cases: Vec<(UBig, UBig)> = vec![
        (UBig::ZERO, UBig::from(1u8)),
        (UBig::from(1u8), UBig::from(1u8)),
        (UBig::from(5u8), UBig::from(8u8)),
        (UBig::from(8u8), UBig::from(8u8)),
        // A digit with the high bit set rounds up past the digit width (fast-path overflow on
        // 64-bit, where `u64::MAX` is a single digit).
        (UBig::from(u64::MAX), UBig::from(1u128 << 64)),
        // Multi-digit and growing past the current width.
        (UBig::from((1u128 << 100) + 1), UBig::from(1u128 << 101)),
        // u128::MAX rounds up to 2^128.
        (UBig::from(u128::MAX), pow2(128)),
    ];
    // Large exact powers of two round up to themselves; one more rounds to the next power.
    for k in [63usize, 64, 127, 128, 200, 255, 256, 500] {
        cases.push((pow2(k), pow2(k)));
        let mut above = pow2(k); // 2^k + 1 (bit 0 is clear for k > 0)
        above.set_bit(0, true);
        cases.push((above, pow2(k + 1)));
    }

    for (value, expected) in cases {
        assert_eq!(value.next_power_of_two(), expected);
        assert_eq!(value.into_next_power_of_two(), expected);
    }
}
