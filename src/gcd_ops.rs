//! Operators for finding greatest common divisor.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    helper_macros,
    ibig::IBig,
    memory::MemoryAllocation,
    ops::{Gcd, ExtendedGcd},
    gcd,
    div,
    primitive::{extend_word, PrimitiveSigned, PrimitiveUnsigned},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::mem;
use static_assertions::const_assert;

impl Gcd<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn gcd(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::gcd_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::gcd_large_word(&buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::gcd_large_word(&buffer0, word1),
            (Large(buffer0), Large(buffer1)) => UBig::gcd_large(&buffer0, &buffer1),
        }
    }
}

impl Gcd<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn gcd(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::gcd_word(word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::gcd_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::gcd_large_word(&buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => UBig::gcd_large(&buffer0, buffer1),
        }
    }
}

impl Gcd<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn gcd(self, rhs: UBig) -> UBig {
        rhs.gcd(self)
    }
}

impl Gcd<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn gcd(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::gcd_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::gcd_large_word(buffer1, *word0),
            (Large(buffer0), Small(word1)) => UBig::gcd_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => UBig::gcd_large(buffer0, buffer1),
        }
    }
}

impl UBig {
    /// Perform gcd on two `Word`s.
    #[inline]
    fn gcd_word(a: Word, b: Word) -> UBig {
        if a == 0 || b == 0 {
            return UBig::from_word(a | b);
        }

        UBig::from_word(gcd::gcd_word_by_word(a, b))
    }

    /// Perform gcd on a large number with a `Word`.
    #[inline]
    fn gcd_large_word(buffer: &Buffer, rhs: Word) -> UBig {
        if rhs == 0 {
            return buffer.clone().into();
        }

        // reduce the large number
        let small = div::rem_by_word(buffer, rhs);
        if small == 0 {
            return UBig::from_word(rhs);
        }

        UBig::from_word(gcd::gcd_word_by_word(small, rhs))
    }

    /// Multiply two large numbers.
    fn gcd_large(lhs: &[Word], rhs: &[Word]) -> UBig {
        unimplemented!()
    }
}
