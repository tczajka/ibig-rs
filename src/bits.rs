use crate::{
    ibig::IBig,
    primitive::{Word, WORD_BITS},
    ubig::{Repr::*, UBig},
};

impl UBig {
    /// Returns true if the `n`-th bit is set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(0b10010).bit(1), true);
    /// assert_eq!(ubig!(0b10010).bit(3), false);
    /// assert_eq!(ubig!(0b10010).bit(100), false);
    /// ```
    #[inline]
    pub fn bit(&self, n: usize) -> bool {
        match self.repr() {
            Small(word) => n < WORD_BITS as usize && word & (1 as Word) << n != 0,
            Large(buffer) => {
                let idx = n / WORD_BITS as usize;
                idx < buffer.len() && buffer[idx] & (1 as Word) << (n % WORD_BITS as usize) != 0
            }
        }
    }

    /// Returns the number of trailing zeros in the binary representation.
    ///
    /// In other words, it is the smallest `n` such that 2 to the power of `n` divides the number.
    ///
    /// For 0, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(17).trailing_zeros(), Some(0));
    /// assert_eq!(ubig!(48).trailing_zeros(), Some(4));
    /// assert_eq!(ubig!(0b101000000).trailing_zeros(), Some(6));
    /// assert_eq!(ubig!(0).trailing_zeros(), None);
    /// ```
    #[inline]
    pub fn trailing_zeros(&self) -> Option<usize> {
        match self.repr() {
            Small(0) => None,
            Small(word) => Some(word.trailing_zeros() as usize),
            Large(buffer) => Some(trailing_zeros_large(buffer)),
        }
    }

    /// Integer logarithm base 2.
    ///
    /// Returns the floor of the logarithm base 2 of the number.
    /// In other words, it is the position of the highest 1 bit in the binary representation.
    ///
    /// For 0, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(17).ilog2(), Some(4));
    /// assert_eq!(ubig!(0b101000000).ilog2(), Some(8));
    /// assert_eq!(ubig!(0).ilog2(), None);
    /// ```
    #[inline]
    pub fn ilog2(&self) -> Option<usize> {
        match self.repr() {
            Small(0) => None,
            Small(word) => Some((WORD_BITS - 1 - word.leading_zeros()) as usize),
            Large(buffer) => Some(
                buffer.len() * WORD_BITS as usize
                    - 1
                    - buffer.last().unwrap().leading_zeros() as usize,
            ),
        }
    }
}

fn trailing_zeros_large(words: &[Word]) -> usize {
    debug_assert!(*words.last().unwrap() != 0);

    for (idx, word) in words.iter().enumerate() {
        if *word != 0 {
            return idx * WORD_BITS as usize + word.trailing_zeros() as usize;
        }
    }
    panic!("trailing_zeros_large(0)")
}

impl IBig {
    /// Returns the number of trailing zeros in the two's complement binary representation.
    ///
    /// In other words, it is the smallest `n` such that 2 to the power of `n` divides the number.
    ///
    /// For 0, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(17).trailing_zeros(), Some(0));
    /// assert_eq!(ibig!(-48).trailing_zeros(), Some(4));
    /// assert_eq!(ibig!(-0b101000000).trailing_zeros(), Some(6));
    /// assert_eq!(ibig!(0).trailing_zeros(), None);
    /// ```
    #[inline]
    pub fn trailing_zeros(&self) -> Option<usize> {
        self.magnitude().trailing_zeros()
    }
}
