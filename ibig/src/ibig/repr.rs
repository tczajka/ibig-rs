//! Contains the definition of [`IBig`] and its internal representation.

use crate::{Digits, INLINE_DIGITS};
use ibig_core::{Digit, SignedDigit, min_len_signed};

/// Signed big integer.
///
/// An arbitrarily large signed integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IBig(
    /// The little-endian digits of the two's complement representation in canonical form:
    /// the buffer is never empty, and its most-significant digit is not a redundant
    /// sign-extension of the digit below it (so the representation uses the fewest possible
    /// digits, with the sign carried by the top digit's most-significant bit). The value
    /// zero is the single digit `[0]`. So every value has exactly one representation and
    /// the derived `Eq`/`Hash` are correct. Every value of at most [`INLINE_DIGITS`]
    /// digits — in particular every single-digit value — is stored inline, and heap buffers
    /// are not heavily over-allocated.
    Digits,
);

#[allow(dead_code)] // Used by arithmetic algorithms added in later commits.
impl IBig {
    /// Construct from a single signed digit.
    #[inline]
    pub(crate) fn from_digit(digit: SignedDigit) -> IBig {
        let mut digits = Digits::new();
        digits.push(digit.cast_unsigned());
        IBig(digits)
    }

    /// Construct from the little-endian digits of a two's complement representation.
    ///
    /// # Panics
    ///
    /// Panics if `digits` is empty.
    pub(crate) fn from_digits(mut digits: Digits) -> IBig {
        assert!(!digits.is_empty());
        digits.truncate(min_len_signed(&digits));
        if digits.spilled()
            && (digits.len() <= INLINE_DIGITS || digits.len() < digits.capacity() / 4)
        {
            digits.shrink_to_fit();
        }
        IBig(digits)
    }

    /// The value as a single signed digit, if it fits in one.
    #[inline]
    pub(crate) fn try_to_digit(&self) -> Option<SignedDigit> {
        if !self.0.spilled() && self.0.len() == 1 {
            Some(self.0[0].cast_signed())
        } else {
            None
        }
    }

    /// The little-endian digits of the two's complement representation, by reference.
    #[inline]
    pub(crate) fn as_digits(&self) -> &[Digit] {
        &self.0
    }

    /// Consume into the little-endian digits of the two's complement representation.
    #[inline]
    pub(crate) fn into_digits(self) -> Digits {
        self.0
    }
}
