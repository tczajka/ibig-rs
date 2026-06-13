//! Integration tests for byte serialization of digit slices.

use ibig_core::{
    Digit, from_be_bytes_unsigned, from_bytes_unsigned, to_bytes_signed, to_bytes_unsigned,
};
use proptest::prelude::*;

fn digit(n: u8) -> Digit {
    Digit::from(n)
}

#[test]
fn test_to_bytes_unsigned() {
    // Exact-length buffer: the digit's bytes only.
    let mut bytes = vec![0xffu8; Digit::BYTES];
    to_bytes_unsigned(&[Digit::from(0x0102u16)], &mut bytes);
    assert_eq!(&bytes[..2], &[0x02, 0x01]);
    assert!(bytes[2..].iter().all(|&b| b == 0));

    // A longer buffer is zero-extended, including across multiple digits.
    let mut bytes = vec![0xffu8; 2 * Digit::BYTES + 1];
    to_bytes_unsigned(&[digit(1), digit(2)], &mut bytes);
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

proptest! {
    #[test]
    fn test_bytes_round_trip(input in proptest::collection::vec(any::<u8>(), 0..50)) {
        let len = input.len().div_ceil(Digit::BYTES);
        let mut digits = vec![Digit::ZERO; len];
        from_bytes_unsigned(&input, &mut digits);

        // `to_bytes_unsigned` reproduces the input, zero-padded up to a whole number of digits.
        let mut bytes = vec![0u8; len * Digit::BYTES];
        to_bytes_unsigned(&digits, &mut bytes);
        prop_assert_eq!(&bytes[..input.len()], &input[..]);
        prop_assert!(bytes[input.len()..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_from_be_bytes_unsigned(input in proptest::collection::vec(any::<u8>(), 0..50)) {
        let len = input.len().div_ceil(Digit::BYTES);
        let mut be = vec![Digit::ZERO; len];
        from_be_bytes_unsigned(&input, &mut be);

        // Reading big-endian bytes equals reading the reversed bytes little-endian.
        let mut reversed = input.clone();
        reversed.reverse();
        let mut le = vec![Digit::ZERO; len];
        from_bytes_unsigned(&reversed, &mut le);

        prop_assert_eq!(be, le);
    }
}
