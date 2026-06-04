//! Miscellaneous standard trait implementations for [`UBig`] and [`IBig`].

use crate::{IBig, UBig};

/// The default value is zero.
impl Default for UBig {
    #[inline]
    fn default() -> UBig {
        UBig::ZERO
    }
}

/// The default value is zero.
impl Default for IBig {
    #[inline]
    fn default() -> IBig {
        IBig::ZERO
    }
}
