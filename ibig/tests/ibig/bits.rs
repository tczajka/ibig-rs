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

#[test]
fn set_bit() {
    // Set and clear within a digit.
    let mut a = IBig::from(0b100i8);
    a.set_bit(0, true);
    assert_eq!(a, IBig::from(0b101i8));
    a.set_bit(2, false);
    assert_eq!(a, IBig::from(0b001i8));

    // Setting a digit's most-significant bit of a positive value must not flip the sign.
    // Bit 63 is the top bit of a digit at every word size (64-1 = 2*32-1 = 4*16-1).
    let mut p = IBig::ZERO;
    p.set_bit(63, true);
    assert!(p.is_positive());
    assert!(p.bit(63));
    assert!(!p.bit(64));

    // Setting a bit far above a positive value: 5 -> 5 + 2^100, still positive.
    let mut q = IBig::from(5i8);
    q.set_bit(100, true);
    assert!(q.is_positive());
    assert!(q.bit(0));
    assert!(q.bit(2));
    assert!(q.bit(100));
    assert!(!q.bit(101));

    // -1 is all ones; clearing bit 0 gives -2.
    let mut n = IBig::from(-1i8);
    n.set_bit(0, false);
    assert_eq!(n, IBig::from(-2i8));

    // Clearing a digit's top bit of a negative value keeps it negative.
    let mut m = IBig::from(-1i8);
    m.set_bit(63, false);
    assert!(m.is_negative());
    assert!(!m.bit(63));
    assert!(m.bit(64)); // still sign-extended ones above

    // Setting a high bit of a negative value (already 1) is a no-op.
    let mut r = IBig::from(-5i8);
    r.set_bit(1000, true);
    assert_eq!(r, IBig::from(-5i8));

    // Clearing a high bit of a non-negative value (already 0) is a no-op.
    let mut z = IBig::from(7i8);
    z.set_bit(1000, false);
    assert_eq!(z, IBig::from(7i8));

    // A bit below the top digit of a multi-digit value: modified in place, no growth.
    // `u64::MAX` is multi-digit at every word size (it gains a sign-zero high digit).
    let mut big = IBig::from(u64::MAX);
    big.set_bit(0, false);
    assert_eq!(big, IBig::from(u64::MAX - 1));
    big.set_bit(0, true);
    assert_eq!(big, IBig::from(u64::MAX));
}
