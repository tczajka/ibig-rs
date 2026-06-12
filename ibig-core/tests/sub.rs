//! Integration tests for subtraction.

use ibig_core::{Digit, add, sub, sub_1, sub_borrow, sub_digit, sub_same_len};
use proptest::collection::vec;
use proptest::prelude::*;

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn sub_basic() {
    let mut a = [digit(4), digit(9), digit(3)];
    assert!(!sub(&mut a, &[digit(3), digit(7)]));
    assert_eq!(a, [digit(1), digit(2), digit(3)]);

    // A borrow out of the low digits propagates through the high digits.
    let mut a = [Digit::ZERO, Digit::ZERO, digit(4)];
    assert!(!sub(&mut a, &[digit(1)]));
    assert_eq!(a, [Digit::MAX, Digit::MAX, digit(3)]);

    // A borrow out of the most-significant digit.
    let mut a = [Digit::ZERO, Digit::ZERO];
    assert!(sub(&mut a, &[digit(1), Digit::ZERO]));
    assert_eq!(a, [Digit::MAX, Digit::MAX]);

    // An empty `rhs` is allowed.
    let mut a = [digit(5)];
    assert!(!sub(&mut a, &[]));
    assert_eq!(a, [digit(5)]);
}

#[test]
#[should_panic]
fn sub_rhs_longer() {
    sub(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn sub_same_len_basic() {
    let mut a = [digit(4), digit(6)];
    assert!(!sub_same_len(&mut a, &[digit(3), digit(4)]));
    assert_eq!(a, [digit(1), digit(2)]);

    // The borrow propagates across digits and out the top.
    let mut a = [Digit::ZERO, Digit::ZERO];
    assert!(sub_same_len(&mut a, &[digit(1), Digit::ZERO]));
    assert_eq!(a, [Digit::MAX, Digit::MAX]);

    // Empty slices are allowed.
    assert!(!sub_same_len(&mut [], &[]));
}

#[test]
#[should_panic]
fn sub_same_len_mismatched() {
    sub_same_len(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn sub_digit_basic() {
    let mut a = [digit(8), digit(2)];
    assert!(!sub_digit(&mut a, digit(7)));
    assert_eq!(a, [digit(1), digit(2)]);

    let mut a = [Digit::ZERO, digit(4)];
    assert!(!sub_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::MAX, digit(3)]);

    let mut a = [Digit::ZERO, Digit::ZERO];
    assert!(sub_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::MAX, Digit::MAX]);
}

#[test]
#[should_panic]
fn sub_digit_empty() {
    sub_digit(&mut [], digit(1));
}

#[test]
fn sub_borrow_basic() {
    let mut a = [Digit::ZERO, digit(1)];
    assert!(!sub_borrow(&mut a, true));
    assert_eq!(a, [Digit::MAX, digit(0)]);

    // Without an incoming borrow nothing changes.
    let mut a = [Digit::ZERO];
    assert!(!sub_borrow(&mut a, false));
    assert_eq!(a, [Digit::ZERO]);

    // A borrow out of the most-significant digit.
    let mut a = [Digit::ZERO, Digit::ZERO];
    assert!(sub_borrow(&mut a, true));
    assert_eq!(a, [Digit::MAX, Digit::MAX]);

    // An empty slice passes the borrow through.
    assert!(sub_borrow(&mut [], true));
    assert!(!sub_borrow(&mut [], false));
}

#[test]
fn sub_1_basic() {
    let mut a = [digit(2), digit(2)];
    assert!(!sub_1(&mut a));
    assert_eq!(a, [digit(1), digit(2)]);

    // The decrement ripples through all-zeros digits.
    let mut a = [Digit::ZERO, Digit::ZERO, digit(4)];
    assert!(!sub_1(&mut a));
    assert_eq!(a, [Digit::MAX, Digit::MAX, digit(3)]);

    // All zeros overflows.
    let mut a = [Digit::ZERO, Digit::ZERO];
    assert!(sub_1(&mut a));
    assert_eq!(a, [Digit::MAX, Digit::MAX]);

    // An empty slice overflows immediately.
    assert!(sub_1(&mut []));
}

proptest! {
    // Subtraction undoes addition: a + b - b == a, and the borrow cancels the carry.
    #[test]
    fn add_sub_roundtrip(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let (longer, shorter) = if a.len() >= b.len() { (a, b) } else { (b, a) };

        let mut sum = longer.clone();
        let carry = add(&mut sum, &shorter);
        let borrow = sub(&mut sum, &shorter);
        prop_assert_eq!(sum, longer);
        prop_assert_eq!(borrow, carry);
    }

    // `sub` with a zero-extended `rhs` agrees with `sub_same_len`.
    #[test]
    fn sub_matches_sub_same_len(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let (mut longer, shorter) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let mut padded = shorter.clone();
        padded.resize(longer.len(), Digit::ZERO);

        let mut same_len = longer.clone();
        let borrow = sub(&mut longer, &shorter);
        let same_len_borrow = sub_same_len(&mut same_len, &padded);
        prop_assert_eq!(longer, same_len);
        prop_assert_eq!(borrow, same_len_borrow);
    }

    // `sub_digit` agrees with `sub` of a one-digit slice.
    #[test]
    fn sub_digit_matches_sub(a in vec(any::<Digit>(), 1..20), d: Digit) {
        let mut via_digit = a.clone();
        let mut via_sub = a;
        let borrow_digit = sub_digit(&mut via_digit, d);
        let borrow_sub = sub(&mut via_sub, &[d]);
        prop_assert_eq!(via_digit, via_sub);
        prop_assert_eq!(borrow_digit, borrow_sub);
    }

    // `sub_borrow` agrees with `sub_digit` of 0 or 1.
    #[test]
    fn sub_borrow_matches_sub_digit(a in vec(any::<Digit>(), 1..20), borrow: bool) {
        let mut via_borrow = a.clone();
        let mut via_digit = a;
        let borrow_out = sub_borrow(&mut via_borrow, borrow);
        let digit_borrow_out = sub_digit(&mut via_digit, Digit::from(u8::from(borrow)));
        prop_assert_eq!(via_borrow, via_digit);
        prop_assert_eq!(borrow_out, digit_borrow_out);
    }
}
