//! A ring of integers modulo a positive integer.

use crate::{fast_divide::FastDivideNormalized, ubig::UBig};
use alloc::sync::Arc;

/// A ring of integers modulo a positive integer.
///
/// # Examples
///
/// ```
/// # use ibig::{ModuloRing, ubig};
/// let ring = ModuloRing::new(&ubig!(100));
/// assert_eq!(ring.modulus(), ubig!(100));
/// ```
#[derive(Clone)]
pub struct ModuloRing(Arc<ModuloRingRepr>);

struct ModuloRingRepr {
    normalized_modulus: UBig,
    shift: u32,
    fast_div_top: FastDivideNormalized,
}

impl ModuloRing {
    /// Create a new ring of integers modulo `n`.
    ///
    /// For two [Modulo](crate::Modulo) numbers to be compatible,
    /// they must come from the same [ModuloRing].
    /// Two different [ModuloRing]s are not compatible even if
    /// they have the same modulus `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// assert_eq!(ring.modulus(), ubig!(100));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `n` is zero.
    #[inline]
    pub fn new(n: &UBig) -> ModuloRing {
        let last = n.as_words().last().expect("ModuloRing::new(0)");
        let shift = last.leading_zeros();
        let normalized_modulus = n << shift;
        let fast_div_top = FastDivideNormalized::new(last);
        ModuloRing(Arc::new(ModuloRingRepr {
            normalized_modulus,
            shift,
            fast_div_top,
        }))
    }

    #[inline]
    pub(crate) fn normalized_modulus(&self) -> &UBig {
        &self.normalized_modulus
    }

    #[inline]
    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }

    #[inline]
    pub(crate) fn fast_div_top(&self) -> FastDivideNormalized {
        self.fast_div_top
    }
}
