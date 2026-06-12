//! Sign operations on [`IBig`].

use crate::IBig;
use crate::repr::{
    AsDigits,
    AsDigitsResult::{Large, Small},
};
use ibig_core::Digit;

impl IBig {
    /// Returns `true` if the number is negative (less than zero).
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self.as_digits() {
            Small(digit) => digit.is_negative(),
            Large(digits) => ibig_core::is_negative(digits),
        }
    }

    /// Returns `true` if the number is positive (greater than zero).
    #[inline]
    pub fn is_positive(&self) -> bool {
        match self.as_digits() {
            Small(digit) => digit.is_positive(),
            // A multi-digit value is never zero, so it is positive iff not negative.
            Large(digits) => !ibig_core::is_negative(digits),
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
        match self.as_digits() {
            Small(digit) => IBig::from_digit(digit.signum()),
            Large(digits) => IBig::signum_ref(digits),
        }
    }

    /// [`IBig::signum`] for a borrowed slice.
    #[inline]
    fn signum_ref(digits: &[Digit]) -> IBig {
        // A multi-digit value is never zero.
        if ibig_core::is_negative(digits) {
            IBig::from(-1i8)
        } else {
            IBig::from(1i8)
        }
    }
}
