//! A big integer library.

#![no_std]

extern crate alloc;

mod ubig;

#[cfg(test)]
mod tests;

pub use ubig::UBig;
