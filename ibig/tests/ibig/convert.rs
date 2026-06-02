//! Integration tests for `IBig` <-> two's complement byte-sequence conversions.

use ibig::{IBig, UBig};

/// Little-endian two's complement byte sequences that are already canonical (no redundant
/// most-significant sign-extension byte), covering zero, both signs, and multi-digit
/// values at every digit width.
fn canonical_le() -> Vec<Vec<u8>> {
    vec![
        vec![0],                                              // 0
        vec![5],                                              // +5
        vec![0x7f],                                           // +127
        vec![0xff],                                           // -1
        vec![0x80],                                           // -128
        vec![0xc8, 0x00], // +200 (needs the leading zero to stay positive)
        vec![0x00, 0x80], // -32768
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], // a multi-digit positive value
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe], // a multi-digit negative value
    ]
}

#[test]
fn le_round_trip() {
    for bytes in canonical_le() {
        assert_eq!(IBig::from_le_bytes(&bytes).to_le_bytes(), bytes);
    }
}

#[test]
fn be_round_trip() {
    for le in canonical_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        assert_eq!(IBig::from_be_bytes(&be).to_be_bytes(), be);
    }
}

#[test]
fn le_be_agree() {
    for le in canonical_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        assert_eq!(IBig::from_le_bytes(&le), IBig::from_be_bytes(&be));

        // `to_be_bytes` is `to_le_bytes` reversed.
        let n = IBig::from_le_bytes(&le);
        let mut le_reversed = n.to_le_bytes();
        le_reversed.reverse();
        assert_eq!(n.to_be_bytes(), le_reversed);
    }
}

#[test]
fn sign_extension_is_ignored() {
    // +200 with extra zero sign padding.
    assert_eq!(IBig::from_le_bytes(&[200, 0, 0]), IBig::from(200i16));
    // -100 with extra 0xff sign padding.
    assert_eq!(IBig::from_le_bytes(&[0x9c, 0xff, 0xff]), IBig::from(-100i8));
    // The big-endian side ignores leading sign bytes too.
    assert_eq!(IBig::from_be_bytes(&[0xff, 0xff, 0x9c]), IBig::from(-100i8));
}

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
}

#[test]
#[should_panic]
fn from_le_empty_panics() {
    IBig::from_le_bytes(&[]);
}

#[test]
#[should_panic]
fn from_be_empty_panics() {
    IBig::from_be_bytes(&[]);
}
