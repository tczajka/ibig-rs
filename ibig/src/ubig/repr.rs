//! Contains the definition of [`UBig`] and its internal representation.

use self::Repr::*;
use alloc::vec::Vec;
use ibig_core::Digit;

/// Unsigned big integer.
///
/// An arbitrarily large unsigned integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UBig(
    /// The internal representation in canonical form.
    Repr,
);

#[allow(dead_code)] // Used by arithmetic algorithms added in later commits.
impl UBig {
    /// Construct from a single digit.
    #[inline]
    pub(crate) fn from_digit(digit: Digit) -> UBig {
        UBig(Small(digit))
    }

    /// Construct from little-endian digits, normalizing to the canonical representation.
    ///
    /// Trailing (most-significant) zero digits are removed; a value that fits in a single
    /// digit is stored as [`Repr::Small`]. Excess capacity is released when the buffer is
    /// heavily over-allocated.
    pub(crate) fn from_digits(mut digits: Vec<Digit>) -> UBig {
        while let Some(&Digit::ZERO) = digits.last() {
            digits.pop();
        }
        match digits.len() {
            0 => UBig::from_digit(Digit::ZERO),
            1 => UBig::from_digit(digits[0]),
            _ => {
                if digits.len() < digits.capacity() / 4 {
                    digits.shrink_to_fit();
                }
                UBig(Large(digits))
            }
        }
    }

    /// The internal representation, by reference.
    #[inline]
    pub(crate) fn repr(&self) -> &Repr {
        &self.0
    }

    /// Consume into the internal representation.
    #[inline]
    pub(crate) fn into_repr(self) -> Repr {
        self.0
    }
}

/// Internal representation of [`UBig`].
#[allow(dead_code)] // Constructed by arithmetic algorithms added in later commits.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Repr {
    /// A number that fits in a single [`Digit`].
    Small(Digit),
    /// A number that does not fit in a single [`Digit`].
    ///
    /// The digits are stored little-endian.
    ///
    /// In a canonical representation, this has:
    /// * length at least 2,
    /// * no leading zero (the most significant digit is non-zero),
    /// * capacity no more than ~4x length (assuming no such over-allocation inside `Vec`).
    Large(Vec<Digit>),
}
