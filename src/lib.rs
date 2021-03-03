//! Big integer library.
//!
//! The library implements arbitrarily large integer arithmetic in pure Rust.
//!
//! The two integer types are [UBig](struct.UBig.html) (for unsigned integers)
//! and [IBig](struct.IBig.html) (for signed integers).
//!
//! ```
//! # use ibig::ParseError;
//! use ibig::prelude::*;
//!
//! let a = ubig!(12345678);
//! let b = ubig!(0x10ff);
//! let c = ibig!(-azz base 36);
//! let d: UBig = "15033211231241234523452345345787".parse()?;
//!
//! assert_eq!(c.to_string(), "-14255");
//! assert_eq!(
//!     (a * b.pow(10)).to_str_radix(16),
//!     "1589bda8effbfc495d8d73c83d8b27f94954e"
//! );
//! assert_eq!(
//!     format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
//!     "hello 0x1a7e7c487267d2658a93"
//! );
//! # Ok::<(), ParseError>(())
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
