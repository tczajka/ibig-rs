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

    /// Returns a number representing the sign of `self`:
    /// * `-1` if negative
    /// * `0` if zero
    /// * `1` if positive
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(-5i8).signum(), IBig::from(-1i8));
    /// assert_eq!(IBig::ZERO.signum(), IBig::ZERO);
    /// assert_eq!(IBig::from(5i8).signum(), IBig::from(1i8));
    /// ```
    #[inline]
    pub fn signum(&self) -> IBig {
        match self.try_to_digit() {
            Some(digit) => IBig::from_digit(digit.signum()),
            None => {
                // A multi-digit value is never zero.
                if ibig_core::is_negative(self.as_digits()) {
                    IBig::from_i8(-1)
                } else {
                    IBig::from_i8(1)
                }
            }
        }
    }
}
