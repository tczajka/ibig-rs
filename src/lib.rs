//! Big integer library.

#![no_std]

#[cfg_attr(test, macro_use)]
extern crate alloc;

mod ubig;
mod word;

pub use crate::ubig::UBig;
