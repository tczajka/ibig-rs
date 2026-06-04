//! Integration tests for `UBig` and `IBig` <-> byte-sequence conversions.

use ibig::{IBig, UBig};

/// Little-endian byte sequences that are already normalized (no most-significant zero
/// byte), chosen to exercise sub-digit, single-digit and multi-digit values at every digit
/// width.
fn ubig_canonical_le() -> Vec<Vec<u8>> {
    vec![
        vec![1],
        vec![0xff],
        vec![1, 2],
        vec![0, 1], // a zero low byte is kept; only top zeros are not
        vec![0xff, 0xff, 0xff],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 2], // spans more than one 64-bit digit
        (1..=121).collect(),
    ]
}

/// Little-endian two's complement byte sequences that are already canonical (no redundant
/// most-significant sign-extension byte), covering zero, both signs, and multi-digit
/// values at every digit width.
fn ibig_canonical_le() -> Vec<Vec<u8>> {
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
        (1..=121).collect(),
    ]
}

#[test]
fn ubig_zero() {
    assert_eq!(UBig::from_le_bytes(&[]), UBig::ZERO);
    assert_eq!(UBig::from_le_bytes(&[0, 0, 0]), UBig::ZERO);
    assert_eq!(UBig::from_be_bytes(&[]), UBig::ZERO);
    assert_eq!(UBig::from_be_bytes(&[0, 0, 0]), UBig::ZERO);
    assert_eq!(UBig::ZERO.to_le_bytes(), []);
    assert_eq!(UBig::ZERO.to_be_bytes(), []);
}

#[test]
fn ubig_le_round_trip() {
    for bytes in ubig_canonical_le() {
        assert_eq!(UBig::from_le_bytes(&bytes).to_le_bytes(), bytes);
    }
}

#[test]
fn ubig_le_normalizes_trailing_zeros() {
    assert_eq!(UBig::from_le_bytes(&[5, 0, 0]).to_le_bytes(), [5]);
    assert_eq!(UBig::from_le_bytes(&[1, 2, 0, 0, 0]).to_le_bytes(), [1, 2]);
}

#[test]
fn ibig_le_round_trip() {
    for bytes in ibig_canonical_le() {
        assert_eq!(IBig::from_le_bytes(&bytes).to_le_bytes(), bytes);
    }
}

#[test]
fn ubig_be_round_trip() {
    for le in ubig_canonical_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        assert_eq!(UBig::from_be_bytes(&be).to_be_bytes(), be);
    }
}

#[test]
fn ubig_be_normalizes_leading_zeros() {
    assert_eq!(UBig::from_be_bytes(&[0, 0, 5]).to_be_bytes(), [5]);
    assert_eq!(UBig::from_be_bytes(&[0, 0, 0, 1, 2]).to_be_bytes(), [1, 2]);
}

#[test]
fn ibig_be_round_trip() {
    for le in ibig_canonical_le() {
        let be: Vec<u8> = le.iter().rev().copied().collect();
        assert_eq!(IBig::from_be_bytes(&be).to_be_bytes(), be);
    }
}

#[test]
fn ubig_le_be_agree() {
    for le in ubig_canonical_le() {
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
fn ibig_le_be_agree() {
    for le in ibig_canonical_le() {
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
fn ibig_sign_extension_is_ignored() {
    // +200 with extra zero sign padding.
    assert_eq!(IBig::from_le_bytes(&[200, 0, 0]), IBig::from(200i16));
    // -100 with extra 0xff sign padding.
    assert_eq!(IBig::from_le_bytes(&[0x9c, 0xff, 0xff]), IBig::from(-100i8));
    // The big-endian side ignores leading sign bytes too.
    assert_eq!(IBig::from_be_bytes(&[0xff, 0xff, 0x9c]), IBig::from(-100i8));
}

#[test]
#[should_panic]
fn ibig_from_le_empty_panics() {
    IBig::from_le_bytes(&[]);
}

#[test]
#[should_panic]
fn ibig_from_be_empty_panics() {
    IBig::from_be_bytes(&[]);
}
