//! Big integer library.

#![no_std]

extern crate alloc;

pub use crate::{ibig::IBig, ubig::UBig};

mod buffer;
mod convert;
mod ibig;
pub mod radix;
mod ubig;
mod word;
