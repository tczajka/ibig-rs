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
        (1..=121).collect(),
    ]
}

#[test]
fn zero() {
    assert_eq!(UBig::from_le_bytes(&[]), UBig::ZERO);
    assert_eq!(UBig::from_le_bytes(&[0, 0, 0]), UBig::ZERO);
    assert_eq!(UBig::from_be_bytes(&[]), UBig::ZERO);
    assert_eq!(UBig::from_be_bytes(&[0, 0, 0]), UBig::ZERO);
    assert_eq!(UBig::ZERO.to_le_bytes(), []);
    assert_eq!(UBig::ZERO.to_be_bytes(), []);
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
