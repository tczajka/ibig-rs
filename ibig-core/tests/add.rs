//! Integration tests for addition.

use ibig_core::{DIGIT_BITS_USIZE, Digit, add, add_carry, add_digit, add_same_len};
use proptest::collection::vec;
use proptest::prelude::*;

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn add_basic() {
    let mut a = [digit(1), digit(2), digit(3)];
    assert!(!add(&mut a, &[digit(3), digit(7)]));
    assert_eq!(a, [digit(4), digit(9), digit(3)]);

    // A carry out of the low digits propagates through the high digits.
    let mut a = [Digit::MAX, Digit::MAX, digit(3)];
    assert!(!add(&mut a, &[digit(1)]));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO, digit(4)]);

    // A carry out of the most-significant digit.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add(&mut a, &[digit(1), Digit::ZERO]));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // An empty `rhs` is allowed.
    let mut a = [digit(5)];
    assert!(!add(&mut a, &[]));
    assert_eq!(a, [digit(5)]);
}

#[test]
#[should_panic]
fn add_rhs_longer() {
    add(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn add_same_len_basic() {
    let mut a = [digit(1), digit(2)];
    assert!(!add_same_len(&mut a, &[digit(3), digit(4)]));
    assert_eq!(a, [digit(4), digit(6)]);

    // The carry propagates across digits and out the top.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_same_len(&mut a, &[digit(1), Digit::ZERO]));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // Empty slices are allowed.
    assert!(!add_same_len(&mut [], &[]));
}

#[test]
#[should_panic]
fn add_same_len_mismatched() {
    add_same_len(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn add_digit_basic() {
    let mut a = [digit(1), digit(2)];
    assert!(!add_digit(&mut a, digit(7)));
    assert_eq!(a, [digit(8), digit(2)]);

    let mut a = [Digit::MAX, digit(3)];
    assert!(!add_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::ZERO, digit(4)]);

    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);
}

#[test]
#[should_panic]
fn add_digit_empty() {
    add_digit(&mut [], digit(1));
}

#[test]
fn add_carry_basic() {
    let mut a = [Digit::MAX, digit(0)];
    assert!(!add_carry(&mut a, true));
    assert_eq!(a, [Digit::ZERO, digit(1)]);

    // Without an incoming carry nothing changes.
    let mut a = [Digit::MAX];
    assert!(!add_carry(&mut a, false));
    assert_eq!(a, [Digit::MAX]);

    // A carry out of the most-significant digit.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_carry(&mut a, true));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // An empty slice passes the carry through.
    assert!(add_carry(&mut [], true));
    assert!(!add_carry(&mut [], false));
}

/// The number of digits that fit in a `u128`, used to check against `u128` arithmetic.
const ORACLE_LEN: usize = 128 / DIGIT_BITS_USIZE;

/// The value of `digits` as a `u128`. The total bit width must be at most 128.
fn value(digits: &[Digit]) -> u128 {
    let mut v: u128 = 0;
    for &d in digits.iter().rev() {
        v = (v << Digit::BITS) | d.to_u128();
    }
    v
}

proptest! {
    // `add` matches `u128` arithmetic on slices short enough to fit.
    #[test]
    fn add_matches_u128(
        a in vec(any::<Digit>(), 1..=ORACLE_LEN),
        b in vec(any::<Digit>(), 0..=ORACLE_LEN),
    ) {
        let mut b = b;
        b.truncate(a.len());

        let (sum, mut expected_carry) = value(&a).overflowing_add(value(&b));
        let bits = a.len() * DIGIT_BITS_USIZE;
        let mut expected = sum;
        if bits < 128 {
            expected = sum & ((1u128 << bits) - 1);
            expected_carry = (sum >> bits) != 0;
        }

        let mut a = a;
        let carry = add(&mut a, &b);
        prop_assert_eq!(value(&a), expected);
        prop_assert_eq!(carry, expected_carry);
    }

    // `add` with a zero-extended `rhs` agrees with `add_same_len`.
    #[test]
    fn add_matches_add_same_len(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let (mut longer, shorter) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let mut padded = shorter.clone();
        padded.resize(longer.len(), Digit::ZERO);

        let mut same_len = longer.clone();
        let carry = add(&mut longer, &shorter);
        let same_len_carry = add_same_len(&mut same_len, &padded);
        prop_assert_eq!(longer, same_len);
        prop_assert_eq!(carry, same_len_carry);
    }

    // Addition of zero-padded equal-length slices is commutative.
    #[test]
    fn add_same_len_commutes(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let n = a.len().max(b.len());
        let mut x = a;
        x.resize(n, Digit::ZERO);
        let mut y = b;
        y.resize(n, Digit::ZERO);

        let mut x_plus_y = x.clone();
        let carry_xy = add_same_len(&mut x_plus_y, &y);
        let mut y_plus_x = y.clone();
        let carry_yx = add_same_len(&mut y_plus_x, &x);
        prop_assert_eq!(x_plus_y, y_plus_x);
        prop_assert_eq!(carry_xy, carry_yx);
    }

    // `add_digit` agrees with `add` of a one-digit slice.
    #[test]
    fn add_digit_matches_add(a in vec(any::<Digit>(), 1..20), d: Digit) {
        let mut via_digit = a.clone();
        let mut via_add = a;
        let carry_digit = add_digit(&mut via_digit, d);
        let carry_add = add(&mut via_add, &[d]);
        prop_assert_eq!(via_digit, via_add);
        prop_assert_eq!(carry_digit, carry_add);
    }

    // `add_carry` agrees with `add_digit` of 0 or 1.
    #[test]
    fn add_carry_matches_add_digit(a in vec(any::<Digit>(), 1..20), carry: bool) {
        let mut via_carry = a.clone();
        let mut via_digit = a;
        let carry_out = add_carry(&mut via_carry, carry);
        let digit_carry_out = add_digit(&mut via_digit, Digit::from(u8::from(carry)));
        prop_assert_eq!(via_carry, via_digit);
        prop_assert_eq!(carry_out, digit_carry_out);
    }
}
