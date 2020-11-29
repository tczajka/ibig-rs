//! Big integer library.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{fmt::InRadix, ibig::IBig, primitive::OutOfBoundsError, ubig::UBig};

mod buffer;
mod convert;
mod fmt;
mod ibig;
mod primitive;
mod radix;
mod ubig;
