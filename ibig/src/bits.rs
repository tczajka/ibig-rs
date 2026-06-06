//! Bit operations on [`UBig`] and [`IBig`].

use crate::repr::{AsDigits, DIGIT_BITS_USIZE};
use crate::{IBig, UBig};
use core::mem;
use ibig_core::{Digit, SignedDigit};

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
        self.checked_ilog2()
            .expect("argument of ilog2 must be positive")
    }

    /// Returns the base-2 logarithm, rounded down, or `None` if the value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0b101u8).checked_ilog2(), Some(2));
    /// assert_eq!(UBig::ZERO.checked_ilog2(), None);
    /// ```
    #[inline]
    pub fn checked_ilog2(&self) -> Option<usize> {
        self.bit_width().checked_sub(1)
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
        let mut digits = mem::take(self).into_digits();
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

    /// Returns the number of trailing zero bits.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero, which has infinitely many trailing zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(1u8).trailing_zeros(), 0);
    /// assert_eq!(UBig::from(0b101000u8).trailing_zeros(), 3);
    /// ```
    pub fn trailing_zeros(&self) -> usize {
        match self.try_to_digit() {
            Some(digit) => {
                assert!(
                    digit != Digit::ZERO,
                    "zero has infinitely many trailing zeros"
                );
                digit.trailing_zeros().try_into().unwrap()
            }
            None => ibig_core::trailing_zeros(self.as_digits()),
        }
    }

    /// Returns the number of trailing one bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0u8).trailing_ones(), 0);
    /// assert_eq!(UBig::from(0b100111u8).trailing_ones(), 3);
    /// ```
    pub fn trailing_ones(&self) -> usize {
        match self.try_to_digit() {
            Some(digit) => digit.trailing_ones().try_into().unwrap(),
            None => ibig_core::trailing_ones(self.as_digits()),
        }
    }

    /// Returns `true` if the value is a power of two (exactly one bit set).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert!(UBig::from(8u8).is_power_of_two());
    /// assert!(!UBig::from(6u8).is_power_of_two());
    /// assert!(!UBig::ZERO.is_power_of_two());
    /// ```
    pub fn is_power_of_two(&self) -> bool {
        match self.try_to_digit() {
            Some(digit) => digit.is_power_of_two(),
            None => ibig_core::is_power_of_two(self.as_digits()),
        }
    }

    /// Returns the smallest power of two greater than or equal to the value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(5u8).next_power_of_two(), UBig::from(8u8));
    /// assert_eq!(UBig::from(8u8).next_power_of_two(), UBig::from(8u8));
    /// assert_eq!(UBig::ZERO.next_power_of_two(), UBig::from(1u8));
    /// ```
    pub fn next_power_of_two(&self) -> UBig {
        // Fast path: a single digit.
        if let Some(digit) = self.try_to_digit() {
            return match digit.checked_next_power_of_two() {
                Some(power) => UBig::from_digit(power),
                None => UBig::const_from_digits(&[Digit::ZERO, Digit::from_u8(1)]),
            };
        }

        // Slow path: multiple digits.
        let mut digits = self.clone().into_digits();
        if ibig_core::next_power_of_two_in_place(&mut digits) {
            // Overflow.
            digits.push(Digit::from_u8(1));
        }
        UBig::from_digits(digits)
    }
}

impl IBig {
    /// Returns the number of bits needed to represent the value: the position of the
    /// most-significant set bit plus one, or 0 for the value zero.
    ///
    /// # Panics
    ///
    /// Panics if the value is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::ZERO.bit_width(), 0);
    /// assert_eq!(IBig::from(0b101i8).bit_width(), 3);
    /// ```
    #[inline]
    pub fn bit_width(&self) -> usize {
        self.checked_bit_width()
            .expect("bit_width is not defined for negative values")
    }

    /// Returns the number of bits needed to represent the value (the position of the
    /// most-significant set bit plus one, or 0 for zero), or `None` if the value is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0b101i8).checked_bit_width(), Some(3));
    /// assert_eq!(IBig::ZERO.checked_bit_width(), Some(0));
    /// assert_eq!(IBig::from(-1i8).checked_bit_width(), None);
    /// ```
    #[inline]
    pub fn checked_bit_width(&self) -> Option<usize> {
        match self.try_to_digit() {
            Some(digit) => {
                if digit.is_negative() {
                    None
                } else {
                    Some(
                        DIGIT_BITS_USIZE
                            - usize::try_from(digit.cast_unsigned().leading_zeros()).unwrap(),
                    )
                }
            }
            None => {
                let digits = self.as_digits();
                if ibig_core::is_negative(digits) {
                    None
                } else {
                    Some(ibig_core::bit_width(digits))
                }
            }
        }
    }

    /// Returns the base-2 logarithm, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if the value is not positive (zero or negative).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(1i8).ilog2(), 0);
    /// assert_eq!(IBig::from(0b101i8).ilog2(), 2);
    /// ```
    #[inline]
    pub fn ilog2(&self) -> usize {
        self.checked_ilog2()
            .expect("argument of ilog2 must be positive")
    }

    /// Returns the base-2 logarithm, rounded down, or `None` if the value is not positive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::IBig;
    /// assert_eq!(IBig::from(0b101i8).checked_ilog2(), Some(2));
    /// assert_eq!(IBig::ZERO.checked_ilog2(), None);
    /// assert_eq!(IBig::from(-4i8).checked_ilog2(), None);
    /// ```
    #[inline]
    pub fn checked_ilog2(&self) -> Option<usize> {
        self.checked_bit_width()?.checked_sub(1)
    }

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
        let mut digits = mem::take(self).into_digits();
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
}
