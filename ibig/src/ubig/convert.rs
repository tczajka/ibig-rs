//! Conversions to and from [`UBig`].

use crate::{IBig, TryFromBigError, UBig};
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

/// Converts a `bool`: `false` to zero and `true` to one.
impl From<bool> for UBig {
    #[inline]
    fn from(value: bool) -> UBig {
        UBig::from_digit(Digit::from(value))
    }
}

/// Converts a `char` to its Unicode scalar value (code point).
impl From<char> for UBig {
    #[inline]
    fn from(value: char) -> UBig {
        UBig::from(u32::from(value))
    }
}

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

/// Forwards `TryFrom<UBig> for $t` to the by-reference conversion.
macro_rules! impl_try_into_by_value {
    ($t:ty) => {
        impl TryFrom<UBig> for $t {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: UBig) -> Result<$t, TryFromBigError> {
                <$t>::try_from(&value)
            }
        }
    };
}

/// Implements `TryFrom<&UBig> for $t` for an unsigned primitive. A single-digit value is
/// converted directly; a larger value is read from its little-endian bytes.
macro_rules! impl_try_into_unsigned {
    ($t:ty) => {
        impl TryFrom<&UBig> for $t {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: &UBig) -> Result<$t, TryFromBigError> {
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

/// Implements `TryFrom<&UBig> for $signed` for a signed primitive: a single-digit value is
/// converted directly; otherwise the value is converted to the same-width unsigned type
/// `$unsigned` and then narrowed (which rejects values past the signed maximum).
macro_rules! impl_try_into_signed {
    ($signed:ty => $unsigned:ty) => {
        impl TryFrom<&UBig> for $signed {
            type Error = TryFromBigError;

            #[inline]
            fn try_from(value: &UBig) -> Result<$signed, TryFromBigError> {
                // Fast path: a single digit.
                if let Some(digit) = value.try_to_digit() {
                    return <$signed>::try_from(digit).map_err(|_| TryFromBigError);
                }
                // Slow path.
                let unsigned = <$unsigned>::try_from(value)?;
                <$signed>::try_from(unsigned).map_err(|_| TryFromBigError)
            }
        }

        impl_try_into_by_value!($signed);
    };
}

impl_try_into_signed!(i8 => u8);
impl_try_into_signed!(i16 => u16);
impl_try_into_signed!(i32 => u32);
impl_try_into_signed!(i64 => u64);
impl_try_into_signed!(i128 => u128);
impl_try_into_signed!(isize => usize);

/// Converts to `bool`: zero is `false`, one is `true`, anything else is out of range.
impl TryFrom<&UBig> for bool {
    type Error = TryFromBigError;

    #[inline]
    fn try_from(value: &UBig) -> Result<bool, TryFromBigError> {
        value
            .try_to_digit()
            .ok_or(TryFromBigError)?
            .try_into()
            .map_err(|_| TryFromBigError)
    }
}

impl_try_into_by_value!(bool);
