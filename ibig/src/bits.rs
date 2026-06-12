//! Bit operations on [`UBig`] and [`IBig`].

use crate::repr::{
    AsDigits,
    AsDigitsResult::{Large, Small},
    Digits,
};
use crate::{IBig, UBig};
use core::mem;
use ibig_core::{BitIndex, DIGIT_BITS_USIZE, Digit, SignedDigit};
use smallvec::smallvec;

impl UBig {
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
    pub fn bit(&self, index: usize) -> bool {
        match self.as_digits() {
            Small(digit) => {
                index < DIGIT_BITS_USIZE && (digit >> index) & Digit::from_u8(1) != Digit::ZERO
            }
            Large(digits) => ibig_core::bit(digits, BitIndex::from(index)),
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
    pub fn set_bit(&mut self, index: usize, value: bool) {
        let index = BitIndex::from(index);

        let mut digits = match mem::take(self).into_digits() {
            Small(digit) => {
                if index.digit_index() == 0 {
                    let mask = Digit::from_u8(1) << index.bit_index();
                    let new_digit = if value { digit | mask } else { digit & !mask };
                    *self = UBig::from_digit(new_digit);
                    return;
                } else if !value {
                    *self = UBig::from_digit(digit);
                    return;
                } else {
                    smallvec![digit]
                }
            }
            Large(digits) => digits,
        };

        if index.digit_index() >= digits.len() {
            if value {
                digits.resize(index.digit_index() + 1, Digit::ZERO);
                ibig_core::set_bit(&mut digits, index, true);
            }
        } else {
            ibig_core::set_bit(&mut digits, index, value);
        }
        *self = UBig::from_digits(digits);
    }

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
        match self.as_digits() {
            Small(digit) => DIGIT_BITS_USIZE - usize::try_from(digit.leading_zeros()).unwrap(),
            Large(digits) => {
                // A multi-digit value is nonzero, so it has a highest set bit.
                let highest = ibig_core::highest_one(digits).unwrap();
                // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                usize::try_from(highest).unwrap() + 1
            }
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
        match self.as_digits() {
            Small(digit) => {
                assert!(
                    digit != Digit::ZERO,
                    "zero has infinitely many trailing zeros"
                );
                digit.trailing_zeros().try_into().unwrap()
            }
            Large(digits) => {
                // A multi-digit value is nonzero, so it has a lowest set bit.
                let lowest = ibig_core::lowest_one(digits).unwrap();
                // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                lowest.try_into().unwrap()
            }
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
        match self.as_digits() {
            Small(digit) => digit.trailing_ones().try_into().unwrap(),
            Large(digits) => {
                // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                match ibig_core::lowest_zero(digits) {
                    Some(bit_index) => bit_index.try_into().unwrap(),
                    None => digits.len() * DIGIT_BITS_USIZE,
                }
            }
        }
    }

    /// Returns the number of one bits (the population count).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::ZERO.count_ones(), 0);
    /// assert_eq!(UBig::from(0b10110u8).count_ones(), 3);
    /// ```
    #[inline]
    pub fn count_ones(&self) -> usize {
        match self.as_digits() {
            Small(digit) => digit.count_ones().try_into().unwrap(),
            Large(digits) => ibig_core::count_ones(digits),
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
    #[inline]
    pub fn is_power_of_two(&self) -> bool {
        match self.as_digits() {
            Small(digit) => digit.is_power_of_two(),
            Large(digits) => ibig_core::is_power_of_two(digits),
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
        match self.as_digits() {
            // Fast path: a single digit.
            Small(digit) => match digit.checked_next_power_of_two() {
                Some(power) => UBig::from_digit(power),
                None => UBig::const_from_digits(&[Digit::ZERO, Digit::from_u8(1)]),
            },
            // Slow path: multiple digits.
            Large(slice) => {
                // Clone with room for one more digit in case rounding up overflows.
                let mut digits = Digits::with_capacity(slice.len() + 1);
                digits.extend_from_slice(slice);
                if ibig_core::next_power_of_two(&mut digits) {
                    // Overflow.
                    digits.push(Digit::from_u8(1));
                }
                UBig::from_digits(digits)
            }
        }
    }
}

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
    pub fn bit(&self, index: usize) -> bool {
        match self.as_digits() {
            Small(digit) => {
                if index < DIGIT_BITS_USIZE {
                    (digit >> index) & SignedDigit::from_i8(1) != SignedDigit::ZERO
                } else {
                    // Positions above the digit read the sign bit.
                    digit.is_negative()
                }
            }
            Large(digits) => ibig_core::bit_signed(digits, BitIndex::from(index)),
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
    pub fn set_bit(&mut self, index: usize, value: bool) {
        let mut digits = match mem::take(self).into_digits() {
            Small(digit) => {
                if index < DIGIT_BITS_USIZE - 1 {
                    let mask = SignedDigit::from_i8(1) << index;
                    let new = if value { digit | mask } else { digit & !mask };
                    *self = IBig::from_digit(new);
                    return;
                } else if value == digit.is_negative() {
                    *self = IBig::from_digit(digit);
                    return;
                } else {
                    smallvec![digit.cast_unsigned()]
                }
            }
            Large(digits) => digits,
        };

        // Number of digits needed for the modified bit to sit strictly below the sign bit,
        // i.e. the smallest `min_len` with `index < min_len * DIGIT_BITS_USIZE - 1`. This is
        // `digit_index + 1`, plus one more digit when the bit is the top bit of its digit (so it
        // would otherwise land on the sign bit). Avoids `index + 1` overflowing.
        let index = BitIndex::from(index);
        let min_len = index.digit_index()
            + 1
            + (usize::try_from(index.bit_index()).unwrap() + 1) / DIGIT_BITS_USIZE;

        let len = digits.len();
        if len < min_len {
            if value == ibig_core::is_negative(&digits) {
                *self = IBig::from_digits(digits);
                return;
            }
            digits.resize(min_len, Digit::ZERO);
            ibig_core::extend_signed(&mut digits, len);
        }
        ibig_core::set_bit(&mut digits, index, value);
        *self = IBig::from_digits(digits);
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
    pub fn checked_ilog2(&self) -> Option<usize> {
        match self.as_digits() {
            Small(digit) => digit.checked_ilog2().map(|x| x.try_into().unwrap()),
            Large(digits) => {
                if ibig_core::is_negative(digits) {
                    None
                } else {
                    // A multi-digit non-negative value is nonzero, so it has a highest set bit.
                    let highest = ibig_core::highest_one(digits).unwrap();
                    // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                    Some(highest.try_into().unwrap())
                }
            }
        }
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
        match self.as_digits() {
            Small(digit) => {
                assert!(
                    digit != SignedDigit::ZERO,
                    "zero has infinitely many trailing zeros"
                );
                digit.trailing_zeros().try_into().unwrap()
            }
            Large(digits) => {
                // A multi-digit value is nonzero, so it has a lowest set bit.
                let lowest = ibig_core::lowest_one(digits).unwrap();
                // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                lowest.try_into().unwrap()
            }
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
        match self.as_digits() {
            Small(digit) => {
                assert!(
                    digit != SignedDigit::from_i8(-1),
                    "-1 has infinitely many trailing ones"
                );
                digit.trailing_ones().try_into().unwrap()
            }
            Large(digits) => {
                // A multi-digit two's complement value is never all ones, so it has a lowest zero.
                let lowest = ibig_core::lowest_zero(digits).unwrap();
                // This will not overflow because our numbers are never longer than `usize::MAX` bits.
                lowest.try_into().unwrap()
            }
        }
    }
}
