//! Conversions to and from [`UBig`].

use crate::{Digits, INLINE_DIGITS, UBig};
use alloc::{vec, vec::Vec};
use ibig_core::Digit;

impl UBig {
    /// Constructs from a `u8`.
    #[inline]
    pub const fn from_u8(value: u8) -> UBig {
        UBig::from_digit(Digit::from_u8(value))
    }

    /// Constructs from a `u16`.
    #[inline]
    pub const fn from_u16(value: u16) -> UBig {
        UBig::from_digit(Digit::from_u16(value))
    }

    /// Constructs from a `u32`.
    #[inline]
    pub const fn from_u32(value: u32) -> UBig {
        if Digit::BITS >= u32::BITS {
            UBig::from_digit(Digit::try_from_u32(value).unwrap())
        } else {
            UBig::const_from_le_bytes(&value.to_le_bytes())
        }
    }

    /// Constructs from a `u64`.
    ///
    /// A `u64` always fits in the inline digit buffer, so this never allocates.
    #[inline]
    pub const fn from_u64(value: u64) -> UBig {
        if Digit::BITS >= u64::BITS {
            UBig::from_digit(Digit::try_from_u64(value).unwrap())
        } else {
            UBig::const_from_le_bytes(&value.to_le_bytes())
        }
    }

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

    /// Constructs from at most `INLINE_DIGITS * Digit::BYTES` little-endian bytes.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` is longer than `INLINE_DIGITS * Digit::BYTES`.
    #[inline]
    pub(crate) const fn const_from_le_bytes(bytes: &[u8]) -> UBig {
        assert!(bytes.len() <= INLINE_DIGITS * Digit::BYTES);
        let mut digits = [Digit::ZERO; INLINE_DIGITS];
        let n = bytes.len().div_ceil(Digit::BYTES);
        let (used, _) = digits.split_at_mut(n);
        ibig_core::from_bytes(bytes, used);
        UBig::const_from_digits(used)
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

impl From<u8> for UBig {
    #[inline]
    fn from(value: u8) -> UBig {
        UBig::from_u8(value)
    }
}

impl From<u16> for UBig {
    #[inline]
    fn from(value: u16) -> UBig {
        UBig::from_u16(value)
    }
}

impl From<u32> for UBig {
    #[inline]
    fn from(value: u32) -> UBig {
        UBig::from_u32(value)
    }
}

impl From<u64> for UBig {
    #[inline]
    fn from(value: u64) -> UBig {
        UBig::from_u64(value)
    }
}

impl From<u128> for UBig {
    #[inline]
    fn from(value: u128) -> UBig {
        UBig::from_le_bytes(&value.to_le_bytes())
    }
}

impl From<usize> for UBig {
    #[inline]
    fn from(value: usize) -> UBig {
        if Digit::BITS >= usize::BITS {
            UBig::from_digit(Digit::try_from(value).unwrap())
        } else {
            UBig::from_le_bytes(&value.to_le_bytes())
        }
    }
}
