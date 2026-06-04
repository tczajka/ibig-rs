//! Byte-sequence conversions for [`IBig`].

use crate::{Digits, IBig, INLINE_DIGITS};
use alloc::{vec, vec::Vec};
use ibig_core::Digit;

impl IBig {
    /// Returns the little-endian (least-significant-first) two's complement byte
    /// representation, with no redundant sign-extension bytes. The result
    /// always has at least one byte.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0i8).to_le_bytes(), [0]);
    /// assert_eq!(IBig::from(0x0102i16).to_le_bytes(), [2, 1]);
    /// assert_eq!(IBig::from(0xffffi32).to_le_bytes(), [0xff, 0xff, 0]);
    /// assert_eq!(IBig::from(-1i8).to_le_bytes(), [0xff]);
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
    /// byte.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0i8).to_be_bytes(), [0]);
    /// assert_eq!(IBig::from(0x0102i16).to_be_bytes(), [1, 2]);
    /// assert_eq!(IBig::from(0xffffi32).to_be_bytes(), [0, 0xff, 0xff]);
    /// assert_eq!(IBig::from(-1i8).to_be_bytes(), [0xff]);
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
    /// assert_eq!(IBig::from_le_bytes(&[1, 2, 0]), IBig::from(0x0201i16));
    /// assert_eq!(IBig::from_le_bytes(&[0xff]), IBig::from(-1i8));
    /// ```
    pub fn from_le_bytes(bytes: &[u8]) -> IBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_bytes_signed(bytes, &mut digits);
        IBig::from_digits(digits)
    }

    /// Constructs from at most `INLINE_DIGITS * Digit::BYTES` little-endian two's complement
    /// bytes.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` is empty or longer than `INLINE_DIGITS * Digit::BYTES`.
    #[inline]
    pub(crate) const fn const_from_le_bytes(bytes: &[u8]) -> IBig {
        assert!(bytes.len() <= INLINE_DIGITS * Digit::BYTES);
        let mut digits = [Digit::ZERO; INLINE_DIGITS];
        let n = bytes.len().div_ceil(Digit::BYTES);
        let (used, _) = digits.split_at_mut(n);
        ibig_core::from_bytes_signed(bytes, used);
        IBig::const_from_digits(used)
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
    /// assert_eq!(IBig::from_be_bytes(&[0, 1, 2]), IBig::from(0x0102i16));
    /// assert_eq!(IBig::from_be_bytes(&[0xff]), IBig::from(-1i8));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> IBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_be_bytes_signed(bytes, &mut digits);
        IBig::from_digits(digits)
    }
}
