//! Integration tests for `UBig` <-> byte-sequence conversions.

use ibig::UBig;

/// Little-endian byte sequences that are already normalized (no most-significant zero
/// byte), chosen to exercise sub-digit, single-digit and multi-digit values at every digit
/// width.
fn normalized_le() -> Vec<Vec<u8>> {
    vec![
        vec![1],
        vec![0xff],
        vec![1, 2],
        vec![0, 1], // a zero low byte is kept; only top zeros are not
        vec![0xff, 0xff, 0xff],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 2], // spans more than one 64-bit digit
        (1..=30).collect(),
    ]
}

#[test]
fn zero() {
    assert!(UBig::from_le_bytes(&[]).to_le_bytes().is_empty());
    assert!(UBig::from_be_bytes(&[]).to_be_bytes().is_empty());
    assert!(UBig::from_le_bytes(&[0, 0, 0]).to_le_bytes().is_empty());
    assert!(UBig::from_be_bytes(&[0, 0, 0]).to_be_bytes().is_empty());
    assert_eq!(UBig::from_le_bytes(&[0, 0]), UBig::from_be_bytes(&[]));
}

#[test]
fn le_round_trip() {
    for bytes in normalized_le() {
        assert_eq!(UBig::from_le_bytes(&bytes).to_le_bytes(), bytes);
    }
}

#[test]
fn be_round_trip() {
    for le in normalized_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        assert_eq!(UBig::from_be_bytes(&be).to_be_bytes(), be);
    }
}

#[test]
fn le_normalizes_trailing_zeros() {
    assert_eq!(UBig::from_le_bytes(&[5, 0, 0]).to_le_bytes(), [5]);
    assert_eq!(UBig::from_le_bytes(&[1, 2, 0, 0, 0]).to_le_bytes(), [1, 2]);
}

#[test]
fn be_normalizes_leading_zeros() {
    assert_eq!(UBig::from_be_bytes(&[0, 0, 5]).to_be_bytes(), [5]);
    assert_eq!(UBig::from_be_bytes(&[0, 0, 0, 1, 2]).to_be_bytes(), [1, 2]);
}

#[test]
fn le_be_agree() {
    for le in normalized_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        // Same magnitude whether read little-endian or as the reversed big-endian bytes.
        assert_eq!(UBig::from_le_bytes(&le), UBig::from_be_bytes(&be));

        // `to_be_bytes` is `to_le_bytes` reversed.
        let n = UBig::from_le_bytes(&le);
        let mut le_reversed = n.to_le_bytes();
        le_reversed.reverse();
        assert_eq!(n.to_be_bytes(), le_reversed);
    }
}

#[test]
fn from_unsigned() {
    // Zero from every type is the empty byte sequence.
    assert_eq!(UBig::from(0u8), UBig::from_le_bytes(&[]));
    assert_eq!(UBig::from(0u128), UBig::from_le_bytes(&[]));
    // Each type matches its little-endian bytes.
    assert_eq!(UBig::from(5u8), UBig::from_le_bytes(&[5]));
    assert_eq!(UBig::from(0x0102u16), UBig::from_le_bytes(&[0x02, 0x01]));
    assert_eq!(UBig::from(u32::MAX), UBig::from_le_bytes(&[0xff; 4]));
    assert_eq!(UBig::from(u64::MAX), UBig::from_le_bytes(&[0xff; 8]));
    assert_eq!(UBig::from(u128::MAX), UBig::from_le_bytes(&[0xff; 16]));
    // The same value through different types is equal.
    assert_eq!(UBig::from(255u8), UBig::from(255u128));
    assert_eq!(UBig::from(1234usize), UBig::from(1234u32));
}

#[test]
fn from_unsigned_const() {
    // The `from_uN` constructors are usable in `const` contexts.
    const A: UBig = UBig::from_u64(0x0102030405060708);
    const ZERO: UBig = UBig::from_u8(0);

    assert_eq!(
        A.to_le_bytes(),
        [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
    );
    assert!(ZERO.to_le_bytes().is_empty());
    // They agree with the `From` impls.
    assert_eq!(UBig::from_u32(0xdead_beef), UBig::from(0xdead_beefu32));
    assert_eq!(UBig::from_u16(0x0102), UBig::from_le_bytes(&[0x02, 0x01]));
}

#[test]
fn try_from_signed() {
    // Non-negative values convert and match the unsigned conversion.
    assert_eq!(UBig::try_from(0i8).unwrap(), UBig::from(0u8));
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
    assert!(UBig::try_from(-1i32).is_err());
    assert!(UBig::try_from(i64::MIN).is_err());
    assert!(UBig::try_from(-1i128).is_err());
    assert!(UBig::try_from(-1isize).is_err());
}
