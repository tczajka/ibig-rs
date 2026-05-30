//! A big integer library.

#![no_std]

extern crate alloc;

pub use ubig::UBig;

mod ubig;

#[cfg(test)]
mod tests;
