//! Contains the definition of [`UBig`] and its internal representation.

use crate::{Digits, INLINE_DIGITS, MAX_DIGITS};
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

impl UBig {
    /// Construct from a single digit.
    #[inline]
    pub(crate) const fn from_digit(digit: Digit) -> UBig {
        const { assert!(INLINE_DIGITS >= 1) };
        let mut digits = [Digit::ZERO; INLINE_DIGITS];
        digits[0] = digit;
        // A single digit is always canonical.
        // SAFETY: `1 <= INLINE_DIGITS`.
        UBig(unsafe { Digits::from_const_with_len_unchecked(digits, 1) })
    }

    /// Construct from at most `INLINE_DIGITS` little-endian digits.
    ///
    /// # Panics
    ///
    /// Panics if `digits.len() > INLINE_DIGITS`.
    #[inline]
    pub(crate) const fn const_from_digits(digits: &[Digit]) -> UBig {
        assert!(digits.len() <= INLINE_DIGITS);

        let mut buffer = [Digit::ZERO; INLINE_DIGITS];
        let mut len = min_len(digits);
        let (dest, _) = buffer.split_at_mut(len);
        let (src, _) = digits.split_at(len);
        dest.copy_from_slice(src);

        // `UBig` always keeps at least one digit.
        if len == 0 {
            len = 1;
        }
        // SAFETY: `len <= INLINE_DIGITS`.
        UBig(unsafe { Digits::from_const_with_len_unchecked(buffer, len) })
    }

    /// Construct from little-endian digits.
    ///
    /// # Panics
    ///
    /// Panics if, after normalization, the value has more than [`MAX_DIGITS`] digits.
    pub(crate) fn from_digits(mut digits: Digits) -> UBig {
        digits.truncate(min_len(&digits));
        // `min_len` returns 0 for zero, but `UBig` always keeps at least one digit.
        if digits.is_empty() {
            digits.push(Digit::ZERO);
        }
        if digits.spilled() {
            let len = digits.len();
            assert!(len <= MAX_DIGITS, "number too large");
            if len <= INLINE_DIGITS || len < digits.capacity() / 4 {
                digits.shrink_to_fit();
            }
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
