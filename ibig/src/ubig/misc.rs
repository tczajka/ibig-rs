//! Miscellaneous standard trait implementations for [`UBig`].

use crate::UBig;

/// The default value is zero.
impl Default for UBig {
    #[inline]
    fn default() -> UBig {
        UBig::ZERO
    }
}
