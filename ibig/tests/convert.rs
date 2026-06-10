//! Integration tests for `UBig` and `IBig` conversions to and from primitives.

use ibig::proptest::ubig_up_to_bits;
use ibig::{IBig, UBig};
use proptest::prelude::*;

// Round-trips a primitive value `v: $t` through `$big`: `From` (and the `const` `$from_const`
// constructor, where one exists) must agree, and `TryFrom<&$big>` must recover `v`.
macro_rules! from_roundtrip {
    ($name:ident, $t:ty, $big:ty) => {
        proptest! {
            #[test]
            fn $name(v: $t) {
                let big = <$big>::from(v);
                prop_assert_eq!(<$t>::try_from(&big).unwrap(), v);
            }
        }
    };
    ($name:ident, $t:ty, $big:ty, $from_const:ident) => {
        proptest! {
            #[test]
            fn $name(v: $t) {
                let big = <$big>::from(v);
                prop_assert_eq!(<$t>::try_from(&big).unwrap(), v);
                // The `const` constructor agrees with `From`.
                prop_assert_eq!(<$big>::$from_const(v), big);
            }
        }
    };
}

// `UBig` from unsigned primitives (`const_from_uN` const constructors for the fixed-width types).
from_roundtrip!(ubig_from_u8, u8, UBig, const_from_u8);
from_roundtrip!(ubig_from_u16, u16, UBig, const_from_u16);
from_roundtrip!(ubig_from_u32, u32, UBig, const_from_u32);
from_roundtrip!(ubig_from_u64, u64, UBig, const_from_u64);
from_roundtrip!(ubig_from_u128, u128, UBig);
from_roundtrip!(ubig_from_usize, usize, UBig);
from_roundtrip!(ubig_from_bool, bool, UBig);

// `IBig` from unsigned primitives.
from_roundtrip!(ibig_from_u8, u8, IBig);
from_roundtrip!(ibig_from_u16, u16, IBig);
from_roundtrip!(ibig_from_u32, u32, IBig);
from_roundtrip!(ibig_from_u64, u64, IBig);
from_roundtrip!(ibig_from_u128, u128, IBig);
from_roundtrip!(ibig_from_usize, usize, IBig);
from_roundtrip!(ibig_from_bool, bool, IBig);

// `IBig` from signed primitives (`const_from_iN` const constructors for the fixed-width types).
from_roundtrip!(ibig_from_i8, i8, IBig, const_from_i8);
from_roundtrip!(ibig_from_i16, i16, IBig, const_from_i16);
from_roundtrip!(ibig_from_i32, i32, IBig, const_from_i32);
from_roundtrip!(ibig_from_i64, i64, IBig, const_from_i64);
from_roundtrip!(ibig_from_i128, i128, IBig);
from_roundtrip!(ibig_from_isize, isize, IBig);

#[test]
fn ubig_try_from_signed() {
    // Non-negative values convert and match the unsigned conversion.
    assert_eq!(UBig::try_from(0i8).unwrap(), UBig::ZERO);
    assert_eq!(UBig::try_from(12i16).unwrap(), UBig::from(12u16));
    assert_eq!(UBig::try_from(5i32).unwrap(), UBig::from(5u32));
    assert_eq!(
        UBig::try_from(i64::MAX).unwrap(),
        UBig::from(i64::MAX as u64)
    );
    assert_eq!(
        UBig::try_from(i128::MAX).unwrap(),
        UBig::from(i128::MAX as u128)
    );
    assert_eq!(UBig::try_from(1234isize).unwrap(), UBig::from(1234usize));

    // Negative values are rejected.
    assert!(UBig::try_from(-1i8).is_err());
    assert!(UBig::try_from(-1i16).is_err());
    assert!(UBig::try_from(-1i32).is_err());
    assert!(UBig::try_from(i64::MIN).is_err());
    assert!(UBig::try_from(-1i128).is_err());
    assert!(UBig::try_from(-1isize).is_err());
}

#[test]
fn ubig_try_into_primitive_fails() {
    let big = UBig::from_le_bytes(&[0xff; 20]);
    assert!(u8::try_from(&big).is_err());
    assert!(u16::try_from(&big).is_err());
    assert!(u32::try_from(&big).is_err());
    assert!(u64::try_from(&big).is_err());
    assert!(u128::try_from(&big).is_err());
    assert!(bool::try_from(&big).is_err());
    assert!(usize::try_from(&big).is_err());
    assert!(i8::try_from(&big).is_err());
    assert!(i16::try_from(&big).is_err());
    assert!(i32::try_from(&big).is_err());
    assert!(i64::try_from(&big).is_err());
    assert!(i128::try_from(&big).is_err());
    assert!(isize::try_from(&big).is_err());
}

#[test]
fn ibig_try_into_primitive_fails() {
    let big = IBig::from_le_bytes(&[0x3f; 20]);
    assert!(u8::try_from(&big).is_err());
    assert!(u16::try_from(&big).is_err());
    assert!(u32::try_from(&big).is_err());
    assert!(u64::try_from(&big).is_err());
    assert!(u128::try_from(&big).is_err());
    assert!(bool::try_from(&big).is_err());
    assert!(usize::try_from(&big).is_err());
    assert!(i8::try_from(&big).is_err());
    assert!(i16::try_from(&big).is_err());
    assert!(i32::try_from(&big).is_err());
    assert!(i64::try_from(&big).is_err());
    assert!(i128::try_from(&big).is_err());
    assert!(isize::try_from(&big).is_err());
}

#[test]
fn ibig_try_into_unsigned_fails() {
    let big_neg = IBig::from(-1i8);
    assert!(u8::try_from(&big_neg).is_err());
    assert!(u16::try_from(&big_neg).is_err());
    assert!(u32::try_from(&big_neg).is_err());
    assert!(u64::try_from(&big_neg).is_err());
    assert!(u128::try_from(&big_neg).is_err());
    assert!(bool::try_from(&big_neg).is_err());
    assert!(usize::try_from(&big_neg).is_err());
    assert!(UBig::try_from(&big_neg).is_err());
    assert!(UBig::try_from(big_neg).is_err());
}

#[test]
fn ubig_from_char() {
    assert_eq!(UBig::from('A'), UBig::from(65u8));
    assert_eq!(UBig::from('\0'), UBig::ZERO);
    assert_eq!(UBig::from('\u{10ffff}'), UBig::from(0x10ffffu32));
}

proptest! {
    // `UBig` -> `IBig` -> `UBig` round-trips: `From` produces a non-negative `IBig`, and
    // `TryFrom` recovers the original.
    #[test]
    fn ubig_ibig_round_trip(x in ubig_up_to_bits(1000)) {
        let signed = IBig::from(&x);
        prop_assert_eq!(UBig::try_from(&signed).unwrap(), x);
    }
}

#[test]
fn try_from_big_error_display() {
    let err = u8::try_from(UBig::from(256u16)).unwrap_err();
    assert_eq!(err.to_string(), "number out of range for the target type");
}
