//! Bit-level queries on [`IBig`].

use core::mem;

use crate::{DIGIT_BITS_USIZE, IBig, UBig};
use ibig_core::{Digit, SignedDigit};

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
                if position < DIGIT_BITS_USIZE {
                    (digit.cast_unsigned() >> position) & Digit::from_u8(1) != Digit::ZERO
                } else {
                    // Positions above the digit read the sign bit.
                    digit.is_negative()
                }
            }
            None => ibig_core::bit_signed(self.as_digits(), position),
        }
    }

    /// Sets the bit at `position` of the two's complement representation to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// let mut a = IBig::from(0b100i8);
    /// a.set_bit(0, true);
    /// assert_eq!(a, IBig::from(0b101i8));
    /// // -1 is all ones; clearing bit 0 gives -2.
    /// let mut b = IBig::from(-1i8);
    /// b.set_bit(0, false);
    /// assert_eq!(b, IBig::from(-2i8));
    /// ```
    pub fn set_bit(&mut self, position: usize, value: bool) {
        // Fast path: a single digit whose modification is within a single digit.
        if let Some(digit) = self.try_to_digit()
            && position < DIGIT_BITS_USIZE - 1
        {
            let mask = SignedDigit::from_i8(1) << position;
            let new = if value { digit | mask } else { digit & !mask };
            *self = IBig::from_digit(new);
            return;
        }

        // Slow path.
        // Number of digits needed for the modified bit to sit strictly below the sign bit,
        // i.e. the smallest `min_len` with `position < min_len * DIGIT_BITS_USIZE - 1`.
        // `min_len = (position + 1) / DIGIT_BITS_USIZE + 1`
        // Written this way to avoid `position + 1` overflowing.
        let min_len =
            position / DIGIT_BITS_USIZE + 1 + (position % DIGIT_BITS_USIZE + 1) / DIGIT_BITS_USIZE;
        let len = self.as_digits().len();
        if len < min_len && value == self.is_negative() {
            // The bit is the sign bit or higher and is not changing, nothing to do.
            return;
        }
        let mut digits = mem::replace(self, IBig::ZERO).into_digits();
        if len < min_len {
            digits.resize(min_len, Digit::ZERO);
            ibig_core::extend_signed(&mut digits, len);
        }
        ibig_core::set_bit(&mut digits, position, value);
        *self = IBig::from_digits(digits);
    }

    /// Returns the number of trailing zero bits of the two's complement representation.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero, which has infinitely many trailing zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0b101000i16).trailing_zeros(), 3);
    /// // -4 is ...11111100, with 2 trailing zeros.
    /// assert_eq!(IBig::from(-4i8).trailing_zeros(), 2);
    /// ```
    pub fn trailing_zeros(&self) -> usize {
        match self.try_to_digit() {
            Some(digit) => {
                assert!(
                    digit != SignedDigit::ZERO,
                    "zero has infinitely many trailing zeros"
                );
                digit.trailing_zeros().try_into().unwrap()
            }
            None => ibig_core::trailing_zeros(self.as_digits()),
        }
    }

    /// Returns the number of trailing one bits of the two's complement representation.
    ///
    /// # Panics
    ///
    /// Panics if `self` is -1 (all ones), which has infinitely many trailing ones.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0b100111i16).trailing_ones(), 3);
    /// // -3 is ...11111101, with 1 trailing one.
    /// assert_eq!(IBig::from(-3i8).trailing_ones(), 1);
    /// ```
    pub fn trailing_ones(&self) -> usize {
        match self.try_to_digit() {
            Some(digit) => {
                assert!(
                    digit != SignedDigit::from_i8(-1),
                    "-1 has infinitely many trailing ones"
                );
                digit.trailing_ones().try_into().unwrap()
            }
            None => ibig_core::trailing_ones(self.as_digits()),
        }
    }

    /// Returns `true` if the value is a power of two.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert!(IBig::from(8i8).is_power_of_two());
    /// assert!(!IBig::from(6i8).is_power_of_two());
    /// assert!(!IBig::from(-8i8).is_power_of_two());
    /// assert!(!IBig::ZERO.is_power_of_two());
    /// ```
    pub fn is_power_of_two(&self) -> bool {
        match self.try_to_digit() {
            // A negative digit is never a power of two, even though its bit pattern may be
            // (the most-negative value is a single high bit).
            Some(digit) => !digit.is_negative() && digit.cast_unsigned().is_power_of_two(),
            None => {
                let digits = self.as_digits();
                !ibig_core::is_negative(digits) && ibig_core::is_power_of_two(digits)
            }
        }
    }

    /// Returns the smallest power of two greater than or equal to the value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(5i8).next_power_of_two(), IBig::from(8i8));
    /// assert_eq!(IBig::from(16i8).next_power_of_two(), IBig::from(16i8));
    /// assert_eq!(IBig::from(-5i8).next_power_of_two(), IBig::from(1i8));
    /// assert_eq!(IBig::ZERO.next_power_of_two(), IBig::from(1i8));
    /// ```
    pub fn next_power_of_two(&self) -> IBig {
        self.clone().into_next_power_of_two()
    }

    /// Consumes the value and returns the smallest power of two greater than or equal to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(5i8).into_next_power_of_two(), IBig::from(8i8));
    /// assert_eq!(IBig::from(16i8).into_next_power_of_two(), IBig::from(16i8));
    /// assert_eq!(IBig::from(-5i8).into_next_power_of_two(), IBig::from(1i8));
    /// assert_eq!(IBig::ZERO.into_next_power_of_two(), IBig::from(1i8));
    /// ```
    pub fn into_next_power_of_two(self) -> IBig {
        match UBig::try_from(self) {
            Ok(value) => IBig::from(value.into_next_power_of_two()),
            // A negative value.
            Err(_) => IBig::from(1i8),
        }
    }
}
