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
