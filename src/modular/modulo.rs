//! Element of modular arithmetic.

use crate::{modular::modulo_ring::ModuloRing, ubig::UBig};

/// Modular arithmetic.
///
/// # Examples
///
/// ```
/// # use ibig::{ModuloRing, ubig};
/// let ring = ModuloRing::new(&ubig!(10000));
/// let x = ring.from(12345);
/// let y = ring.from(55443);
/// assert_eq!((x - y).residue(), ubig!(6902));
/// ```
pub struct Modulo {
    ring: ModuloRing,
    normalized_value: UBig,
}

impl Modulo {
    /// Panics when trying to do operations on [Modulo] values from different rings.
    pub(crate) fn panic_different_rings() -> ! {
        panic!("Modulo values from different rings")
    }

    /// Get the ring of this value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(10000));
    /// let x = ring.from(12345);
    /// assert_eq!(x.ring().modulus(), ubig!(10000));
    /// ```
    pub fn ring(&self) -> &ModuloRing {
        &self.ring
    }

    pub(crate) fn from_normalized_value(normalized_value: UBig, ring: &ModuloRing) -> Modulo {
        debug_assert!(normalized_value < ring.normalized_modulus());
        Modulo {
            ring: ring.clone(),
            normalized_value,
        }
    }

    pub(crate) fn set_normalized_value(&mut self, normalized_value: UBig) {
        debug_assert!(normalized_value < self.ring.normalized_modulus());
        self.normalized_value = normalized_value;
    }

    pub(crate) fn take_normalized_value(&mut self) -> UBig {
        std::mem::take(&mut self.normalized_value)
    }
}

impl Clone for Modulo {
    #[inline]
    fn clone(&self) -> Self {
        Modulo {
            ring: self.ring.clone(),
            normalized_value: self.normalized_value.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.ring.clone_from(&source.ring);
        self.normalized_value.clone_from(&source.normalized_value);
    }
}
