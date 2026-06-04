//! A big integer library.

#![no_std]

extern crate alloc;

pub use error::TryFromBigError;
pub use repr::{IBig, UBig};

mod bits;
mod error;
mod ibig;
mod macros;
mod repr;
mod ubig;

#[cfg(test)]
mod tests;
