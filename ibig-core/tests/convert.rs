//! Integration tests for digit-slice normalization helpers.

use ibig_core::{
    Digit, from_be_bytes, from_bytes, min_len, min_len_bytes, min_len_bytes_signed, min_len_signed,
    to_bytes, to_bytes_signed,
};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_min_len() {
    // Zero needs no digits: empty and all-zero slices collapse to 0.
    assert_eq!(min_len(&[]), 0);
    assert_eq!(min_len(&[digit(0)]), 0);
    assert_eq!(min_len(&[digit(0), digit(0), digit(0)]), 0);
    // A nonzero value keeps up to its most-significant nonzero digit.
    assert_eq!(min_len(&[digit(5)]), 1);
    assert_eq!(min_len(&[digit(5), digit(0)]), 1);
    assert_eq!(min_len(&[digit(1), digit(2), digit(0)]), 2);
    // A zero low digit below a nonzero digit is kept.
    assert_eq!(min_len(&[digit(0), digit(1)]), 2);
    // Nothing to strip.
    assert_eq!(min_len(&[digit(1), digit(2), digit(3)]), 3);
}

#[test]
fn test_min_len_signed() {
    // Zero needs one digit.
    assert_eq!(min_len_signed(&[digit(0)]), 1);
    assert_eq!(min_len_signed(&[digit(0), digit(0)]), 1);
    // A redundant zero sign-extension above a non-negative digit is stripped.
    assert_eq!(min_len_signed(&[digit(5), digit(0)]), 1);
    assert_eq!(min_len_signed(&[digit(1), digit(2), digit(0), digit(0)]), 2);
    // -1 is all-ones; the redundant all-ones sign-extension digits are stripped.
    assert_eq!(min_len_signed(&[Digit::MAX]), 1);
    assert_eq!(min_len_signed(&[Digit::MAX, Digit::MAX, Digit::MAX]), 1);
    // A positive value whose top significant digit has its high bit set needs a leading
    // zero digit, which must be kept.
    assert_eq!(min_len_signed(&[Digit::MAX, digit(0)]), 2);
    // A negative value whose top significant digit has its high bit clear needs a leading
    // all-ones digit, which must be kept.
    assert_eq!(min_len_signed(&[digit(0), Digit::MAX]), 2);
    assert_eq!(min_len_signed(&[digit(5), Digit::MAX]), 2);
}

#[test]
#[should_panic]
fn test_min_len_signed_empty() {
    min_len_signed(&[]);
}

#[test]
fn test_min_len_bytes() {
    assert_eq!(min_len_bytes(&[]), 0);
    assert_eq!(min_len_bytes(&[0]), 0);
    assert_eq!(min_len_bytes(&[0, 0, 0]), 0);
    assert_eq!(min_len_bytes(&[5]), 1);
    assert_eq!(min_len_bytes(&[5, 0]), 1);
    assert_eq!(min_len_bytes(&[1, 2, 0]), 2);
    assert_eq!(min_len_bytes(&[0, 1]), 2);
}

#[test]
fn test_min_len_bytes_signed() {
    // Zero and sign-only sequences collapse to a single byte.
    assert_eq!(min_len_bytes_signed(&[0]), 1);
    assert_eq!(min_len_bytes_signed(&[0, 0, 0]), 1);
    assert_eq!(min_len_bytes_signed(&[0xff, 0xff, 0xff]), 1);
    // Redundant sign bytes above a same-signed byte are stripped.
    assert_eq!(min_len_bytes_signed(&[5, 0]), 1);
    assert_eq!(min_len_bytes_signed(&[0xfe, 0xff]), 1);
    // A leading byte needed to carry the sign is kept: 0xc8 alone is negative, so +200
    // needs the leading zero; 0x80 alone is positive-looking below 0x80, so -32768 needs it.
    assert_eq!(min_len_bytes_signed(&[0xc8, 0x00]), 2); // +200
    assert_eq!(min_len_bytes_signed(&[0x00, 0x80]), 2); // -32768
    assert_eq!(min_len_bytes_signed(&[5, 0, 0, 0]), 1);
}

#[test]
#[should_panic]
fn test_min_len_bytes_signed_empty() {
    min_len_bytes_signed(&[]);
}

#[test]
fn test_le_bytes_round_trip() {
    let inputs: [&[u8]; _] = [
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[1, 2, 3],
        &[],
    ];
    for input in inputs {
        let len = input.len().div_ceil(Digit::BYTES);
        let mut digits = vec![Digit::ZERO; len];
        from_bytes(input, &mut digits);

        // The output is the input zero-padded up to a whole number of digits.
        let mut bytes = vec![0u8; len * Digit::BYTES];
        to_bytes(&digits, &mut bytes);
        assert_eq!(&bytes[..input.len()], input);
        assert!(bytes[input.len()..].iter().all(|&b| b == 0));
        // `input` has no trailing zero byte, so this is its minimal length.
        assert_eq!(min_len_bytes(&bytes), input.len());
    }
}

#[test]
fn test_be_bytes_round_trip() {
    let inputs: [&[u8]; _] = [
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[1, 2, 3],
        &[],
    ];
    for input in inputs {
        let len = input.len().div_ceil(Digit::BYTES);
        let mut digits = vec![Digit::ZERO; len];
        from_be_bytes(input, &mut digits);

        // Reversing the little-endian bytes gives the big-endian bytes, right-aligned with
        // leading zero padding up to a whole number of digits.
        let mut bytes = vec![0u8; len * Digit::BYTES];
        to_bytes(&digits, &mut bytes);
        bytes.reverse();
        let pad = len * Digit::BYTES - input.len();
        assert!(bytes[..pad].iter().all(|&b| b == 0));
        assert_eq!(&bytes[pad..], input);
    }
}

#[test]
fn test_to_bytes() {
    // Exact-length buffer: the digit's bytes only.
    let mut bytes = vec![0xffu8; Digit::BYTES];
    to_bytes(&[Digit::from(0x0102u16)], &mut bytes);
    assert_eq!(&bytes[..2], &[0x02, 0x01]);
    assert!(bytes[2..].iter().all(|&b| b == 0));

    // A longer buffer is zero-extended, including across multiple digits.
    let mut bytes = vec![0xffu8; 2 * Digit::BYTES + 1];
    to_bytes(&[digit(1), digit(2)], &mut bytes);
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[Digit::BYTES], 2);
    bytes[0] = 0;
    bytes[Digit::BYTES] = 0;
    assert!(bytes.iter().all(|&b| b == 0));
}

#[test]
fn test_to_bytes_signed() {
    // A non-negative value zero-extends.
    let mut bytes = vec![0xffu8; Digit::BYTES + 1];
    to_bytes_signed(&[digit(5)], &mut bytes);
    assert_eq!(bytes[0], 5);
    assert!(bytes[1..].iter().all(|&b| b == 0));

    // `Digit::MAX` is -1, so it sign-extends to all ones.
    let mut bytes = vec![0u8; Digit::BYTES + 1];
    to_bytes_signed(&[Digit::MAX], &mut bytes);
    assert!(bytes.iter().all(|&b| b == 0xff));

    // A negative multi-digit value: the top digit's sign fills the high bytes.
    let mut bytes = vec![0u8; 2 * Digit::BYTES + 1];
    to_bytes_signed(&[digit(1), Digit::MAX], &mut bytes);
    assert_eq!(bytes[0], 1);
    assert!(bytes[1..Digit::BYTES].iter().all(|&b| b == 0));
    assert!(bytes[Digit::BYTES..].iter().all(|&b| b == 0xff));
}
