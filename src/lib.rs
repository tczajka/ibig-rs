//! Big integer library.
//!
//! The library implements arbitrarily large integer arithmetic in pure Rust.
//!
//! The two integer types are [UBig](struct.UBig.html) (for unsigned integers)
//! and [IBig](struct.IBig.html) (for signed integers).
//!
//! Create numbers using the [ubig](macro.ubig.html) and [ibig](macro.ibig.html) macros.
//! ```
//! use ibig::prelude::*;
//! let a = ubig!(0x10ff);
//! let b = ibig!(-abcd base 32);
//! ```
//!
//! Parsing and formatting in any base 2-36 is supported.
//! ```
//! # use ibig::{prelude::*, ParseError};
//! let a = UBig::from_str_radix("10ff", 16)?;
//! assert_eq!(format!("{:^10X}", a), "   10FF   ");
//! assert_eq!(format!("{}", a.in_radix(4)), "1003333");
//! # Ok::<(), ParseError>(())
//! ```
//!
//! Arithmetic operations will soon arrive.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{
    bits::NextPowerOfTwo,
    fmt::InRadix,
    ibig::IBig,
    parse::ParseError,
    primitive::OutOfBoundsError,
    sign::{Abs, UnsignedAbs},
    ubig::UBig,
};

#[macro_use]
mod macros;

mod bits;
mod buffer;
mod cmp;
mod convert;
mod fmt;
mod ibig;
mod parse;
pub mod prelude;
mod primitive;
mod radix;
mod sign;
mod ubig;
