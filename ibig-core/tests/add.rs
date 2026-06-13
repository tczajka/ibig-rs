//! Integration tests for addition.

use ibig_core::{
    Digit, SignedDigit, add_signed_sdigit, add_signed_signed, add_unsigned_1, add_unsigned_carry,
    add_unsigned_digit, add_unsigned_unsigned, add_unsigned_unsigned_same_len, extend_signed,
};
use proptest::collection::vec;
use proptest::prelude::*;

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

fn sdigit(n: i8) -> SignedDigit {
    SignedDigit::from(n)
}

#[test]
fn add_unsigned_unsigned_basic() {
    let mut a = [digit(1), digit(2), digit(3)];
    assert!(!add_unsigned_unsigned(&mut a, &[digit(3), digit(7)]));
    assert_eq!(a, [digit(4), digit(9), digit(3)]);

    // A carry out of the low digits propagates through the high digits.
    let mut a = [Digit::MAX, Digit::MAX, digit(3)];
    assert!(!add_unsigned_unsigned(&mut a, &[digit(1)]));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO, digit(4)]);

    // A carry out of the most-significant digit.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_unsigned_unsigned(&mut a, &[digit(1), Digit::ZERO]));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // An empty `rhs` is allowed.
    let mut a = [digit(5)];
    assert!(!add_unsigned_unsigned(&mut a, &[]));
    assert_eq!(a, [digit(5)]);
}

#[test]
#[should_panic]
fn add_unsigned_unsigned_rhs_longer() {
    add_unsigned_unsigned(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn add_unsigned_unsigned_same_len_basic() {
    let mut a = [digit(1), digit(2)];
    assert!(!add_unsigned_unsigned_same_len(
        &mut a,
        &[digit(3), digit(4)]
    ));
    assert_eq!(a, [digit(4), digit(6)]);

    // The carry propagates across digits and out the top.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_unsigned_unsigned_same_len(
        &mut a,
        &[digit(1), Digit::ZERO]
    ));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // Empty slices are allowed.
    assert!(!add_unsigned_unsigned_same_len(&mut [], &[]));
}

#[test]
#[should_panic]
fn add_unsigned_unsigned_same_len_mismatched() {
    add_unsigned_unsigned_same_len(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
fn add_unsigned_digit_basic() {
    let mut a = [digit(1), digit(2)];
    assert!(!add_unsigned_digit(&mut a, digit(7)));
    assert_eq!(a, [digit(8), digit(2)]);

    let mut a = [Digit::MAX, digit(3)];
    assert!(!add_unsigned_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::ZERO, digit(4)]);

    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_unsigned_digit(&mut a, digit(1)));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);
}

#[test]
#[should_panic]
fn add_unsigned_digit_empty() {
    add_unsigned_digit(&mut [], digit(1));
}

#[test]
fn add_unsigned_carry_basic() {
    let mut a = [Digit::MAX, digit(0)];
    assert!(!add_unsigned_carry(&mut a, true));
    assert_eq!(a, [Digit::ZERO, digit(1)]);

    // Without an incoming carry nothing changes.
    let mut a = [Digit::MAX];
    assert!(!add_unsigned_carry(&mut a, false));
    assert_eq!(a, [Digit::MAX]);

    // A carry out of the most-significant digit.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_unsigned_carry(&mut a, true));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // An empty slice passes the carry through.
    assert!(add_unsigned_carry(&mut [], true));
    assert!(!add_unsigned_carry(&mut [], false));
}

#[test]
fn add_unsigned_1_basic() {
    let mut a = [digit(1), digit(2)];
    assert!(!add_unsigned_1(&mut a));
    assert_eq!(a, [digit(2), digit(2)]);

    // The increment ripples through all-ones digits.
    let mut a = [Digit::MAX, Digit::MAX, digit(3)];
    assert!(!add_unsigned_1(&mut a));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO, digit(4)]);

    // All ones overflows.
    let mut a = [Digit::MAX, Digit::MAX];
    assert!(add_unsigned_1(&mut a));
    assert_eq!(a, [Digit::ZERO, Digit::ZERO]);

    // An empty slice overflows immediately.
    assert!(add_unsigned_1(&mut []));
}

#[test]
fn add_signed_signed_basic() {
    // Two non-negative values.
    let mut a = [digit(2), digit(3)];
    assert_eq!(add_signed_signed(&mut a, &[digit(5)]), sdigit(0));
    assert_eq!(a, [digit(7), digit(3)]);

    // -1 + -1 == -2.
    let mut a = [Digit::MAX];
    assert_eq!(add_signed_signed(&mut a, &[Digit::MAX]), sdigit(-1));
    assert_eq!(a, [Digit::MAX - digit(1)]);

    // A positive sum that overflows into the returned digit.
    let signed_max = Digit::MAX >> 1;
    let mut a = [signed_max];
    assert_eq!(add_signed_signed(&mut a, &[digit(1)]), sdigit(0));
    assert_eq!(a, [signed_max + digit(1)]);

    // A negative sum that overflows into the returned digit: -2^(bits-1) + -2^(bits-1).
    let signed_min = signed_max + digit(1);
    let mut a = [signed_min];
    assert_eq!(add_signed_signed(&mut a, &[signed_min]), sdigit(-1));
    assert_eq!(a, [Digit::ZERO]);

    // A negative `rhs` sign-extends across the high digits of `lhs` and borrows through
    // all-zeros digits: 2^(2*bits) + -1.
    let mut a = [Digit::ZERO, Digit::ZERO, digit(1)];
    assert_eq!(add_signed_signed(&mut a, &[Digit::MAX]), sdigit(0));
    assert_eq!(a, [Digit::MAX, Digit::MAX, Digit::ZERO]);

    // Mixed signs without overflow: 5 + -3 == 2.
    let mut a = [digit(5)];
    assert_eq!(
        add_signed_signed(&mut a, &[Digit::MAX - digit(2)]),
        sdigit(0)
    );
    assert_eq!(a, [digit(2)]);
}

#[test]
fn add_signed_sdigit_basic() {
    // Two non-negative values.
    let mut a = [digit(2), digit(3)];
    assert_eq!(add_signed_sdigit(&mut a, sdigit(5)), sdigit(0));
    assert_eq!(a, [digit(7), digit(3)]);

    // -1 + -1 == -2.
    let mut a = [Digit::MAX];
    assert_eq!(add_signed_sdigit(&mut a, sdigit(-1)), sdigit(-1));
    assert_eq!(a, [Digit::MAX - digit(1)]);

    // The carry propagates into the high digits.
    let mut a = [Digit::MAX, Digit::ZERO];
    assert_eq!(add_signed_sdigit(&mut a, sdigit(1)), sdigit(0));
    assert_eq!(a, [Digit::ZERO, digit(1)]);

    // A negative digit borrows through all-zeros digits: 2^(2*bits) + -1.
    let mut a = [Digit::ZERO, Digit::ZERO, digit(1)];
    assert_eq!(add_signed_sdigit(&mut a, sdigit(-1)), sdigit(0));
    assert_eq!(a, [Digit::MAX, Digit::MAX, Digit::ZERO]);
}

#[test]
#[should_panic]
fn add_signed_sdigit_empty() {
    add_signed_sdigit(&mut [], sdigit(1));
}

#[test]
#[should_panic]
fn add_signed_signed_rhs_longer() {
    add_signed_signed(&mut [digit(1)], &[digit(1), digit(2)]);
}

#[test]
#[should_panic]
fn add_signed_signed_rhs_empty() {
    add_signed_signed(&mut [digit(1)], &[]);
}

proptest! {
    // `add_unsigned_unsigned` with a zero-extended `rhs` agrees with
    // `add_unsigned_unsigned_same_len`.
    #[test]
    fn add_unsigned_unsigned_matches_same_len(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let (mut longer, shorter) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let mut padded = shorter.clone();
        padded.resize(longer.len(), Digit::ZERO);

        let mut same_len = longer.clone();
        let carry = add_unsigned_unsigned(&mut longer, &shorter);
        let same_len_carry = add_unsigned_unsigned_same_len(&mut same_len, &padded);
        prop_assert_eq!(longer, same_len);
        prop_assert_eq!(carry, same_len_carry);
    }

    // Addition of zero-padded equal-length slices is commutative.
    #[test]
    fn add_unsigned_unsigned_same_len_commutes(
        a in vec(any::<Digit>(), 0..20),
        b in vec(any::<Digit>(), 0..20),
    ) {
        let n = a.len().max(b.len());
        let mut x = a;
        x.resize(n, Digit::ZERO);
        let mut y = b;
        y.resize(n, Digit::ZERO);

        let mut x_plus_y = x.clone();
        let carry_xy = add_unsigned_unsigned_same_len(&mut x_plus_y, &y);
        let mut y_plus_x = y.clone();
        let carry_yx = add_unsigned_unsigned_same_len(&mut y_plus_x, &x);
        prop_assert_eq!(x_plus_y, y_plus_x);
        prop_assert_eq!(carry_xy, carry_yx);
    }

    // `add_unsigned_digit` agrees with `add_unsigned_unsigned` of a one-digit slice.
    #[test]
    fn add_unsigned_digit_matches_unsigned_unsigned(a in vec(any::<Digit>(), 1..20), d: Digit) {
        let mut via_digit = a.clone();
        let mut via_add = a;
        let carry_digit = add_unsigned_digit(&mut via_digit, d);
        let carry_add = add_unsigned_unsigned(&mut via_add, &[d]);
        prop_assert_eq!(via_digit, via_add);
        prop_assert_eq!(carry_digit, carry_add);
    }

    // `add_unsigned_carry` agrees with `add_unsigned_digit` of 0 or 1.
    #[test]
    fn add_unsigned_carry_matches_unsigned_digit(a in vec(any::<Digit>(), 1..20), carry: bool) {
        let mut via_carry = a.clone();
        let mut via_digit = a;
        let carry_out = add_unsigned_carry(&mut via_carry, carry);
        let digit_carry_out = add_unsigned_digit(&mut via_digit, Digit::from(u8::from(carry)));
        prop_assert_eq!(via_carry, via_digit);
        prop_assert_eq!(carry_out, digit_carry_out);
    }

    // `add_signed_signed` agrees with unsigned addition of the operands sign-extended one digit
    // past `lhs` (a signed sum always fits there, so the unsigned carry can be discarded).
    #[test]
    fn add_signed_signed_matches_extended(
        a in vec(any::<Digit>(), 1..20),
        b in vec(any::<Digit>(), 1..20),
    ) {
        let (mut longer, shorter) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let n = longer.len() + 1;
        let mut x = longer.clone();
        x.resize(n, Digit::ZERO);
        extend_signed(&mut x, longer.len());
        let mut y = shorter.clone();
        y.resize(n, Digit::ZERO);
        extend_signed(&mut y, shorter.len());
        let _ = add_unsigned_unsigned_same_len(&mut x, &y);

        let top = add_signed_signed(&mut longer, &shorter);
        longer.push(top.cast_unsigned());
        prop_assert_eq!(longer, x);
    }

    // `add_signed_sdigit` agrees with `add_signed_signed` of a one-digit slice.
    #[test]
    fn add_signed_sdigit_matches_signed_signed(a in vec(any::<Digit>(), 1..20), d: SignedDigit) {
        let mut via_digit = a.clone();
        let mut via_slice = a;
        let high_digit = add_signed_sdigit(&mut via_digit, d);
        let high_slice = add_signed_signed(&mut via_slice, &[d.cast_unsigned()]);
        prop_assert_eq!(via_digit, via_slice);
        prop_assert_eq!(high_digit, high_slice);
    }
}
