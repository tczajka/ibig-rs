//! Operand-form consistency: every operator gives the same result whether its operands are
//! passed by value or by reference, including the assigning forms.

use ibig::proptest::{ibig_up_to_bits, ubig_up_to_bits};
use proptest::prelude::*;

/// Checks that all by-value/by-reference forms of a binary operator, and its assigning form,
/// agree with `&a $op &b`.
macro_rules! check_binop {
    ($op:tt, $op_assign:tt, $a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        let expected = &a $op &b;
        prop_assert_eq!(&(a.clone() $op b.clone()), &expected);
        prop_assert_eq!(&(a.clone() $op &b), &expected);
        prop_assert_eq!(&(&a $op b.clone()), &expected);
        let mut t = a.clone();
        t $op_assign b.clone();
        prop_assert_eq!(&t, &expected);
        let mut t = a.clone();
        t $op_assign &b;
        prop_assert_eq!(&t, &expected);
    }};
}

/// Checks that both forms of a unary operator agree with `$op &a`.
macro_rules! check_unop {
    ($op:tt, $a:expr) => {{
        let a = $a;
        let expected = $op &a;
        prop_assert_eq!(&($op a.clone()), &expected);
    }};
}

proptest! {
    #[test]
    fn ubig_add(a in ubig_up_to_bits(300), b in ubig_up_to_bits(300)) {
        check_binop!(+, +=, a, b);
    }

    #[test]
    fn ibig_add(a in ibig_up_to_bits(300), b in ibig_up_to_bits(300)) {
        check_binop!(+, +=, a, b);
    }

    #[test]
    fn ubig_sub(a in ubig_up_to_bits(300), b in ubig_up_to_bits(300)) {
        // Avoid underflow: subtract `b` from `a + b`.
        let c = &a + &b;
        check_binop!(-, -=, c, b);
    }

    #[test]
    fn ibig_sub(a in ibig_up_to_bits(300), b in ibig_up_to_bits(300)) {
        check_binop!(-, -=, a, b);
    }

    #[test]
    fn ubig_bitand(a in ubig_up_to_bits(300), b in ubig_up_to_bits(300)) {
        check_binop!(&, &=, a, b);
    }

    #[test]
    fn ibig_bitand(a in ibig_up_to_bits(300), b in ibig_up_to_bits(300)) {
        check_binop!(&, &=, a, b);
    }

    #[test]
    fn ubig_bitor(a in ubig_up_to_bits(300), b in ubig_up_to_bits(300)) {
        check_binop!(|, |=, a, b);
    }

    #[test]
    fn ibig_bitor(a in ibig_up_to_bits(300), b in ibig_up_to_bits(300)) {
        check_binop!(|, |=, a, b);
    }

    #[test]
    fn ubig_bitxor(a in ubig_up_to_bits(300), b in ubig_up_to_bits(300)) {
        check_binop!(^, ^=, a, b);
    }

    #[test]
    fn ibig_bitxor(a in ibig_up_to_bits(300), b in ibig_up_to_bits(300)) {
        check_binop!(^, ^=, a, b);
    }

    #[test]
    fn ubig_shl(a in ubig_up_to_bits(300), n in 0usize..400) {
        check_binop!(<<, <<=, a, n);
    }

    #[test]
    fn ibig_shl(a in ibig_up_to_bits(300), n in 0usize..400) {
        check_binop!(<<, <<=, a, n);
    }

    #[test]
    fn ubig_shr(a in ubig_up_to_bits(300), n in 0usize..400) {
        check_binop!(>>, >>=, a, n);
    }

    #[test]
    fn ibig_shr(a in ibig_up_to_bits(300), n in 0usize..400) {
        check_binop!(>>, >>=, a, n);
    }

    #[test]
    fn ibig_not(a in ibig_up_to_bits(300)) {
        check_unop!(!, a);
    }

    #[test]
    fn ibig_neg(a in ibig_up_to_bits(300)) {
        check_unop!(-, a);
    }
}
