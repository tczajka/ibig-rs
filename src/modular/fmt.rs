//! Formatting modular rings and modular numbers.

use crate::modular::{modulo::Modulo, modulo_ring::ModuloRing};
use core::fmt::{self, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};

macro_rules! impl_fmt {
    ($t:ident) => {
        impl $t for ModuloRing {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("mod ")?;
                $t::fmt(&self.modulus(), f)
            }
        }

        impl $t for Modulo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                $t::fmt(&self.residue(), f)?;
                f.write_str(" (")?;
                $t::fmt(self.ring(), f)?;
                f.write_str(")")
            }
        }
    };
}

impl_fmt!(Display);
impl_fmt!(Debug);
impl_fmt!(Binary);
impl_fmt!(Octal);
impl_fmt!(LowerHex);
impl_fmt!(UpperHex);
