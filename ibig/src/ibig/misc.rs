//! Miscellaneous standard trait implementations for [`IBig`].

use crate::IBig;

/// The default value is zero.
impl Default for IBig {
    #[inline]
    fn default() -> IBig {
        IBig::ZERO
    }
}
