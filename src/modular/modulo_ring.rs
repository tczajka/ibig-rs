//! A ring of integers modulo a positive integer.

use crate::{
    arch::word::Word,
    cmp, div,
    fast_divide::{FastDivide, FastDivideNormalized},
    math,
    ubig::{Repr, UBig},
};
use alloc::vec::Vec;
use core::cmp::Ordering;

/// A ring of integers modulo a positive integer.
///
/// # Example
///
/// ```
/// # use ibig::{prelude::*, modular::ModuloRing};
/// let ring = ModuloRing::new(&ubig!(100));
/// assert_eq!(ring.modulus(), ubig!(100));
/// ```
pub struct ModuloRing(ModuloRingRepr);

pub(crate) enum ModuloRingRepr {
    Small(ModuloRingSmall),
    Large(ModuloRingLarge),
}

pub(crate) struct ModuloRingSmall {
    normalized_modulus: Word,
    shift: u32,
    fast_div: FastDivideNormalized,
}

pub(crate) struct ModuloRingLarge {
    normalized_modulus: Vec<Word>,
    shift: u32,
    fast_div_top: FastDivideNormalized,
}

impl ModuloRing {
    /// Create a new ring of integers modulo `n`.
    ///
    /// For two `Modulo` numbers to be compatible, they must come from the same
    /// `ModuloRing`. Two different `ModuloRing`s are not compatible even if
    /// they have the same modulus `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{prelude::*, modular::ModuloRing};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// assert_eq!(ring.modulus(), ubig!(100));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `n` is zero.
    pub fn new(n: &UBig) -> ModuloRing {
        match n.repr() {
            Repr::Small(0) => panic!("ModuloRing::new(0)"),
            Repr::Small(word) => ModuloRing(ModuloRingRepr::Small(ModuloRingSmall::new(*word))),
            Repr::Large(words) => ModuloRing(ModuloRingRepr::Large(ModuloRingLarge::new(words))),
        }
    }

    pub(crate) fn repr(&self) -> &ModuloRingRepr {
        &self.0
    }
}

impl ModuloRingSmall {
    /// Create a new small ring of integers modulo `n`.
    fn new(n: Word) -> ModuloRingSmall {
        let fast_div = FastDivide::new(n);
        ModuloRingSmall {
            normalized_modulus: n << fast_div.shift,
            shift: fast_div.shift,
            fast_div: fast_div.normalized,
        }
    }

    pub(crate) fn normalized_modulus(&self) -> Word {
        self.normalized_modulus
    }

    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }

    pub(crate) fn fast_div(&self) -> &FastDivideNormalized {
        &self.fast_div
    }

    pub(crate) fn is_valid(&self, val: Word) -> bool {
        val < self.normalized_modulus && val & math::ones::<Word>(self.shift) == 0
    }
}

impl ModuloRingLarge {
    /// Create a new large ring of integers modulo `n`.
    fn new(n: &[Word]) -> ModuloRingLarge {
        let mut normalized_modulus = n.to_vec();
        let (shift, fast_div_top) = div::normalize_large(&mut normalized_modulus);
        ModuloRingLarge {
            normalized_modulus,
            shift,
            fast_div_top,
        }
    }

    pub(crate) fn normalized_modulus(&self) -> &[Word] {
        &self.normalized_modulus
    }

    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }

    pub(crate) fn fast_div_top(&self) -> &FastDivideNormalized {
        &self.fast_div_top
    }

    pub(crate) fn is_valid(&self, val: &[Word]) -> bool {
        val.len() == self.normalized_modulus.len()
            && cmp::cmp_same_len(val, &self.normalized_modulus) == Ordering::Less
            && val[0] & math::ones::<Word>(self.shift) == 0
    }
}
