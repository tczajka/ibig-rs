//! Conversions to and from [`IBig`].

use crate::{Digits, IBig, INLINE_DIGITS, TryFromBigError, UBig};
use alloc::{vec, vec::Vec};
use ibig_core::{Digit, SignedDigit};

impl IBig {
    /// The number zero.
    pub const ZERO: IBig = IBig::from_digit(SignedDigit::ZERO);

    /// Constructs from an `i8`.
    #[inline]
    pub const fn from_i8(value: i8) -> IBig {
        IBig::from_digit(SignedDigit::from_i8(value))
    }

    /// Constructs from an `i16`.
    #[inline]
    pub const fn from_i16(value: i16) -> IBig {
        IBig::from_digit(SignedDigit::from_i16(value))
    }

    /// Constructs from an `i32`.
    #[inline]
    pub const fn from_i32(value: i32) -> IBig {
        match SignedDigit::try_from_i32(value) {
            Some(digit) => IBig::from_digit(digit),
            None => IBig::const_from_le_bytes(&value.to_le_bytes()),
        }
    }

    /// Constructs from an `i64`.
    #[inline]
    pub const fn from_i64(value: i64) -> IBig {
        match SignedDigit::try_from_i64(value) {
            Some(digit) => IBig::from_digit(digit),
            None => IBig::const_from_le_bytes(&value.to_le_bytes()),
        }
    }

    /// Returns `true` if the number is negative (less than zero).
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self.try_to_digit() {
            Some(digit) => digit.is_negative(),
            None => ibig_core::is_negative(self.as_digits()),
        }
    }

    /// Returns `true` if the number is positive (greater than zero).
    #[inline]
    pub fn is_positive(&self) -> bool {
        match self.try_to_digit() {
            Some(digit) => digit.is_positive(),
            // A multi-digit value is never zero, so it is positive iff not negative.
            None => !ibig_core::is_negative(self.as_digits()),
        }
    }

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

/// Implements `From<$t> for IBig` for a signed primitive: a value that fits in a single
/// digit takes the fast path, otherwise it goes through the little-endian bytes.
macro_rules! impl_from_signed {
    ($t:ty) => {
        impl From<$t> for IBig {
            #[inline]
            fn from(value: $t) -> IBig {
                match SignedDigit::try_from(value) {
                    Ok(digit) => IBig::from_digit(digit),
                    Err(_) => IBig::from_le_bytes(&value.to_le_bytes()),
                }
            }
        }
    };
}

impl_from_signed!(i8);
impl_from_signed!(i16);
impl_from_signed!(i32);
impl_from_signed!(i64);
impl_from_signed!(i128);
impl_from_signed!(isize);

impl From<UBig> for IBig {
    #[inline]
    fn from(value: UBig) -> IBig {
        // Fast path: a single digit whose sign bit is clear is already a canonical
        // non-negative two's complement digit.
        if let Some(digit) = value.try_to_digit() {
            let signed = digit.cast_signed();
            if !signed.is_negative() {
                return IBig::from_digit(signed);
            }
        }

        // Slow path.
        let mut digits = value.into_digits();
        // The unsigned digits are non-negative. If the most-significant digit's sign bit is
        // set, the two's complement reading would be negative, so append a zero digit.
        if ibig_core::is_negative(&digits) {
            digits.push(Digit::ZERO);
        }
        IBig::from_digits(digits)
    }
}

impl From<&UBig> for IBig {
    #[inline]
    fn from(value: &UBig) -> IBig {
        IBig::from(value.clone())
    }
}

/// Implements `From<$t> for IBig` for an unsigned primitive by converting through `UBig`.
macro_rules! impl_from_unsigned {
    ($t:ty) => {
        impl From<$t> for IBig {
            #[inline]
            fn from(value: $t) -> IBig {
                IBig::from(UBig::from(value))
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

/// Forwards `TryFrom<IBig> for $t` to the by-reference conversion.
macro_rules! impl_try_into_by_value {
    ($t:ty) => {
        impl TryFrom<IBig> for $t {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: IBig) -> Result<$t, TryFromBigError> {
                <$t>::try_from(&value)
            }
        }
    };
}

/// Implements `TryFrom<&IBig> for $t` for a signed primitive. A single-digit value is
/// converted directly; a larger value is read from its sign-extended little-endian bytes.
macro_rules! impl_try_into_signed {
    ($t:ty) => {
        impl TryFrom<&IBig> for $t {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: &IBig) -> Result<$t, TryFromBigError> {
                // Fast path: a single digit.
                if let Some(digit) = value.try_to_digit() {
                    return <$t>::try_from(digit).map_err(|_| TryFromBigError);
                }

                const N: usize = size_of::<$t>();
                const {
                    assert!(Digit::BYTES.is_power_of_two());
                    assert!(N.is_power_of_two());
                }

                // The minimum required number of bits is b:
                // b > (len - 1) * Digit::BITS
                // b <= len * Digit::BITS
                //
                // Since len >= 2 and Digit::BITS is a power of two:
                // next_power_of_two(b) = next_power_of_two(len * Digit::BITS)
                //
                // If the number fits, we must have:
                // b <= N * 8
                // next_power_of_two(b) <= N * 8
                // len * Digit::BITS <= N * 8
                // len * Digit::BYTES <= N

                // Compile-time fast path: two-digit values are too large for the target type.
                if 2 * Digit::BYTES > N {
                    return Err(TryFromBigError);
                }

                // Slow path.
                let digits = value.as_digits();
                let num_bytes = digits.len() * Digit::BYTES;
                if num_bytes > N {
                    return Err(TryFromBigError);
                }
                let mut arr = [0u8; N];
                ibig_core::to_bytes_signed(digits, &mut arr);
                Ok(<$t>::from_le_bytes(arr))
            }
        }

        impl_try_into_by_value!($t);
    };
}

impl_try_into_signed!(i8);
impl_try_into_signed!(i16);
impl_try_into_signed!(i32);
impl_try_into_signed!(i64);
impl_try_into_signed!(i128);
impl_try_into_signed!(isize);

/// Implements `TryFrom<&IBig> for $t` for an unsigned primitive. A single-digit value is
/// converted directly; a larger value must be non-negative and is read from the little-endian
/// bytes of its unsigned magnitude.
macro_rules! impl_try_into_unsigned {
    ($t:ty) => {
        impl TryFrom<&IBig> for $t {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: &IBig) -> Result<$t, TryFromBigError> {
                // Fast path: a single digit.
                if let Some(digit) = value.try_to_digit() {
                    return <$t>::try_from(digit).map_err(|_| TryFromBigError);
                }

                // A negative value is out of range for any unsigned type.
                let digits = value.as_digits();
                if ibig_core::is_negative(digits) {
                    return Err(TryFromBigError);
                }
                // A non-negative value carries at most one most-significant sign-extension
                // zero digit; drop it.
                let (&top, rest) = digits.split_last().unwrap();
                let digits = if top == Digit::ZERO { rest } else { digits };

                const N: usize = size_of::<$t>();
                const {
                    assert!(Digit::BYTES.is_power_of_two());
                    assert!(N.is_power_of_two());
                }

                // The minimum required number of bits is b.
                // For len >= 2:
                // b > (len - 1) * Digit::BITS
                // b <= len * Digit::BITS
                //
                // For len = 1 the top digit's high bit is set (since we don't fit in a single
                // signed digit, otherwise we would have used the fast path).
                // b = len * Digit::BITS
                //
                // In either case:
                // next_power_of_two(b) = next_power_of_two(len * Digit::BITS)
                //
                // If the number fits, we must have:
                // b <= N * 8
                // next_power_of_two(b) <= N * 8
                // len * Digit::BITS <= N * 8
                // len * Digit::BYTES <= N
                let num_bytes = digits.len() * Digit::BYTES;
                if num_bytes > N {
                    return Err(TryFromBigError);
                }
                let mut arr = [0u8; N];
                ibig_core::to_bytes(digits, &mut arr);
                Ok(<$t>::from_le_bytes(arr))
            }
        }

        impl_try_into_by_value!($t);
    };
}

impl_try_into_unsigned!(u8);
impl_try_into_unsigned!(u16);
impl_try_into_unsigned!(u32);
impl_try_into_unsigned!(u64);
impl_try_into_unsigned!(u128);
impl_try_into_unsigned!(usize);
