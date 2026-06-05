//! Integration tests for `UBig` and `IBig` bitwise operators.

use ibig::proptest::ibig_up_to_bits;
use ibig::{IBig, UBig};
use proptest::prelude::*;

// Checks `big(a) <op> big(b) == big((a as wide) <op> (b as wide))` over random primitives `a:
// $a` and `b: $b`, where `$wide` is a primitive wide enough to hold both. Letting the operands
// have different widths exercises the single-/multi-digit and unequal-length code paths.
macro_rules! binop_vs_primitive {
    ($name:ident, $big:ty, $op:tt, $op_assign:tt,$a:ty, $b:ty, $wide:ty) => {
        proptest! {
            #[test]
            // `as $wide` is the identity for the same-width operands.
            #[allow(clippy::unnecessary_cast)]
            fn $name(a: $a, b: $b) {
                let big = <$big>::from(a) $op <$big>::from(b);
                let mut big2 = <$big>::from(a);
                big2 $op_assign <$big>::from(b);
                let expected = <$big>::from(<$wide>::from(a) $op <$wide>::from(b));
                prop_assert_eq!(&big, &expected);
                prop_assert_eq!(&big2, &expected);
            }
        }
    };
}

// AND vs primitive, across operand widths.
binop_vs_primitive!(ubig_bitand_16_16, UBig, &, &=, u16, u16, u128);
binop_vs_primitive!(ubig_bitand_16_128, UBig, &, &=, u16, u128, u128);
binop_vs_primitive!(ubig_bitand_128_16, UBig, &, &=, u128, u16, u128);
binop_vs_primitive!(ubig_bitand_128_128, UBig, &, &=, u128, u128, u128);
binop_vs_primitive!(ibig_bitand_16_16, IBig, &, &=, i16, i16, i128);
binop_vs_primitive!(ibig_bitand_16_128, IBig, &, &=, i16, i128, i128);
binop_vs_primitive!(ibig_bitand_128_16, IBig, &, &=, i128, i16, i128);
binop_vs_primitive!(ibig_bitand_128_128, IBig, &, &=, i128, i128, i128);

// OR vs primitive, across operand widths.
binop_vs_primitive!(ubig_bitor_16_16, UBig, |, |=, u16, u16, u128);
binop_vs_primitive!(ubig_bitor_16_128, UBig, |, |=, u16, u128, u128);
binop_vs_primitive!(ubig_bitor_128_16, UBig, |, |=, u128, u16, u128);
binop_vs_primitive!(ubig_bitor_128_128, UBig, |, |=, u128, u128, u128);
binop_vs_primitive!(ibig_bitor_16_16, IBig, |, |=, i16, i16, i128);
binop_vs_primitive!(ibig_bitor_16_128, IBig, |, |=, i16, i128, i128);
binop_vs_primitive!(ibig_bitor_128_16, IBig, |, |=, i128, i16, i128);
binop_vs_primitive!(ibig_bitor_128_128, IBig, |, |=, i128, i128, i128);

proptest! {
    // Bitwise NOT vs the primitive: `!IBig::from(v) == IBig::from(!v)`.
    #[test]
    fn ibig_not_vs_i16(v: i16) {
        prop_assert_eq!(!IBig::from(v), IBig::from(!v));
    }

    #[test]
    fn ibig_not_vs_i128(v: i128) {
        prop_assert_eq!(!IBig::from(v), IBig::from(!v));
    }

    // Bitwise NOT is an involution: `!!x == x`.
    #[test]
    fn ibig_not_not(x in ibig_up_to_bits(1000)) {
        prop_assert_eq!(!!&x, x);
    }

    // De Morgan over arbitrary sizes: `a | b == !(!a & !b)`.
    #[test]
    fn ibig_de_morgan(a in ibig_up_to_bits(1000), b in ibig_up_to_bits(1000)) {
        prop_assert_eq!(&a | &b, !(!&a & !&b));
    }
}
