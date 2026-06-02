//! Conversions to and from [`IBig`].

use crate::{Digits, IBig};
use alloc::{vec, vec::Vec};
use ibig_core::Digit;

impl IBig {
    /// Returns the little-endian (least-significant-first) two's complement byte
    /// representation, with no redundant sign-extension bytes. The result
    /// always has at least one byte (zero produces `[0]`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from_le_bytes(&[5]).to_le_bytes(), [5]);
    /// // -1 is all-ones in two's complement.
    /// assert_eq!(IBig::from_le_bytes(&[0xff]).to_le_bytes(), [0xff]);
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let digits = self.as_digits();
        let mut bytes = vec![0u8; digits.len() * Digit::BYTES];
        ibig_core::to_bytes(digits, &mut bytes);
        bytes.truncate(ibig_core::min_len_bytes_signed(&bytes));
        bytes
    }

    /// Returns the big-endian (most-significant-first) two's complement byte representation,
    /// with no redundant leading sign-extension bytes. The result always has at least one
    /// byte (zero produces `[0]`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from_be_bytes(&[5]).to_be_bytes(), [5]);
    /// assert_eq!(IBig::from_be_bytes(&[0xff]).to_be_bytes(), [0xff]);
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }

    /// Constructs a number from its little-endian (least-significant-first) two's complement
    /// byte representation.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` is empty: a signed value needs at least one byte to carry its sign.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from_le_bytes(&[0xc8, 0]), IBig::from_le_bytes(&[0xc8, 0, 0]));
    /// ```
    pub fn from_le_bytes(bytes: &[u8]) -> IBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_le_bytes_signed(bytes, &mut digits);
        IBig::from_digits(digits)
    }

    /// Constructs a number from its big-endian (most-significant-first) two's complement
    /// byte representation.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` is empty: a signed value needs at least one byte to carry its sign.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from_be_bytes(&[1, 5]), IBig::from_le_bytes(&[5, 1]));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> IBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_be_bytes_signed(bytes, &mut digits);
        IBig::from_digits(digits)
    }
}
