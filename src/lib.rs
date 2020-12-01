//! Big integer library.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{
    fmt::InRadix, ibig::IBig, parse::ParseError, primitive::OutOfBoundsError, ubig::UBig,
};

mod bits;
mod buffer;
mod convert;
mod fmt;
mod ibig;
#[macro_use]
mod macros;
mod parse;
mod primitive;
mod radix;
mod sub;
mod ubig;
