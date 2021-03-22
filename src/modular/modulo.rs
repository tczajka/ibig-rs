//! Element of modular arithmetic.

use crate::{
    arch::word::Word,
    modular::modulo_ring::{ModuloRingLarge, ModuloRingSmall},
};
use alloc::vec::Vec;

/// Modular arithmetic.
///
/// # Examples
///
/// ```
/// # use ibig::{prelude::*, modular::ModuloRing};
/// let ring = ModuloRing::new(&ubig!(100));
/// let x = ring.from(ibig!(-1234));
/// assert_eq!(x.residue(), ubig!(66));
/// ```
pub struct Modulo<'a>(ModuloRepr<'a>);

pub(crate) enum ModuloRepr<'a> {
    Small(ModuloSmall<'a>),
    Large(ModuloLarge<'a>),
}

#[derive(Clone)]
pub(crate) struct ModuloSmall<'a> {
    ring: &'a ModuloRingSmall,
    normalized_value: Word,
}

pub(crate) struct ModuloLarge<'a> {
    ring: &'a ModuloRingLarge,
    /// normalized_value.len() == ring.normalized_modulus.len()
    normalized_value: Vec<Word>,
}

impl<'a> Modulo<'a> {
    /// Get representation.
    pub(crate) fn repr(&self) -> &ModuloRepr<'a> {
        &self.0
    }

    /// Get mutable representation.
    pub(crate) fn repr_mut(&mut self) -> &mut ModuloRepr<'a> {
        &mut self.0
    }

    /// Panics when trying to do operations on `Modulo` values from different rings.
    pub(crate) fn panic_different_rings() -> ! {
        panic!("Modulo values from different rings")
    }
}

impl<'a> From<ModuloSmall<'a>> for Modulo<'a> {
    fn from(a: ModuloSmall<'a>) -> Self {
        Modulo(ModuloRepr::Small(a))
    }
}

impl<'a> From<ModuloLarge<'a>> for Modulo<'a> {
    fn from(a: ModuloLarge<'a>) -> Self {
        Modulo(ModuloRepr::Large(a))
    }
}

impl<'a> ModuloSmall<'a> {
    /// Create new ModuloSmall.
    ///
    /// normalized_value must be in range 0..modulus and divisible by the shift.
    pub(crate) fn new(normalized_value: Word, ring: &'a ModuloRingSmall) -> Self {
        debug_assert!(ring.is_valid(normalized_value));
        ModuloSmall {
            ring,
            normalized_value,
        }
    }

    /// Get the ring.
    pub(crate) fn ring(&self) -> &'a ModuloRingSmall {
        self.ring
    }

    /// Get normalized value.
    pub(crate) fn normalized_value(&self) -> Word {
        self.normalized_value
    }

    /// Set normalized value.
    pub(crate) fn set_normalized_value(&mut self, val: Word) {
        debug_assert!(self.ring.is_valid(val));
        self.normalized_value = val
    }

    /// Checks that two values are from the same ring.
    pub(crate) fn check_same_ring(&self, other: &ModuloSmall) {
        if self.ring() != other.ring() {
            Modulo::panic_different_rings();
        }
    }
}

impl<'a> ModuloLarge<'a> {
    /// Create new ModuloLarge.
    ///
    /// normalized_value must have the same length as the modulus, be in range 0..modulus,
    /// and be divisible by the shift.
    pub(crate) fn new(normalized_value: Vec<Word>, ring: &'a ModuloRingLarge) -> Self {
        debug_assert!(ring.is_valid(&normalized_value));
        ModuloLarge {
            ring,
            normalized_value,
        }
    }

    /// Get the ring.
    pub(crate) fn ring(&self) -> &'a ModuloRingLarge {
        self.ring
    }

    /// Get normalized value.
    pub(crate) fn normalized_value(&self) -> &[Word] {
        &self.normalized_value
    }

    /// Modify normalized value.
    pub(crate) fn modify_normalized_value<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Word], &ModuloRingLarge),
    {
        f(&mut self.normalized_value, &self.ring);
        debug_assert!(self.ring.is_valid(&self.normalized_value));
    }

    /// Checks that two values are from the same ring.
    pub(crate) fn check_same_ring(&self, other: &ModuloLarge) {
        if self.ring() != other.ring() {
            Modulo::panic_different_rings();
        }
    }
}

impl Clone for Modulo<'_> {
    fn clone(&self) -> Self {
        Modulo(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl Clone for ModuloRepr<'_> {
    fn clone(&self) -> Self {
        match self {
            ModuloRepr::Small(modulo_small) => ModuloRepr::Small(modulo_small.clone()),
            ModuloRepr::Large(modulo_large) => ModuloRepr::Large(modulo_large.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        if let (ModuloRepr::Large(modulo_large), ModuloRepr::Large(source_large)) =
            (&mut *self, source)
        {
            modulo_large.clone_from(source_large);
            return;
        } else {
            *self = source.clone();
        }
    }
}

impl Clone for ModuloLarge<'_> {
    fn clone(&self) -> Self {
        ModuloLarge {
            ring: self.ring,
            normalized_value: self.normalized_value.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ring = source.ring;
        if self.normalized_value.len() == source.normalized_value.len() {
            self.normalized_value
                .copy_from_slice(&source.normalized_value)
        } else {
            // We don't want to have spare capacity, so do not clone_from.
            self.normalized_value = source.normalized_value.clone();
        }
    }
}
