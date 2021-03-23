//! Conversion between Modulo, UBig and IBig.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    div,
    ibig::IBig,
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
        modulo_ring::{ModuloRing, ModuloRingLarge, ModuloRingRepr, ModuloRingSmall},
    },
    primitive::extend_word,
    shift,
    sign::Sign::*,
    ubig::{Repr, UBig},
};
use alloc::vec::Vec;
use core::iter;

impl ModuloRing {
    /// The ring modulus.
    ///
    /// # Example
    ///
    /// ```
    /// # use ibig::{prelude::*, modular::ModuloRing};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// assert_eq!(ring.modulus(), ubig!(100));
    /// ```
    pub fn modulus(&self) -> UBig {
        match self.repr() {
            ModuloRingRepr::Small(self_small) => UBig::from_word(self_small.modulus()),
            ModuloRingRepr::Large(self_large) => self_large.modulus(),
        }
    }

    /// Create an element of the ring from another type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{prelude::*, modular::ModuloRing};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// let y = ring.from(ubig!(3366));
    /// assert!(x == y);
    /// ```
    pub fn from<'a, T: ToModulo>(&'a self, x: T) -> Modulo<'a> {
        x.to_modulo(self)
    }
}

impl ModuloRingSmall {
    fn modulus(&self) -> Word {
        self.normalized_modulus() >> self.shift()
    }
}

impl ModuloRingLarge {
    fn modulus(&self) -> UBig {
        let normalized_modulus = self.normalized_modulus();
        let mut buffer = Buffer::allocate(normalized_modulus.len());
        buffer.extend(normalized_modulus);
        let low_bits = shift::shr_in_place(&mut buffer, self.shift());
        assert!(low_bits == 0);
        buffer.into()
    }
}

impl Modulo<'_> {
    /// Get the residue in range `0..n` in an n-element ring.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{prelude::*, modular::ModuloRing};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// assert_eq!(x.residue(), ubig!(66));
    /// ```
    pub fn residue(&self) -> UBig {
        match self.repr() {
            ModuloRepr::Small(self_small) => UBig::from_word(self_small.residue()),
            ModuloRepr::Large(self_large) => self_large.residue(),
        }
    }
}

impl ModuloSmall<'_> {
    fn residue(&self) -> Word {
        self.normalized_value() >> self.ring().shift()
    }
}

impl ModuloLarge<'_> {
    fn residue(&self) -> UBig {
        let words = self.normalized_value();
        let mut buffer = Buffer::allocate(words.len());
        buffer.extend(words);
        let low_bits = shift::shr_in_place(&mut buffer, self.ring().shift());
        assert!(low_bits == 0);
        buffer.into()
    }
}

/// Trait for types that can be converted to `Modulo` in a `ModuloRing`.
pub trait ToModulo {
    fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a>;
}

impl ToModulo for UBig {
    fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
        match ring.repr() {
            ModuloRingRepr::Small(ring_small) => ModuloSmall::from_ubig(&self, ring_small).into(),
            ModuloRingRepr::Large(ring_large) => ModuloLarge::from_ubig(self, ring_large).into(),
        }
    }
}

impl ToModulo for &UBig {
    fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
        match ring.repr() {
            ModuloRingRepr::Small(ring_small) => ModuloSmall::from_ubig(self, ring_small).into(),
            ModuloRingRepr::Large(ring_large) => {
                ModuloLarge::from_ubig(self.clone(), ring_large).into()
            }
        }
    }
}

impl ToModulo for IBig {
    fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
        let (sign, mag) = self.into_sign_magnitude();
        let modulo = mag.to_modulo(ring);
        match sign {
            Positive => modulo,
            Negative => -modulo,
        }
    }
}

impl ToModulo for &IBig {
    fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
        let modulo = self.magnitude().to_modulo(ring);
        match self.sign() {
            Positive => modulo,
            Negative => -modulo,
        }
    }
}

impl<'a> ModuloSmall<'a> {
    fn from_ubig(x: &UBig, ring: &'a ModuloRingSmall) -> ModuloSmall<'a> {
        let rem = match x.repr() {
            Repr::Small(word) => ring.fast_div().div_rem_word(*word).1,
            Repr::Large(words) => div::fast_rem_by_word(words, *ring.fast_div()),
        };
        // Effectively shifts x left by ring.shift().
        let (_, rem) = ring.fast_div().div_rem(extend_word(rem) << ring.shift());
        ModuloSmall::new(rem, ring)
    }
}

impl<'a> ModuloLarge<'a> {
    fn from_ubig(mut x: UBig, ring: &'a ModuloRingLarge) -> ModuloLarge<'a> {
        x <<= ring.shift();
        let modulus = ring.normalized_modulus();
        let mut vec = Vec::with_capacity(modulus.len());
        match x.into_repr() {
            Repr::Small(word) => vec.push(word),
            Repr::Large(mut words) => {
                if words.len() < modulus.len() {
                    vec.extend(&*words);
                } else {
                    let _overflow =
                        div::div_rem_in_place(&mut words, modulus, *ring.fast_div_top());
                    vec.extend(&words[..modulus.len()]);
                }
            }
        }
        vec.extend(iter::repeat(0).take(modulus.len() - vec.len()));
        ModuloLarge::new(vec, ring)
    }
}

/// Implement `ToModulo` for unsigned primitives.
macro_rules! impl_to_modulo_for_unsigned {
    ($t:ty) => {
        impl ToModulo for $t {
            fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
                UBig::from(self).to_modulo(ring)
            }
        }
    };
}

/// Implement `ToModulo` for signed primitives.
macro_rules! impl_to_modulo_for_signed {
    ($t:ty) => {
        impl ToModulo for $t {
            fn to_modulo<'a>(self, ring: &'a ModuloRing) -> Modulo<'a> {
                IBig::from(self).to_modulo(ring)
            }
        }
    };
}

impl_to_modulo_for_unsigned!(bool);
impl_to_modulo_for_unsigned!(u8);
impl_to_modulo_for_unsigned!(u16);
impl_to_modulo_for_unsigned!(u32);
impl_to_modulo_for_unsigned!(u64);
impl_to_modulo_for_unsigned!(u128);
impl_to_modulo_for_unsigned!(usize);
impl_to_modulo_for_signed!(i8);
impl_to_modulo_for_signed!(i16);
impl_to_modulo_for_signed!(i32);
impl_to_modulo_for_signed!(i64);
impl_to_modulo_for_signed!(i128);
impl_to_modulo_for_signed!(isize);
