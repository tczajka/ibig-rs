//! Integration tests for `UBig` bit operations.

use ibig::UBig;

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
