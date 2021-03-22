//! Modular arithmetic.
//!
//! Modular arithmetic is performed on values `Modulo` attached to a modular ring of integers,
//! `ModuloRing`.
//!
//! Trying to mix different rings (even with the same modulus!) will cause a panic.
//!
//! # Examples
//!
//! ```
//! use ibig::{prelude::*, modular::ModuloRing};
//!
//! let ring = ModuloRing::new(&ubig!(10000));
//! let x = ring.from(ubig!(12345));
//! let y = -x;
//! assert_eq!(y.residue(), ubig!(7655));
//! ```

pub use convert::ToModulo;
pub use modulo::Modulo;
pub use modulo_ring::ModuloRing;

mod add;
mod cmp;
pub(crate) mod convert;
pub(crate) mod modulo;
pub(crate) mod modulo_ring;
