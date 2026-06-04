//! Conversions to and from [`IBig`].

use crate::{IBig, TryFromBigError, UBig};
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

/// Converts a `bool`: `false` to zero and `true` to one.
impl From<bool> for IBig {
    #[inline]
    fn from(value: bool) -> IBig {
        IBig::from_digit(SignedDigit::from(value))
    }
}

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

/// Converts to `bool`: zero is `false`, one is `true`, anything else is out of range.
impl TryFrom<&IBig> for bool {
    type Error = TryFromBigError;

    #[inline]
    fn try_from(value: &IBig) -> Result<bool, TryFromBigError> {
        value
            .try_to_digit()
            .ok_or(TryFromBigError)?
            .try_into()
            .map_err(|_| TryFromBigError)
    }
}

impl_try_into_by_value!(bool);
