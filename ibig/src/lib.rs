//! A big integer library.

#![no_std]

extern crate alloc;

pub use error::TryFromBigError;
pub use repr::{IBig, UBig};

mod bits;
mod bitwise;
mod bytes;
mod convert;
mod error;
mod macros;
mod misc;
mod repr;
mod sign;

#[cfg(test)]
mod tests;
