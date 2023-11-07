//! Comparisons.

use alloc::sync::Arc;

use crate::modular::{modulo::Modulo, modulo_ring::ModuloRing};

/// Two rings are equal only if they are clones of each other.
///
/// # Examples
///
/// ```
/// # use ibig::{ModuloRing, ubig};
/// let ring1 = ModuloRing::new(&ubig!(100));
/// let ring2 = ring1.clone();
/// let ring3 = ModuloRing::new(&ubig!(100));
/// assert_eq!(ring1, ring2);
/// assert_ne!(ring1, ring3);
/// ```
impl PartialEq for ModuloRing {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::eq(self, other)
    }
}

impl Eq for ModuloRing {}

/// Equality within a ring.
///
/// # Panics
///
/// Panics if the two values are from different rings.
impl PartialEq for Modulo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.check_same_ring(other);
        self.normalized_value() == other.normalized_value()
    }
}
