//! Integration tests for `UBig`/`IBig` shift operators.

// These tests deliberately pass `&usize` operands to exercise the by-reference operator impls.
#![allow(clippy::op_ref)]

use ibig::proptest::{ibig_up_to_bits, ubig_up_to_bits};
use ibig::{IBig, UBig};
use proptest::prelude::*;

proptest! {
    // Shifting a `UBig` left by `n` then right by `n` restores the original value, checking
    // every owned/borrowed and assigning operand form along the way.
    #[test]
    fn ubig_shl_shr_round_trip(x in ubig_up_to_bits(300), n in 0usize..300) {
        let shl = &x << n;
        prop_assert_eq!(x.clone() << n, shl.clone());
        prop_assert_eq!(x.clone() << &n, shl.clone());
        prop_assert_eq!(&x << &n, shl.clone());
        let mut t = x.clone();
        t <<= n;
        prop_assert_eq!(&t, &shl);
        let mut t = x.clone();
        t <<= &n;
        prop_assert_eq!(&t, &shl);

        let shr = &shl >> n;
        prop_assert_eq!(shl.clone() >> n, shr.clone());
        prop_assert_eq!(shl.clone() >> &n, shr.clone());
        prop_assert_eq!(&shl >> &n, shr.clone());
        let mut t = shl.clone();
        t >>= n;
        prop_assert_eq!(&t, &shr);
        let mut t = shl.clone();
        t >>= &n;
        prop_assert_eq!(&t, &shr);

        prop_assert_eq!(shr, x);
    }

    // Same for `IBig`: `<<` multiplies by `2^n`, and the arithmetic `>>` divides it back.
    #[test]
    fn ibig_shl_shr_round_trip(x in ibig_up_to_bits(300), n in 0usize..300) {
        let shl = &x << n;
        prop_assert_eq!(x.clone() << n, shl.clone());
        prop_assert_eq!(x.clone() << &n, shl.clone());
        prop_assert_eq!(&x << &n, shl.clone());
        let mut t = x.clone();
        t <<= n;
        prop_assert_eq!(&t, &shl);
        let mut t = x.clone();
        t <<= &n;
        prop_assert_eq!(&t, &shl);

        let shr = &shl >> n;
        prop_assert_eq!(shl.clone() >> n, shr.clone());
        prop_assert_eq!(shl.clone() >> &n, shr.clone());
        prop_assert_eq!(&shl >> &n, shr.clone());
        let mut t = shl.clone();
        t >>= n;
        prop_assert_eq!(&t, &shr);
        let mut t = shl.clone();
        t >>= &n;
        prop_assert_eq!(&t, &shr);

        prop_assert_eq!(shr, x);
    }

    // `UBig` shifts match the corresponding primitive shifts, across every operand form.
    #[test]
    fn ubig_vs_primitive(a: u64, n in 0usize..64) {
        let x = UBig::from(a);
        let shl = UBig::from((a as u128) << n);
        let shr = UBig::from(a >> n);

        prop_assert_eq!(x.clone() << n, shl.clone());
        prop_assert_eq!(x.clone() << &n, shl.clone());
        prop_assert_eq!(&x << n, shl.clone());
        prop_assert_eq!(&x << &n, shl.clone());
        let mut t = x.clone();
        t <<= n;
        prop_assert_eq!(&t, &shl);
        let mut t = x.clone();
        t <<= &n;
        prop_assert_eq!(&t, &shl);

        prop_assert_eq!(x.clone() >> n, shr.clone());
        prop_assert_eq!(x.clone() >> &n, shr.clone());
        prop_assert_eq!(&x >> n, shr.clone());
        prop_assert_eq!(&x >> &n, shr.clone());
        let mut t = x.clone();
        t >>= n;
        prop_assert_eq!(&t, &shr);
        let mut t = x.clone();
        t >>= &n;
        prop_assert_eq!(&t, &shr);
    }

    // `IBig` shifts match the corresponding primitive (arithmetic) shifts, across every form.
    #[test]
    fn ibig_vs_primitive(a: i64, n in 0usize..63) {
        let x = IBig::from(a);
        let shl = IBig::from((a as i128) << n);
        let shr = IBig::from(a >> n);

        prop_assert_eq!(x.clone() << n, shl.clone());
        prop_assert_eq!(x.clone() << &n, shl.clone());
        prop_assert_eq!(&x << n, shl.clone());
        prop_assert_eq!(&x << &n, shl.clone());
        let mut t = x.clone();
        t <<= n;
        prop_assert_eq!(&t, &shl);
        let mut t = x.clone();
        t <<= &n;
        prop_assert_eq!(&t, &shl);

        prop_assert_eq!(x.clone() >> n, shr.clone());
        prop_assert_eq!(x.clone() >> &n, shr.clone());
        prop_assert_eq!(&x >> n, shr.clone());
        prop_assert_eq!(&x >> &n, shr.clone());
        let mut t = x.clone();
        t >>= n;
        prop_assert_eq!(&t, &shr);
        let mut t = x.clone();
        t >>= &n;
        prop_assert_eq!(&t, &shr);
    }
}

#[test]
fn shl_basic() {
    assert_eq!(UBig::from(1u8) << 0, UBig::from(1u8));
    assert_eq!(UBig::from(1u8) << 10, UBig::from(1024u16));
    assert_eq!(IBig::from(-1i8) << 4, IBig::from(-16i8));
}

#[test]
fn shr_basic() {
    assert_eq!(UBig::from(1024u16) >> 10, UBig::from(1u8));
    assert_eq!(UBig::from(1u8) >> 1000, UBig::ZERO);
    // Arithmetic right shift rounds toward negative infinity.
    assert_eq!(IBig::from(-1i8) >> 100, IBig::from(-1i8));
    assert_eq!(IBig::from(-3i8) >> 1, IBig::from(-2i8));
    assert_eq!(IBig::from(8i8) >> 2, IBig::from(2i8));
    // A single-digit shift past the digit width saturates to the sign.
    assert_eq!(IBig::from(-5i8) >> 1000, IBig::from(-1i8));
    assert_eq!(IBig::from(5i8) >> 1000, IBig::ZERO);
    assert_eq!(IBig::from(-5i8) >> usize::MAX, IBig::from(-1i8));
    assert_eq!(UBig::from(5u8) >> usize::MAX, UBig::ZERO);
}

#[test]
fn shl_zero_huge() {
    // Shifting zero never allocates, so even an enormous shift is fine.
    assert_eq!(UBig::ZERO << usize::MAX, UBig::ZERO);
    assert_eq!(&UBig::ZERO << usize::MAX, UBig::ZERO);
    assert_eq!(IBig::ZERO << usize::MAX, IBig::ZERO);
    assert_eq!(&IBig::ZERO << usize::MAX, IBig::ZERO);
}

#[test]
#[should_panic(expected = "number too large")]
fn shl_overflow_ubig() {
    let _ = UBig::from(1u8) << usize::MAX;
}

#[test]
#[should_panic(expected = "number too large")]
fn shl_overflow_ibig() {
    let _ = IBig::from(1u8) << usize::MAX;
}
