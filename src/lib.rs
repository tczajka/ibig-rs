//! Big integer library.

#![no_std]

extern crate alloc;

pub use crate::{fmt::InRadix, ibig::IBig, ubig::UBig};

mod buffer;
mod convert;
mod fmt;
mod ibig;
mod primitive;
mod radix;
mod ubig;
