//! Big integer library.

#![no_std]

extern crate alloc;

pub use crate::{ibig::IBig, ubig::UBig};

mod buffer;
mod convert;
pub mod fmt;
mod ibig;
mod radix;
mod ubig;
mod word;
