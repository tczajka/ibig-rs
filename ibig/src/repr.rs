//! Contains the definitions of [`UBig`] and [`IBig`] and maintains their invariants.

use core::hint::assert_unchecked;
use ibig_core::{DIGIT_BITS_USIZE, Digit, SignedDigit, min_len, min_len_signed};
use smallvec::SmallVec;

/// Number of [`Digit`]s stored inline before the representation spills to the heap.
pub(crate) const INLINE_DIGITS: usize = 4;

/// Maximum number of [`Digit`]s in a value, chosen so that the total bit length
/// (`MAX_DIGITS * Digit::BITS`) still fits in a `usize`.
const MAX_DIGITS: usize = usize::MAX / DIGIT_BITS_USIZE;

/// Storage for little-endian digits.
///
/// Values of at most [`INLINE_DIGITS`] digits are stored inline; larger values spill to a
/// heap allocation.
pub(crate) type Digits = SmallVec<[Digit; INLINE_DIGITS]>;

/// Unsigned big integer.
///
/// An arbitrarily large unsigned integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UBig(
    /// The little-endian digits in canonical form:
    /// * the buffer is never empty
    /// * most-significant digit is nonzero except for the value zero
    /// * heap buffer is at least 25% used
    /// * a single digit is always stored inline
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
            if len <= digits.capacity() / 4 || len == 1 {
                digits.shrink_to_fit();
            }
        }
        UBig(digits)
    }
}

/// Signed big integer.
///
/// An arbitrarily large signed integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IBig(
    /// The little-endian digits of the two's complement representation in canonical form:
    /// * the buffer is never empty
    /// * most-significant digit is not a redundant sign-extension of the digit below it
    /// * heap buffer is at least 25% used
    /// * a single digit is always stored inline
    Digits,
);

impl IBig {
    /// Construct from a single signed digit.
    #[inline]
    pub(crate) const fn from_digit(digit: SignedDigit) -> IBig {
        let mut buffer = [Digit::ZERO; INLINE_DIGITS];
        buffer[0] = digit.cast_unsigned();
        // A single signed digit is always canonical.
        // SAFETY: `1 <= INLINE_DIGITS`.
        IBig(unsafe { Digits::from_const_with_len_unchecked(buffer, 1) })
    }

    /// Construct from at most `INLINE_DIGITS` little-endian two's complement digits.
    ///
    /// # Panics
    ///
    /// Panics if `digits` is empty or longer than `INLINE_DIGITS`.
    #[inline]
    pub(crate) const fn const_from_digits(digits: &[Digit]) -> IBig {
        assert!(!digits.is_empty() && digits.len() <= INLINE_DIGITS);
        let mut buffer = [Digit::ZERO; INLINE_DIGITS];
        // `min_len_signed` is always at least 1, so the buffer keeps a sign-carrying digit.
        let len = min_len_signed(digits);
        let (dest, _) = buffer.split_at_mut(len);
        let (src, _) = digits.split_at(len);
        dest.copy_from_slice(src);
        // SAFETY: `1 <= len <= INLINE_DIGITS`.
        IBig(unsafe { Digits::from_const_with_len_unchecked(buffer, len) })
    }

    /// Construct from the little-endian digits of a two's complement representation.
    ///
    /// # Panics
    ///
    /// Panics if `digits` is empty, or if, after normalization, the value has more than
    /// [`MAX_DIGITS`] digits.
    pub(crate) fn from_digits(mut digits: Digits) -> IBig {
        digits.truncate(min_len_signed(&digits));
        if digits.spilled() {
            let len = digits.len();
            assert!(len <= MAX_DIGITS, "number too large");
            if len <= digits.capacity() / 4 || len == 1 {
                digits.shrink_to_fit();
            }
        }
        IBig(digits)
    }
}

/// Result of `AsDigits::as_digits` and `AsDigits::into_digits`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum AsDigitsResult<S, L> {
    /// The value fits in one digit.
    Small(S),
    /// The value doesn't fit in one digit.
    Large(L),
}

/// Access to the digit representation.
pub(crate) trait AsDigits: Default {
    /// The single-digit type.
    type SingleDigit;

    /// The little-endian digits, by reference.
    fn as_digits(&self) -> AsDigitsResult<Self::SingleDigit, &[Digit]>;

    /// Consume into the little-endian digits.
    fn into_digits(self) -> AsDigitsResult<Self::SingleDigit, Digits>;
}

impl AsDigits for UBig {
    type SingleDigit = Digit;

    #[inline]
    fn as_digits(&self) -> AsDigitsResult<Digit, &[Digit]> {
        if !self.0.spilled() && self.0.len() == 1 {
            AsDigitsResult::Small(self.0[0])
        } else {
            let res = self.0.as_slice();
            // SAFETY: We never have 0 digits and 1 digit is always inline.
            unsafe { assert_unchecked(res.len() > 1) };
            AsDigitsResult::Large(res)
        }
    }

    #[inline]
    fn into_digits(self) -> AsDigitsResult<Digit, Digits> {
        if !self.0.spilled() && self.0.len() == 1 {
            AsDigitsResult::Small(self.0[0])
        } else {
            // SAFETY: We never have 0 digits and 1 digit is always inline.
            unsafe { assert_unchecked(self.0.len() > 1) };
            AsDigitsResult::Large(self.0)
        }
    }
}

impl AsDigits for IBig {
    type SingleDigit = SignedDigit;

    #[inline]
    fn as_digits(&self) -> AsDigitsResult<SignedDigit, &[Digit]> {
        if !self.0.spilled() && self.0.len() == 1 {
            AsDigitsResult::Small(self.0[0].cast_signed())
        } else {
            let res = self.0.as_slice();
            // SAFETY: We never have 0 digits and 1 digit is always inline.
            unsafe { assert_unchecked(res.len() > 1) };
            AsDigitsResult::Large(res)
        }
    }

    #[inline]
    fn into_digits(self) -> AsDigitsResult<SignedDigit, Digits> {
        if !self.0.spilled() && self.0.len() == 1 {
            AsDigitsResult::Small(self.0[0].cast_signed())
        } else {
            // SAFETY: We never have 0 digits and 1 digit is always inline.
            unsafe { assert_unchecked(self.0.len() > 1) };
            AsDigitsResult::Large(self.0)
        }
    }
}
