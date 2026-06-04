//! Integration tests for `IBig` conversions to and from primitives and `UBig`.

use ibig::{IBig, UBig};

#[test]
fn from_signed_const() {
    // The `from_iN` constructors are usable in `const` contexts.
    const NEG: IBig = IBig::from_i64(-0x0102030405060708);
    const POS: IBig = IBig::from_i32(0x0a0b0c0d);
    const ZERO: IBig = IBig::from_i8(0);

    assert_eq!(
        NEG,
        IBig::from_le_bytes(&(-0x0102030405060708i64).to_le_bytes())
    );
    assert_eq!(POS, IBig::from_le_bytes(&0x0a0b0c0di32.to_le_bytes()));
    assert_eq!(ZERO.to_le_bytes(), [0]);
    // -1 is all-ones in two's complement.
    assert_eq!(IBig::from_i8(-1).to_le_bytes(), [0xff]);
}

#[test]
fn from_signed() {
    // The `From` impls agree with the `from_iN` constructors and the byte conversions.
    assert_eq!(IBig::from(0i8), IBig::ZERO);
    assert_eq!(IBig::from(5i16), IBig::from_i16(5));
    assert_eq!(IBig::from(-1i32).to_le_bytes(), [0xff]);
    assert_eq!(
        IBig::from(i64::MIN),
        IBig::from_le_bytes(&i64::MIN.to_le_bytes())
    );
    assert_eq!(
        IBig::from(i128::MAX),
        IBig::from_le_bytes(&i128::MAX.to_le_bytes())
    );
    assert_eq!(IBig::from(-1234isize), IBig::from_i16(-1234));
    // The same value through different types is equal.
    assert_eq!(IBig::from(-5i8), IBig::from(-5i128));
}

#[test]
fn from_unsigned() {
    assert_eq!(IBig::from(0u8), IBig::ZERO);
    // Non-negative values match the signed conversion of the same value.
    assert_eq!(IBig::from(127u8), IBig::from(127i8));
    assert_eq!(IBig::from(200u32), IBig::from(200i32));
    assert_eq!(IBig::from(255usize), IBig::from(255i32));
    // `u64::MAX` is a positive `2^64 - 1`, gaining a leading zero byte to stay non-negative.
    assert_eq!(
        IBig::from(u64::MAX),
        IBig::from_le_bytes(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0])
    );
}

#[test]
fn from_ubig() {
    assert_eq!(IBig::from(UBig::ZERO), IBig::ZERO);
    // A value whose top bit is clear keeps the same magnitude.
    assert_eq!(IBig::from(UBig::from(200u8)), IBig::from(200i16));
    // A value whose most-significant digit has its sign bit set gains a zero high digit so
    // it stays non-negative; the unsigned `u64::MAX` becomes a positive `2^64 - 1`.
    assert_eq!(
        IBig::from(UBig::from(u64::MAX)),
        IBig::from_le_bytes(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0])
    );
    // The by-reference conversion agrees with the by-value one.
    let big = UBig::from(u64::MAX);
    assert_eq!(IBig::from(&big), IBig::from(big.clone()));
    assert_eq!(IBig::from(&UBig::from(200u8)), IBig::from(200i16));
}

#[test]
fn try_into_signed_primitive() {
    // Single-digit fast path, both signs.
    assert_eq!(i32::try_from(IBig::from(100i8)).unwrap(), 100);
    assert_eq!(i8::try_from(IBig::from(-100i8)).unwrap(), -100);
    assert!(i8::try_from(IBig::from(200i16)).is_err());
    assert!(i8::try_from(IBig::from(-200i16)).is_err());
    assert_eq!(i32::try_from(IBig::ZERO).unwrap(), 0);

    // Multi-digit values.
    assert_eq!(
        i128::try_from(IBig::from(u64::MAX)).unwrap(),
        i128::from(u64::MAX)
    );
    assert!(i64::try_from(IBig::from(u64::MAX)).is_err());
    // A large negative value (multi-digit at every digit width).
    let neg = IBig::from(-1i128 << 100);
    assert_eq!(i128::try_from(&neg).unwrap(), -(1i128 << 100));
    assert!(i64::try_from(&neg).is_err());
}

#[test]
fn try_into_unsigned_primitive() {
    // Single-digit fast path.
    assert_eq!(u32::try_from(IBig::from(100i8)).unwrap(), 100);
    assert_eq!(u8::try_from(IBig::from(255i16)).unwrap(), 255);
    assert_eq!(u32::try_from(IBig::ZERO).unwrap(), 0);
    // Negative values are out of range for any unsigned type.
    assert!(u8::try_from(IBig::from(-1i8)).is_err());
    assert!(u128::try_from(IBig::from(-1i8)).is_err());
    // Too large for the target type.
    assert!(u8::try_from(IBig::from(256i16)).is_err());

    // Multi-digit values.
    assert_eq!(
        u128::try_from(IBig::from(u64::MAX)).unwrap(),
        u128::from(u64::MAX)
    );
    let big = IBig::from(1i128 << 100);
    assert_eq!(u128::try_from(&big).unwrap(), 1u128 << 100);
    assert!(u64::try_from(&big).is_err());
}

#[test]
fn bool_conversions() {
    assert_eq!(IBig::from(false), IBig::ZERO);
    assert_eq!(IBig::from(true), IBig::from(1i8));
    assert!(!bool::try_from(IBig::ZERO).unwrap());
    assert!(bool::try_from(IBig::from(1i8)).unwrap());
    assert!(bool::try_from(IBig::from(2i8)).is_err());
    assert!(bool::try_from(IBig::from(-1i8)).is_err());
    // By reference.
    assert!(bool::try_from(&IBig::from(1i8)).unwrap());
}
