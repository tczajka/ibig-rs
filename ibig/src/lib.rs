//! A big integer library.

#![no_std]

extern crate alloc;

pub use error::TryFromBigError;
pub use repr::{IBig, UBig};

mod bits;
mod bytes;
mod error;
mod ibig;
mod macros;
mod misc;
mod repr;
mod sign;
mod ubig;

#[cfg(test)]
mod tests;
