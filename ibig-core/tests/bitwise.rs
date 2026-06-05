//! Integration tests for bitwise operations between digit slices.

use ibig_core::{Digit, bitand_same_len, bitandnot_same_len, bitor_same_len, bitxor_same_len, not};
use proptest::prelude::*;

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_not() {
    let mut a = [digit(0b1100), Digit::ZERO, Digit::MAX];
    not(&mut a);
    assert_eq!(a, [!digit(0b1100), Digit::MAX, Digit::ZERO]);

    // Empty slice is allowed.
    let mut empty: [Digit; 0] = [];
    not(&mut empty);
    assert_eq!(empty, []);
}

proptest! {
    // Applying `not` twice restores the original digits.
    #[test]
    fn test_not_not(digits in proptest::collection::vec(any::<Digit>(), 0..20)) {
        let mut a = digits.clone();
        not(&mut a);
        not(&mut a);
        prop_assert_eq!(a, digits);
    }
}

#[test]
fn test_bitand_same_len() {
    let mut a = [digit(0b1100), Digit::MAX, digit(0)];
    bitand_same_len(&mut a, &[digit(0b1010), digit(0b1111), Digit::MAX]);
    assert_eq!(a, [digit(0b1000), digit(0b1111), digit(0)]);

    // Empty slices are allowed.
    let mut a: [Digit; 0] = [];
    bitand_same_len(&mut a, &[]);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn test_bitand_same_len_mismatched() {
    bitand_same_len(&mut [Digit::ZERO], &[Digit::ZERO, Digit::ZERO]);
}

#[test]
fn test_bitor_same_len() {
    let mut a = [digit(0b1100), Digit::ZERO, Digit::ZERO];
    bitor_same_len(&mut a, &[digit(0b1010), digit(0b1111), Digit::MAX]);
    assert_eq!(a, [digit(0b1110), digit(0b1111), Digit::MAX]);

    // Empty slices are allowed.
    let mut a: [Digit; 0] = [];
    bitor_same_len(&mut a, &[]);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn test_bitor_same_len_mismatched() {
    bitor_same_len(&mut [Digit::ZERO], &[Digit::ZERO, Digit::ZERO]);
}

#[test]
fn test_bitxor_same_len() {
    let mut a = [digit(0b1100), Digit::MAX, digit(0b1010)];
    bitxor_same_len(&mut a, &[digit(0b1010), Digit::MAX, digit(0b1110)]);
    assert_eq!(a, [digit(0b0110), Digit::ZERO, digit(0b0100)]);

    // Empty slices are allowed.
    let mut a: [Digit; 0] = [];
    bitxor_same_len(&mut a, &[]);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn test_bitxor_same_len_mismatched() {
    bitxor_same_len(&mut [Digit::ZERO], &[Digit::ZERO, Digit::ZERO]);
}

/// Two equal-length random digit slices.
fn same_len_pair() -> impl Strategy<Value = (Vec<Digit>, Vec<Digit>)> {
    (0usize..20).prop_flat_map(|n| {
        (
            proptest::collection::vec(any::<Digit>(), n),
            proptest::collection::vec(any::<Digit>(), n),
        )
    })
}

proptest! {
    // `a ^ b == (a | b) & !(a & b)`.
    #[test]
    fn test_xor_identity((a, b) in same_len_pair()) {
        let mut xor = a.clone();
        bitxor_same_len(&mut xor, &b);

        let mut x = a.clone();
        bitor_same_len(&mut x, &b); // a | b
        let mut nand = a.clone();
        bitand_same_len(&mut nand, &b);
        not(&mut nand); // !(a & b)
        bitand_same_len(&mut x, &nand); // (a | b) & !(a & b)
        prop_assert_eq!(xor, x);
    }

    // De Morgan: `!(a & b) == !a | !b`.
    #[test]
    fn test_de_morgan((a, b) in same_len_pair()) {
        let mut nand = a.clone();
        bitand_same_len(&mut nand, &b);
        not(&mut nand); // !(a & b)

        let mut x = a.clone();
        not(&mut x); // !a
        let mut not_b = b.clone();
        not(&mut not_b); // !b
        bitor_same_len(&mut x, &not_b); // !a | !b
        prop_assert_eq!(nand, x);
    }

    // `andnot(a,b) == a & !b`.
    #[test]
    fn test_bitandnot((a, b) in same_len_pair()) {
        let mut andnot = a.clone();
        bitandnot_same_len(&mut andnot, &b);

        let mut not_b = b.clone();
        not(&mut not_b); // !b
        let mut x = a.clone();
        bitand_same_len(&mut x, &not_b); // a & !b
        prop_assert_eq!(andnot, x);
    }
}
