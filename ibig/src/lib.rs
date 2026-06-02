//! A big integer library.

#![no_std]

extern crate alloc;

use ibig_core::Digit;
use smallvec::SmallVec;

pub use ibig::IBig;
pub use ubig::UBig;

mod ibig;
mod ubig;

/// Number of [`Digit`]s stored inline before the representation spills to the heap.
const INLINE_DIGITS: usize = 4;

/// Storage for little-endian digits.
///
/// Values of at most [`INLINE_DIGITS`] digits are stored inline; larger values spill to a
/// heap allocation.
pub(crate) type Digits = SmallVec<[Digit; INLINE_DIGITS]>;

#[cfg(test)]
mod tests;
