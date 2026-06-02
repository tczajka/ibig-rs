//! Conversions to and from [`UBig`].

use crate::{Digits, IBig, INLINE_DIGITS, TryFromBigError, UBig};
use alloc::{vec, vec::Vec};
use core::num::TryFromIntError;
use ibig_core::Digit;

impl UBig {
    /// The number zero.
    pub const ZERO: UBig = UBig::from_digit(Digit::ZERO);

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
        match Digit::try_from_u32(value) {
            Some(digit) => UBig::from_digit(digit),
            None => UBig::const_from_le_bytes(&value.to_le_bytes()),
        }
    }

    /// Constructs from a `u64`.
    #[inline]
    pub const fn from_u64(value: u64) -> UBig {
        match Digit::try_from_u64(value) {
            Some(digit) => UBig::from_digit(digit),
            None => UBig::const_from_le_bytes(&value.to_le_bytes()),
        }
    }

    /// Returns the little-endian (least-significant-first) byte representation, with no
    /// most-significant zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0x0105u16).to_le_bytes(), [5, 1]);
    /// assert_eq!(UBig::from(0u8).to_le_bytes(), []);
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let digits = self.as_digits();
        let mut bytes = vec![0u8; digits.len() * Digit::BYTES];
        ibig_core::to_bytes(digits, &mut bytes);
        bytes.truncate(ibig_core::min_len_bytes(&bytes));
        bytes
    }

    /// Returns the big-endian (most-significant-first) byte representation, with no leading
    /// zero bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0x0105u16).to_be_bytes(), [1, 5]);
    /// assert_eq!(UBig::from(0u8).to_be_bytes(), []);
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
    /// assert_eq!(UBig::from_le_bytes(&[5, 1]), UBig::from(0x0105u16));
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
    /// assert_eq!(UBig::from_be_bytes(&[1, 5]), UBig::from(0x0105u16));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        let mut digits = Digits::new();
        digits.resize(bytes.len().div_ceil(Digit::BYTES), Digit::ZERO);
        ibig_core::from_be_bytes(bytes, &mut digits);
        UBig::from_digits(digits)
    }
}

/// Implements `From<$t> for UBig` for an unsigned primitive: a value that fits in a single
/// digit takes the fast path, otherwise it goes through the little-endian bytes.
macro_rules! impl_from_unsigned {
    ($t:ty) => {
        impl From<$t> for UBig {
            #[inline]
            fn from(value: $t) -> UBig {
                match Digit::try_from(value) {
                    Ok(digit) => UBig::from_digit(digit),
                    Err(_) => UBig::from_le_bytes(&value.to_le_bytes()),
                }
            }
        }
    };
}

impl_from_unsigned!(u8);
impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(u128);
impl_from_unsigned!(usize);

/// Implements `TryFrom<$signed> for UBig` by forwarding through the unsigned `$unsigned`: a
/// non-negative value converts, while a negative value yields the same `TryFromIntError`
/// that the unsigned conversion produces.
macro_rules! impl_try_from_signed {
    ($signed:ty => $unsigned:ty) => {
        impl TryFrom<$signed> for UBig {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: $signed) -> Result<UBig, TryFromIntError> {
                <$unsigned>::try_from(value).map(UBig::from)
            }
        }
    };
}

impl_try_from_signed!(i8 => u8);
impl_try_from_signed!(i16 => u16);
impl_try_from_signed!(i32 => u32);
impl_try_from_signed!(i64 => u64);
impl_try_from_signed!(i128 => u128);
impl_try_from_signed!(isize => usize);

impl TryFrom<IBig> for UBig {
    type Error = TryFromBigError;

    #[inline]
    fn try_from(value: IBig) -> Result<UBig, TryFromBigError> {
        if value.is_negative() {
            return Err(TryFromBigError);
        }
        // A non-negative two's complement value's digits are its unsigned magnitude.
        if let Some(digit) = value.try_to_digit() {
            // Fast path: a single non-negative digit.
            Ok(UBig::from_digit(digit.cast_unsigned()))
        } else {
            Ok(UBig::from_digits(value.into_digits()))
        }
    }
}

impl TryFrom<&IBig> for UBig {
    type Error = TryFromBigError;

    #[inline]
    fn try_from(value: &IBig) -> Result<UBig, TryFromBigError> {
        // Fast path to avoid cloning.
        if value.is_negative() {
            return Err(TryFromBigError);
        }
        UBig::try_from(value.clone())
    }
}
