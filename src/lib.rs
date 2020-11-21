//! Big integer library.

#![no_std]

#[cfg_attr(test, macro_use)]
extern crate alloc;

mod ubig;

pub use self::ubig::UBig;
