//! Bit operations on [`UBig`].

use crate::{DIGIT_BITS_USIZE, UBig};
use core::mem;
use ibig_core::Digit;

impl UBig {
    /// Returns the number of bits needed to represent the value: the position of the
    /// most-significant set bit plus one, or 0 for the value zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0u8).bit_width(), 0);
    /// assert_eq!(UBig::from(1u8).bit_width(), 1);
    /// assert_eq!(UBig::from(0b101u8).bit_width(), 3);
    /// ```
    #[inline]
    pub fn bit_width(&self) -> usize {
        match self.try_to_digit() {
            Some(digit) => DIGIT_BITS_USIZE - usize::try_from(digit.leading_zeros()).unwrap(),
            None => ibig_core::bit_width(self.as_digits()),
        }
    }

    /// Returns the base-2 logarithm, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if the value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(1u8).ilog2(), 0);
    /// assert_eq!(UBig::from(0b101u8).ilog2(), 2);
    /// ```
    #[inline]
    pub fn ilog2(&self) -> usize {
        let width = self.bit_width();
        assert!(width != 0, "argument of ilog2 must be positive");
        width - 1
    }

    /// Returns the bit at `position`, counting from the least-significant bit. Positions at or
    /// above [`bit_width`](UBig::bit_width) read as `false`, since the value is zero-extended.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert!(UBig::from(0b10010u8).bit(1));
    /// assert!(!UBig::from(0b10010u8).bit(3));
    /// assert!(!UBig::from(0b10010u8).bit(100));
    /// ```
    #[inline]
    pub fn bit(&self, position: usize) -> bool {
        match self.try_to_digit() {
            Some(digit) => {
                position < DIGIT_BITS_USIZE
                    && (digit >> position) & Digit::from_u8(1) != Digit::ZERO
            }
            None => ibig_core::bit(self.as_digits(), position),
        }
    }

    /// Sets the bit at `position`, counting from the least-significant bit, to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// let mut a = UBig::from(0b100u8);
    /// a.set_bit(0, true);
    /// assert_eq!(a, UBig::from(0b101u8));
    /// a.set_bit(2, false);
    /// assert_eq!(a, UBig::from(0b001u8));
    /// ```
    pub fn set_bit(&mut self, position: usize, value: bool) {
        // Fast path: a single digit that stays a single digit.
        if let Some(digit) = self.try_to_digit()
            && position < DIGIT_BITS_USIZE
        {
            let mask = Digit::from_u8(1) << position;
            *self = UBig::from_digit(if value { digit | mask } else { digit & !mask });
            return;
        }

        // Slow path: the value or position spans multiple digits.
        let digit_index = position / DIGIT_BITS_USIZE;
        let mut digits = mem::replace(self, UBig::ZERO).into_digits();
        if digit_index >= digits.len() {
            if value {
                digits.resize(digit_index + 1, Digit::ZERO);
                ibig_core::set_bit(&mut digits, position, true);
            }
        } else {
            ibig_core::set_bit(&mut digits, position, value);
        }
        *self = UBig::from_digits(digits);
    }
}
