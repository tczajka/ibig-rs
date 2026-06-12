//! Core big-integer algorithms.
//!
//! This crate implements the fundamental arithmetic algorithms — addition, subtraction,
//! multiplication and division — operating on sequences of [`Digit`]s.
//!
//! Numbers are represented as slices of [`Digit`]s in little-endian order — the
//! least-significant digit first — regardless of the platform's native byte order (so the
//! layout is the same on big-endian machines).
//!
//! Unless a function's documentation states otherwise, the digits are interpreted as an
//! unsigned (non-negative) value. Functions that work with signed two's complement values say
//! so explicitly (and typically have a `_signed` suffix).

#![no_std]

mod add;
mod bits;
mod bitwise;
mod bytes;
mod len;
mod shift;
mod sign;

pub use add::{add, add_carry, add_digit, add_same_len};
pub use bits::{
    BitIndex, BitIndexOutOfRange, DIGIT_BITS_USIZE, bit, bit_signed, count_ones, highest_one,
    is_power_of_two, lowest_one, lowest_zero, next_power_of_two, set_bit,
};
pub use bitwise::{bitand_same_len, bitandnot_same_len, bitor_same_len, bitxor_same_len, not};
pub use bytes::{
    from_be_bytes, from_be_bytes_signed, from_bytes, from_bytes_signed, to_bytes, to_bytes_signed,
};
pub use len::{min_len, min_len_bytes, min_len_bytes_signed, min_len_signed};
pub use shift::{
    shl_small, shl_small_digit, shl_small_signed, shl_small_signed_digit, shr_small,
    shr_small_signed,
};
pub use sign::{
    extend_signed, extend_signed_bytes, is_negative, sign_extension, sign_extension_byte,
};
use unative::{INative, UNative};

/// A single digit of a big integer.
///
/// Big integers are represented as little-endian sequences of `Digit`s. A `Digit` is the
/// platform's native unsigned integer type ([`UNative`]), chosen for efficient hardware
/// arithmetic.
pub type Digit = UNative;

/// A [`Digit`] interpreted as a signed (two's complement) value.
///
/// This is the platform's native signed integer type ([`INative`]), the same width as
/// [`Digit`]. It is used as the most-significant digit of a signed two's complement
/// number, where the sign bit lives.
pub type SignedDigit = INative;
