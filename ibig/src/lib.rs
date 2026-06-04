//! A big integer library.

#![no_std]

extern crate alloc;

use ibig_core::Digit;
use smallvec::SmallVec;

pub use error::TryFromBigError;
pub use ibig::IBig;
pub use ubig::UBig;

mod error;
mod ibig;
mod macros;
mod ubig;

/// Number of [`Digit`]s stored inline before the representation spills to the heap.
const INLINE_DIGITS: usize = 4;

/// The number of bits in a [`Digit`], as a `usize`.
const DIGIT_BITS_USIZE: usize = Digit::BITS as usize;

/// Maximum number of [`Digit`]s in a value, chosen so that the total bit length
/// (`MAX_DIGITS * Digit::BITS`) still fits in a `usize`.
const MAX_DIGITS: usize = usize::MAX / DIGIT_BITS_USIZE;

/// Storage for little-endian digits.
///
/// Values of at most [`INLINE_DIGITS`] digits are stored inline; larger values spill to a
/// heap allocation.
pub(crate) type Digits = SmallVec<[Digit; INLINE_DIGITS]>;

#[cfg(test)]
mod tests;
