//! Core big-integer algorithms.
//!
//! This crate implements the fundamental arithmetic algorithms — addition, subtraction,
//! multiplication and division — operating on sequences of [`Digit`]s.

#![no_std]

use unative::UNative;

/// A single digit of a big integer.
///
/// Big integers are represented as little-endian sequences of `Digit`s. A `Digit` is the
/// platform's native unsigned integer type ([`UNative`]), chosen for efficient hardware
/// arithmetic.
pub type Digit = UNative;
