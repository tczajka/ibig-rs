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
        self.clone().into_next_power_of_two()
    }

    /// Consumes the value and returns the smallest power of two greater than or equal to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(5u8).into_next_power_of_two(), UBig::from(8u8));
    /// assert_eq!(UBig::from(8u8).into_next_power_of_two(), UBig::from(8u8));
    /// assert_eq!(UBig::ZERO.into_next_power_of_two(), UBig::from(1u8));
    /// ```
    pub fn into_next_power_of_two(self) -> UBig {
        // Fast path: a single digit.
        if let Some(digit) = self.try_to_digit() {
            return match digit.checked_next_power_of_two() {
                Some(power) => UBig::from_digit(power),
                None => UBig::const_from_digits(&[Digit::ZERO, Digit::from_u8(1)]),
            };
        }

        // Slow path: multiple digits.
        let mut digits = self.into_digits();
        if ibig_core::next_power_of_two_in_place(&mut digits) {
            // Overflow.
            digits.push(Digit::from_u8(1));
        }
        UBig::from_digits(digits)
    }
}
