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
//! let a = ubig!(12345678);
//! let b = ubig!(0x10ff);
//! let c = ibig!(-azz base 36);
//! assert_eq!(c.to_string(), "-14255");
//! ```
//!
//! Parsing and formatting in any base 2-36 is supported.
//! ```
//! # use ibig::{prelude::*, ParseError};
//! let a: UBig = "1503321".parse()?;
//! let b = IBig::from_str_radix("-10ff", 16)?;
//! assert_eq!(format!("{:^10X}", b), "  -10FF   ");
//! assert_eq!(format!("hello {}", b.in_radix(4)), "hello -1003333");
//! # Ok::<(), ParseError>(())
//! ```
//!
//! Standard arithmetic operations are supported on values and on references.
//! ```
//! # use ibig::{prelude::*, ParseError};
//! assert_eq!(ubig!(100) * ubig!(200), ubig!(20000));
//! let a = ubig!(0xff);
//! assert_eq!(&a / &a, ubig!(1));
//! assert_eq!(ibig!(1) << 1000 >> 999, ibig!(2));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use crate::{
    bits::{AndNot, NextPowerOfTwo},
    div::{DivEuclid, DivRem, DivRemEuclid, RemEuclid},
    fmt::InRadix,
    ibig::IBig,
    parse::ParseError,
    primitive::OutOfBoundsError,
    sign::{Abs, UnsignedAbs},
    ubig::UBig,
};

mod add;
mod bits;
mod buffer;
mod cmp;
mod convert;
mod div;
mod fmt;
mod ibig;
mod mul;
mod parse;
mod pow;
pub mod prelude;
mod primitive;
mod radix;
mod shift;
mod sign;
mod ubig;

#[macro_use]
mod macros;

#[cfg(feature = "rand")]
mod random;
#[cfg(feature = "rand")]
pub use crate::random::{UniformIBig, UniformUBig};
