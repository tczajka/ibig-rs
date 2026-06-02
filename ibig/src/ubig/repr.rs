//! Contains the definition of [`UBig`] and its internal representation.

use crate::{Digits, INLINE_DIGITS};
use ibig_core::{Digit, min_len};

/// Unsigned big integer.
///
/// An arbitrarily large unsigned integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UBig(
    /// The little-endian digits in canonical form: the buffer is never empty, and its
    /// most-significant digit is nonzero except for the value zero, which is the single
    /// digit `[0]`. So every value has exactly one representation and the derived
    /// `Eq`/`Hash` are correct. Every value of at most [`INLINE_DIGITS`] digits — in
    /// particular every single-digit value — is stored inline, and heap buffers are not
    /// heavily over-allocated.
    Digits,
);

#[allow(dead_code)] // Used by arithmetic algorithms added in later commits.
impl UBig {
    /// Construct from a single digit.
    #[inline]
    pub(crate) fn from_digit(digit: Digit) -> UBig {
        let mut digits = Digits::new();
        digits.push(digit);
        UBig(digits)
    }

    /// Construct from little-endian digits.
    pub(crate) fn from_digits(mut digits: Digits) -> UBig {
        if digits.is_empty() {
            digits.push(Digit::ZERO);
        }
        digits.truncate(min_len(&digits));
        if digits.spilled()
            && (digits.len() <= INLINE_DIGITS || digits.len() < digits.capacity() / 4)
        {
            digits.shrink_to_fit();
        }
        UBig(digits)
    }

    /// The value as a single digit, if it fits in one.
    #[inline]
    pub(crate) fn try_to_digit(&self) -> Option<Digit> {
        if !self.0.spilled() && self.0.len() == 1 {
            Some(self.0[0])
        } else {
            None
        }
    }

    /// The little-endian digits, by reference.
    #[inline]
    pub(crate) fn as_digits(&self) -> &[Digit] {
        &self.0
    }

    /// Consume into the little-endian digits.
    #[inline]
    pub(crate) fn into_digits(self) -> Digits {
        self.0
    }
}
