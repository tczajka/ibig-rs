//! Core big-integer algorithms.
//!
//! This crate implements the fundamental arithmetic algorithms — addition, subtraction,
//! multiplication and division — operating on sequences of [`Digit`]s.

#![no_std]

mod convert;

pub use convert::{min_len, min_len_signed};
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
