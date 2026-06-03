//! Bit-level queries on [`IBig`].

use crate::IBig;
use ibig_core::Digit;

impl IBig {
    /// Returns the bit at `position` of the two's complement representation, counting from the
    /// least-significant bit. Positions at or above the stored width read as the sign bit,
    /// since the value is sign-extended: `false` for a non-negative value and `true` for a
    /// negative one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// // 0b10010 = 18.
    /// assert!(IBig::from(0b10010i8).bit(1));
    /// assert!(!IBig::from(0b10010i8).bit(0));
    /// // -1 is all ones, including the sign-extended positions.
    /// assert!(IBig::from(-1i8).bit(100));
    /// // A non-negative value reads as zero above its bits.
    /// assert!(!IBig::from(2i8).bit(100));
    /// ```
    #[inline]
    pub fn bit(&self, position: usize) -> bool {
        match self.try_to_digit() {
            Some(digit) => {
                if position < usize::try_from(Digit::BITS).unwrap() {
                    (digit.cast_unsigned() >> position) & Digit::from_u8(1) != Digit::ZERO
                } else {
                    // Positions above the digit read the sign bit.
                    digit.is_negative()
                }
            }
            None => ibig_core::bit_signed(self.as_digits(), position),
        }
    }
}
