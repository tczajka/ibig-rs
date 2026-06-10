//! Miscellaneous functionality.

use crate::{IBig, UBig};
use ibig_core::{Digit, SignedDigit};

impl UBig {
    /// The number zero.
    pub const ZERO: UBig = UBig::const_from_digit(Digit::ZERO);
}

impl IBig {
    /// The number zero.
    pub const ZERO: IBig = IBig::const_from_digit(SignedDigit::ZERO);
}

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
