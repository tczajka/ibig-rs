//! Conversions to and from [`UBig`].

use crate::{Digits, UBig};
use alloc::{vec, vec::Vec};
use ibig_core::Digit;

impl UBig {
    /// Returns the little-endian (least-significant-first) byte representation, with no
    /// most-significant zero bytes. Zero produces an empty sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[5, 1, 0]).to_le_bytes(), [5, 1]);
    /// assert!(UBig::from_le_bytes(&[]).to_le_bytes().is_empty());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let digits = self.as_digits();
        let mut bytes = vec![0u8; digits.len() * Digit::BYTES];
        ibig_core::to_bytes(digits, &mut bytes);
        bytes.truncate(ibig_core::min_len_bytes(&bytes));
        bytes
    }

    /// Returns the big-endian (most-significant-first) byte representation, with no leading
    /// zero bytes. Zero produces an empty sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_be_bytes(&[0, 1, 5]).to_be_bytes(), [1, 5]);
    /// assert!(UBig::from_be_bytes(&[]).to_be_bytes().is_empty());
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        // Big-endian is the little-endian representation reversed.
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }

    /// Constructs a number from its little-endian (least-significant-first) byte
    /// representation. Any length is accepted, and most-significant zero bytes are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[5, 1]), UBig::from_be_bytes(&[1, 5]));
    /// ```
    pub fn from_le_bytes(bytes: &[u8]) -> UBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_bytes(bytes, &mut digits);
        UBig::from_digits(digits)
    }

    /// Constructs a number from its big-endian (most-significant-first) byte representation.
    /// Any length is accepted, and leading zero bytes are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_be_bytes(&[1, 5]), UBig::from_le_bytes(&[5, 1]));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_be_bytes(bytes, &mut digits);
        UBig::from_digits(digits)
    }
}
