//! Integration tests for `IBig` bit operations.

use ibig::IBig;

#[test]
fn bit() {
    // Single-digit non-negative (fast path).
    let a = IBig::from(0b10010i8);
    assert!(!a.bit(0));
    assert!(a.bit(1));
    assert!(a.bit(4));
    assert!(!a.bit(5));
    assert!(!a.bit(1000));

    // Single-digit negative (fast path): -1 is all ones.
    let neg1 = IBig::from(-1i8);
    assert!(neg1.bit(0));
    assert!(neg1.bit(1000));

    // -2 = ...11111110.
    let neg2 = IBig::from(-2i8);
    assert!(!neg2.bit(0));
    assert!(neg2.bit(1));
    assert!(neg2.bit(1000));

    // Multi-digit non-negative (slow path): `u64::MAX` gains a sign-zero high digit.
    let big = IBig::from(u64::MAX);
    assert!(big.bit(0));
    assert!(big.bit(63));
    assert!(!big.bit(64));
    assert!(!big.bit(1000));

    // Multi-digit negative (slow path): -(2^100) is zero below bit 100, all ones at and above.
    let neg = IBig::from(-1i128 << 100);
    assert!(!neg.bit(0));
    assert!(!neg.bit(99));
    assert!(neg.bit(100));
    assert!(neg.bit(200));
    assert!(neg.bit(1000));
}
