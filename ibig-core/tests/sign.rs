//! Integration tests for sign operations.

use ibig_core::{Digit, extend_signed, extend_signed_bytes, is_negative, sign_extension};

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_is_negative() {
    let cases: &[(&[Digit], bool)] = &[
        // The sign is the most-significant digit's high bit.
        (&[digit(0)], false),
        (&[digit(5)], false),
        (&[Digit::MAX], true), // -1
        // A negative top digit makes the whole value negative regardless of lower digits.
        (&[digit(5), Digit::MAX], true),
        // A non-negative top digit keeps the value non-negative, even across digits.
        (&[Digit::MAX, digit(0)], false),
    ];
    for &(digits, expected) in cases {
        assert_eq!(is_negative(digits), expected);
    }
}

#[test]
#[should_panic]
fn test_is_negative_empty() {
    is_negative(&[]);
}

#[test]
fn test_extend_signed() {
    // `(value digits, value length, expected after extension)`.
    let cases: &[(&[Digit], usize, &[Digit])] = &[
        // A negative value extends with all-ones.
        (
            &[Digit::MAX, digit(0), digit(0)],
            1,
            &[Digit::MAX, Digit::MAX, Digit::MAX],
        ),
        // A non-negative value extends with zeros.
        (
            &[digit(5), Digit::MAX, Digit::MAX],
            1,
            &[digit(5), digit(0), digit(0)],
        ),
        // The sign comes from the most-significant digit of the value part.
        (
            &[digit(1), Digit::MAX, digit(0)],
            2,
            &[digit(1), Digit::MAX, Digit::MAX],
        ),
        // `len == digits.len()` leaves the buffer unchanged.
        (&[digit(7), digit(8)], 2, &[digit(7), digit(8)]),
    ];
    for &(input, len, expected) in cases {
        let mut digits = input.to_vec();
        extend_signed(&mut digits, len);
        assert_eq!(digits, expected);
    }
}

#[test]
#[should_panic]
fn test_extend_signed_empty() {
    extend_signed(&mut [Digit::ZERO], 0);
}

#[test]
fn test_extend_signed_bytes() {
    // `(value bytes, value length, expected after extension)`.
    let cases: &[(&[u8], usize, &[u8])] = &[
        // A negative value extends with all-ones.
        (&[0xff, 0, 0], 1, &[0xff, 0xff, 0xff]),
        // A non-negative value extends with zeros.
        (&[5, 0xff, 0xff], 1, &[5, 0, 0]),
        // The sign comes from the most-significant byte of the value part.
        (&[1, 0xff, 0], 2, &[1, 0xff, 0xff]),
        // `len == bytes.len()` leaves the buffer unchanged.
        (&[7, 8], 2, &[7, 8]),
    ];
    for &(input, len, expected) in cases {
        let mut bytes = input.to_vec();
        extend_signed_bytes(&mut bytes, len);
        assert_eq!(bytes, expected);
    }
}

#[test]
#[should_panic]
fn test_extend_signed_bytes_empty() {
    extend_signed_bytes(&mut [0u8], 0);
}

#[test]
fn sign_extension_digit() {
    // A negative top digit extends with all-ones; a non-negative one with zeros.
    assert_eq!(sign_extension(Digit::MAX), Digit::MAX); // -1
    assert_eq!(sign_extension(digit(5)), Digit::ZERO);
    assert_eq!(sign_extension(Digit::ZERO), Digit::ZERO);
    // Only the high (sign) bit matters, not the lower bits.
    assert_eq!(sign_extension(Digit::MAX ^ Digit::from(1u8)), Digit::MAX);
}
