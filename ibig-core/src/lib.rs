//! Core big-integer algorithms.
//!
//! This crate implements the fundamental arithmetic algorithms — addition, subtraction,
//! multiplication and division — operating on sequences of [`Digit`]s.

#![no_std]

mod bits;
mod bytes;
mod len;
mod sign;

pub use bits::{
    bit, bit_signed, bit_width, is_power_of_two, next_power_of_two_in_place, set_bit,
    trailing_ones, trailing_zeros,
};
pub use bytes::{
    from_be_bytes, from_be_bytes_signed, from_bytes, from_bytes_signed, to_bytes, to_bytes_signed,
};
pub use len::{min_len, min_len_bytes, min_len_bytes_signed, min_len_signed};
pub use sign::{extend_signed, is_negative};
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
