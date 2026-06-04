//! Sign operations on [`IBig`].

use crate::IBig;
use crate::repr::AsDigits;

impl IBig {
    /// Returns `true` if the number is negative (less than zero).
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self.try_to_digit() {
            Some(digit) => digit.is_negative(),
            None => ibig_core::is_negative(self.as_digits()),
        }
    }

    /// Returns `true` if the number is positive (greater than zero).
    #[inline]
    pub fn is_positive(&self) -> bool {
        match self.try_to_digit() {
            Some(digit) => digit.is_positive(),
            // A multi-digit value is never zero, so it is positive iff not negative.
            None => !ibig_core::is_negative(self.as_digits()),
        }
    }
}
