//! Bit operations on [`UBig`].

use crate::UBig;
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
            Some(digit) => (Digit::BITS - digit.leading_zeros()).try_into().unwrap(),
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
                position < usize::try_from(Digit::BITS).unwrap()
                    && (digit >> position) & Digit::from_u8(1) != Digit::ZERO
            }
            None => ibig_core::bit(self.as_digits(), position),
        }
    }
}
