//! Signed big integer.

use crate::{
    primitive::Sign::{self, *},
    ubig::UBig,
};

/// Signed big integer.
#[derive(Eq, PartialEq)]
pub struct IBig {
    sign: Sign,
    magnitude: UBig,
}

impl IBig {
    pub(crate) fn from_sign_magnitude(mut sign: Sign, magnitude: UBig) -> IBig {
        if magnitude.is_zero() {
            sign = Positive;
        }
        IBig { sign, magnitude }
    }

    pub(crate) fn sign(&self) -> Sign {
        self.sign
    }

    pub(crate) fn magnitude(&self) -> &UBig {
        &self.magnitude
    }

    pub(crate) fn into_sign_magnitude(self) -> (Sign, UBig) {
        (self.sign, self.magnitude)
    }

    /// Is the number smaller than 0?
    pub fn is_negative(&self) -> bool {
        self.sign == Negative
    }

    /// Is the number greater than 0?
    pub fn is_positive(&self) -> bool {
        self.sign == Positive && !self.magnitude.is_zero()
    }
}
