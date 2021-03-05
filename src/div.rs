use crate::{
    add,
    buffer::Buffer,
    cmp,
    ibig::IBig,
    mul,
    primitive::{
        double_word, extend_word, split_double_word, DoubleWord, SignedWord, Word, WORD_BITS,
    },
    shift,
    sign::Abs,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    cmp::Ordering,
    iter, mem,
    ops::{Div, DivAssign, Rem, RemAssign},
};
use static_assertions::const_assert;

/// If divisor or quotient is at most this length, use the simple division algorithm.
const MAX_LEN_SIMPLE: usize = 32;

/// Compute quotient and remainder at the same time.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ubig!(23).div_rem(ubig!(10)), (ubig!(2), ubig!(3)));
/// ```
pub trait DivRem<Rhs = Self> {
    type OutputDiv;
    type OutputRem;

    fn div_rem(self, rhs: Rhs) -> (Self::OutputDiv, Self::OutputRem);
}

/// Compute Euclidean quotient.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ibig!(-23).div_euclid(ibig!(10)), ibig!(-3));
/// ```
pub trait DivEuclid<Rhs = Self> {
    type Output;

    fn div_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Compute Euclidean remainder.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ibig!(-23).rem_euclid(ibig!(10)), ibig!(7));
/// ```
pub trait RemEuclid<Rhs = Self> {
    type Output;

    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Compute Euclidean quotient and remainder at the same time.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ibig!(-23).div_rem_euclid(ibig!(10)), (ibig!(-3), ibig!(7)));
/// ```
pub trait DivRemEuclid<Rhs = Self> {
    type OutputDiv;
    type OutputRem;

    fn div_rem_euclid(self, rhs: Rhs) -> (Self::OutputDiv, Self::OutputRem);
}

impl Div<UBig> for UBig {
    type Output = UBig;

    fn div(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(word0, word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0, buffer1)
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl Div<&UBig> for UBig {
    type Output = UBig;

    fn div(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::div_word(word0, *word1),
                Large(_) => UBig::from_word(0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::div_large_word(buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_large(buffer0, buffer1.clone())
                    } else {
                        UBig::from_word(0)
                    }
                }
            },
        }
    }
}

impl Div<UBig> for &UBig {
    type Output = UBig;

    fn div(self, rhs: UBig) -> UBig {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::div_word(*word0, word1),
                Large(_) => UBig::from_word(0),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::div_large_word(buffer0.clone(), word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_large(buffer0.clone(), buffer1)
                    } else {
                        UBig::from_word(0)
                    }
                }
            },
        }
    }
}

impl Div<&UBig> for &UBig {
    type Output = UBig;

    fn div(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(*word0, *word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0.clone(), buffer1.clone())
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl DivAssign<UBig> for UBig {
    fn div_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&UBig> for UBig {
    fn div_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<UBig> for UBig {
    type Output = UBig;

    fn rem(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(word0, word1),
            (Small(word0), Large(_)) => UBig::from_word(word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(&buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0, buffer1)
                } else {
                    buffer0.into()
                }
            }
        }
    }
}

impl Rem<&UBig> for UBig {
    type Output = UBig;

    fn rem(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::rem_word(word0, *word1),
                Large(_) => UBig::from_word(word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::rem_large_word(&buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::rem_large(buffer0, buffer1.clone())
                    } else {
                        buffer0.into()
                    }
                }
            },
        }
    }
}

impl Rem<UBig> for &UBig {
    type Output = UBig;

    fn rem(self, rhs: UBig) -> UBig {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::rem_word(*word0, word1),
                Large(_) => UBig::from_word(*word0),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::rem_large_word(buffer0, word1),
                Large(mut buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::rem_large(buffer0.clone(), buffer1)
                    } else {
                        // Reuse buffer1 for the remainder.
                        buffer1.resizing_clone_from(buffer0);
                        buffer1.into()
                    }
                }
            },
        }
    }
}

impl Rem<&UBig> for &UBig {
    type Output = UBig;

    fn rem(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(*word0, *word1),
            (Small(word0), Large(_)) => UBig::from_word(*word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl RemAssign<UBig> for UBig {
    fn rem_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&UBig> for UBig {
    fn rem_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(word0, word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0, buffer1)
                } else {
                    (UBig::from_word(0), buffer0.into())
                }
            }
        }
    }
}

impl DivRem<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::div_rem_word(word0, *word1),
                Large(_) => (UBig::from_word(0), UBig::from_word(word0)),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::div_rem_large_word(buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_rem_large(buffer0, buffer1.clone())
                    } else {
                        (UBig::from_word(0), buffer0.into())
                    }
                }
            },
        }
    }
}

impl DivRem<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::div_rem_word(*word0, word1),
                Large(_) => (UBig::from_word(0), UBig::from_word(*word0)),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::div_rem_large_word(buffer0.clone(), word1),
                Large(mut buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_rem_large(buffer0.clone(), buffer1)
                    } else {
                        // Reuse buffer1 for the remainder.
                        buffer1.resizing_clone_from(buffer0);
                        (UBig::from_word(0), buffer1.into())
                    }
                }
            },
        }
    }
}

impl DivRem<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(*word0, *word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(*word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    (UBig::from_word(0), self.clone())
                }
            }
        }
    }
}

impl DivEuclid<UBig> for UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<UBig> for &UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for &UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl RemEuclid<UBig> for UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<UBig> for &UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for &UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl DivRemEuclid<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl Div<IBig> for IBig {
    type Output = IBig;

    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for IBig {
    type Output = IBig;

    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<IBig> for &IBig {
    type Output = IBig;

    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for &IBig {
    type Output = IBig;

    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl DivAssign<IBig> for IBig {
    fn div_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&IBig> for IBig {
    fn div_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<IBig> for IBig {
    type Output = IBig;

    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for IBig {
    type Output = IBig;

    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<IBig> for &IBig {
    type Output = IBig;

    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for &IBig {
    type Output = IBig;

    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl RemAssign<IBig> for IBig {
    fn rem_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&IBig> for IBig {
    fn rem_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivEuclid<IBig> for IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            q - s
        } else {
            q
        }
    }
}

impl DivEuclid<&IBig> for IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            q - rhs.signum()
        } else {
            q
        }
    }
}

impl DivEuclid<IBig> for &IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            q - s
        } else {
            q
        }
    }
}

impl DivEuclid<&IBig> for &IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            q - rhs.signum()
        } else {
            q
        }
    }
}

impl RemEuclid<IBig> for IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        if r.is_negative() {
            r + rhs.abs()
        } else {
            r
        }
    }
}

impl RemEuclid<&IBig> for IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        if r.is_negative() {
            r + rhs.abs()
        } else {
            r
        }
    }
}

impl RemEuclid<IBig> for &IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        if r.is_negative() {
            r + rhs.abs()
        } else {
            r
        }
    }
}

impl RemEuclid<&IBig> for &IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        if r.is_negative() {
            r + rhs.abs()
        } else {
            r
        }
    }
}

impl DivRemEuclid<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        if r.is_negative() {
            (q - rhs.signum(), r + rhs.abs())
        } else {
            (q, r)
        }
    }
}

impl DivRemEuclid<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            (q - rhs.signum(), r + rhs.abs())
        } else {
            (q, r)
        }
    }
}

impl DivRemEuclid<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        if r.is_negative() {
            (q - rhs.signum(), r + rhs.abs())
        } else {
            (q, r)
        }
    }
}

impl DivRemEuclid<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            (q - rhs.signum(), r + rhs.abs())
        } else {
            (q, r)
        }
    }
}

impl UBig {
    /// `lhs / rhs`
    fn div_word(lhs: Word, rhs: Word) -> UBig {
        match lhs.checked_div(rhs) {
            Some(res) => UBig::from_word(res),
            None => panic_divide_by_0(),
        }
    }

    /// `lhs % rhs`
    fn rem_word(lhs: Word, rhs: Word) -> UBig {
        match lhs.checked_rem(rhs) {
            Some(res) => UBig::from_word(res),
            None => panic_divide_by_0(),
        }
    }

    /// (lhs / rhs, lhs % rhs)
    fn div_rem_word(lhs: Word, rhs: Word) -> (UBig, UBig) {
        // If division works, remainder also works.
        match lhs.checked_div(rhs) {
            Some(res) => (UBig::from_word(res), UBig::from_word(lhs % rhs)),
            None => panic_divide_by_0(),
        }
    }

    /// `lhs / rhs`
    fn div_large_word(lhs: Buffer, rhs: Word) -> UBig {
        let (q, _) = UBig::div_rem_large_word(lhs, rhs);
        q
    }

    /// `lhs % rhs`
    fn rem_large_word(lhs: &[Word], rhs: Word) -> UBig {
        if rhs == 0 {
            panic_divide_by_0();
        }
        UBig::from_word(rem_by_word(lhs, rhs))
    }

    /// (buffer / rhs, buffer % rhs)
    fn div_rem_large_word(mut buffer: Buffer, rhs: Word) -> (UBig, UBig) {
        if rhs == 0 {
            panic_divide_by_0();
        }
        let rem = div_by_word_in_place(&mut buffer, rhs);
        (buffer.into(), UBig::from_word(rem))
    }

    /// `lhs / rhs`
    fn div_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let (_shift, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let carry = div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
        if carry {
            lhs.push_may_reallocate(1);
        }
        UBig::shr_large_words(lhs, rhs.len())
    }

    /// `lhs % rhs`
    fn rem_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let (sh, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let _carry = div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
        let n = rhs.len();
        rhs.copy_from_slice(&lhs[..n]);
        shift::shr_in_place(&mut rhs, sh);
        rhs.into()
    }

    /// `(lhs / rhs, lhs % rhs)`
    fn div_rem_large(mut lhs: Buffer, mut rhs: Buffer) -> (UBig, UBig) {
        let (sh, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let carry = div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
        if carry {
            lhs.push_may_reallocate(1);
        }
        let n = rhs.len();
        rhs.copy_from_slice(&lhs[..n]);
        shift::shr_in_place(&mut rhs, sh);
        let q = UBig::shr_large_words(lhs, n);
        (q, rhs.into())
    }

    /// Normalizes large arguments for division by shifting them left:
    /// * lhs as least as long as rhs
    /// * top bit of rhs is 1
    ///
    /// Returns (shift size, Fast21 for top rhs word).
    fn div_normalize(lhs: &mut Buffer, rhs: &mut [Word]) -> (u32, FastDiv21) {
        assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let sh = rhs.last().unwrap().leading_zeros();
        assert!(sh != WORD_BITS);
        let rhs_carry = shift::shl_in_place(rhs, sh);
        assert!(rhs_carry == 0);
        let lhs_carry = shift::shl_in_place(lhs, sh);
        if lhs_carry != 0 {
            lhs.push_may_reallocate(lhs_carry);
        }
        (sh, FastDiv21::by(*rhs.last().unwrap()))
    }
}

fn panic_divide_by_0() -> ! {
    panic!("divide by 0")
}

/// words = words / rhs
///
/// rhs must be non-zero
///
/// Returns words % rhs.
pub(crate) fn div_by_word_in_place(words: &mut [Word], rhs: Word) -> Word {
    debug_assert!(rhs != 0);
    if words.is_empty() {
        return 0;
    }
    if rhs.is_power_of_two() {
        let rem = words[0] & (rhs - 1);
        let sh = rhs.trailing_zeros();
        shift::shr_in_place(words, sh);
        return rem;
    }

    let shift = rhs.leading_zeros();
    let mut rem = shift::shl_in_place(words, shift);
    let fast_div_rhs_top = FastDiv21::by(rhs << shift);

    for word in words.iter_mut().rev() {
        let a = double_word(*word, rem);
        let (q, r) = fast_div_rhs_top.div_rem(a);
        *word = q;
        rem = r;
    }
    rem >> shift
}

/// words % rhs
pub(crate) fn rem_by_word(words: &[Word], rhs: Word) -> Word {
    debug_assert!(rhs != 0);
    if words.is_empty() {
        return 0;
    }
    if rhs.is_power_of_two() {
        return words[0] & (rhs - 1);
    }

    let shift = rhs.leading_zeros();
    let fast_div_rhs_top = FastDiv21::by(rhs << shift);

    if shift == 0 {
        let mut rem: Word = 0;
        for word in words.iter().rev() {
            let a = double_word(*word, rem);
            let (_, r) = fast_div_rhs_top.div_rem(a);
            rem = r;
        }
        rem
    } else {
        let mut rem = 0;
        let mut carry = 0;
        for word in words.iter().rev().chain(iter::once(&0)) {
            let w = carry | word >> (WORD_BITS - shift);
            carry = word << shift;
            let a = double_word(w, rem);
            let (_, r) = fast_div_rhs_top.div_rem(a);
            rem = r;
        }
        rem >> shift
    }
}

/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
fn div_rem_in_place(lhs: &mut [Word], rhs: &[Word], fast_div_rhs_top: FastDiv21) -> bool {
    assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);

    if rhs.len() <= MAX_LEN_SIMPLE || lhs.len() - rhs.len() <= MAX_LEN_SIMPLE {
        div_rem_in_place_simple(lhs, rhs, fast_div_rhs_top)
    } else {
        // We need space for multiplications summing up to rhs.len().
        // One of the factors will be at most floor(rhs.len()/2).
        let mut temp = mul::allocate_temp_mul_buffer(rhs.len() / 2);
        div_rem_in_place_divide_conquer(lhs, rhs, fast_div_rhs_top, &mut temp)
    }
}

/// Division in place using the simple algorithm.
///
/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
fn div_rem_in_place_simple(lhs: &mut [Word], rhs: &[Word], fast_div_rhs_top: FastDiv21) -> bool {
    // The Art of Computer Programming, algorithm 4.3.1D.

    let n = rhs.len();
    assert!(n >= 2);
    let rhs0 = rhs[n - 1];
    let rhs1 = rhs[n - 2];
    debug_assert!(rhs0.leading_zeros() == 0);

    let mut lhs_len = lhs.len();
    assert!(lhs_len >= n);

    let quotient_carry = cmp::cmp_same_len(&lhs[lhs_len - n..], &rhs) >= Ordering::Equal;
    if quotient_carry {
        let overflow = add::sub_same_len_in_place(&mut lhs[lhs_len - n..], &rhs);
        assert!(!overflow);
    }

    while lhs_len > n {
        let lhs0 = lhs[lhs_len - 1];
        let lhs1 = lhs[lhs_len - 2];
        let lhs2 = lhs[lhs_len - 3];
        let lhs01 = double_word(lhs1, lhs0);

        // Approximate the next word of quotient by
        // q = floor([lhs0, lhs1] / rhs0)
        // r = remainder
        // q may be too large, but never too small
        //
        // Then improve the approximation by adding an extra word.
        // q' = floor([lhs0, lhs1, lhs2] / [rhs0, rhs1])
        // Most of the time q' will be exact, but may be 1 too large.
        //
        // q must be decreased if subtracting q * rhs1 from [r, lhs2] overflows,
        // i.e. if q * rhs1 > [r, lhs2].
        //
        // This can happen at most twice, because r will certainly overflow after
        // adding rhs0 twice.
        let mut q = if lhs0 < rhs0 {
            let (mut q, mut r) = fast_div_rhs_top.div_rem(lhs01);
            while extend_word(q) * extend_word(rhs1) > double_word(lhs2, r) {
                q -= 1;
                match r.checked_add(rhs0) {
                    None => break,
                    Some(r2) => r = r2,
                }
            }
            q
        } else {
            // In this case MAX is accurate (r is already overflown).
            Word::MAX
        };

        // Subtract a multiple of rhs.
        let mut borrow =
            mul::sub_mul_word_same_len_in_place(&mut lhs[lhs_len - 1 - n..lhs_len - 1], q, &rhs);

        if borrow > lhs0 {
            // Rare case: q is too large (by 1).
            // Add a correction.
            q -= 1;
            let carry = add::add_same_len_in_place(&mut lhs[lhs_len - 1 - n..lhs_len - 1], &rhs);
            debug_assert!(carry);
            borrow -= 1;
        }
        debug_assert!(borrow == lhs0);
        // lhs0 is now logically zeroed out
        lhs_len -= 1;
        // Store next digit of quotient.
        lhs[lhs_len] = q;
    }
    // Quotient is now in lhs[n..] and remainder in lhs[..n].
    quotient_carry
}

/// Division in place using divide and conquer.
///
/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
fn div_rem_in_place_divide_conquer(
    lhs: &mut [Word],
    rhs: &[Word],
    fast_div_rhs_top: FastDiv21,
    temp: &mut [Word],
) -> bool {
    let mut overflow = false;
    let n = rhs.len();
    let mut m = lhs.len();
    assert!(n > MAX_LEN_SIMPLE && m >= n);
    while m >= 2 * n {
        let o = div_rem_in_place_divide_conquer_same_len(
            &mut lhs[m - 2 * n..m],
            rhs,
            fast_div_rhs_top,
            temp,
        );
        if o {
            assert!(m == lhs.len());
            overflow = true;
        }
        m -= n;
    }
    let o =
        div_rem_in_place_divide_conquer_small_quotient(&mut lhs[..m], rhs, fast_div_rhs_top, temp);
    if o {
        assert!(m == lhs.len());
        overflow = true;
    }
    overflow
}

/// Division in place using divide and conquer.
/// Quotient length = divisor length.
///
/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
fn div_rem_in_place_divide_conquer_same_len(
    lhs: &mut [Word],
    rhs: &[Word],
    fast_div_rhs_top: FastDiv21,
    temp: &mut [Word],
) -> bool {
    let n = rhs.len();
    assert!(n > MAX_LEN_SIMPLE && lhs.len() == 2 * n);
    // To guarantee n_lo >= 2.
    const_assert!(MAX_LEN_SIMPLE >= 3);
    let n_lo = n / 2;

    // Divide lhs[n_lo..] by rhs, putting quotient in lhs[n+n_lo..] and remainder in lhs[n_lo..n+n_lo].
    let overflow = div_rem_in_place_divide_conquer_small_quotient(
        &mut lhs[n_lo..],
        rhs,
        fast_div_rhs_top,
        temp,
    );

    // Divide lhs[..n+n_lo] by rhs, putting the rest of the quotient in lhs[n..n+n_lo] and remainder
    // in lhs[..n].
    let overflow_lo = div_rem_in_place_divide_conquer_small_quotient(
        &mut lhs[..n + n_lo],
        rhs,
        fast_div_rhs_top,
        temp,
    );
    assert!(!overflow_lo);

    overflow
}

/// Division in place using divide and conquer.
/// Quotient length < divisor length.
///
/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
fn div_rem_in_place_divide_conquer_small_quotient(
    lhs: &mut [Word],
    rhs: &[Word],
    fast_div_rhs_top: FastDiv21,
    temp: &mut [Word],
) -> bool {
    let n = rhs.len();
    assert!(n >= 2 && lhs.len() >= n);
    let m = lhs.len() - n;
    assert!(m < n);
    if m <= MAX_LEN_SIMPLE {
        return div_rem_in_place_simple(lhs, rhs, fast_div_rhs_top);
    }
    const_assert!(MAX_LEN_SIMPLE >= 1);
    // Use top m words of the divisor to get a quotient approximation. It may be too large by at most 2.
    // Quotient is in lhs[n..], remainder in lhs[..n].
    // This is 2m / m division.
    let mut q_overflow: SignedWord = div_rem_in_place_divide_conquer_same_len(
        &mut lhs[n - m..],
        &rhs[n - m..],
        fast_div_rhs_top,
        temp,
    )
    .into();
    let (rem, q) = lhs.split_at_mut(n);

    // Subtract q * (the rest of rhs) from rem.
    // The multiplication here is m words by * (n-m) words.
    let mut rem_overflow: SignedWord = mul::add_signed_mul(rem, Negative, q, &rhs[..n - m], temp);
    if q_overflow != 0 {
        rem_overflow -= SignedWord::from(add::sub_same_len_in_place(&mut rem[m..], &rhs[..n - m]));
    }

    // If the remainder overflowed, adjust q and rem.
    while rem_overflow < 0 {
        rem_overflow += SignedWord::from(add::add_same_len_in_place(rem, rhs));
        q_overflow -= SignedWord::from(add::sub_one_in_place(q));
    }

    assert!(rem_overflow == 0 && q_overflow >= 0 && q_overflow <= 1);
    q_overflow != 0
}

/// Fast repeated division of 2 words by a given word.
#[derive(Clone, Copy)]
struct FastDiv21 {
    divisor: Word,
    reciprocal: Word,
}

impl FastDiv21 {
    /// Initialize from a given normalized divisor.
    fn by(divisor: Word) -> FastDiv21 {
        debug_assert!(divisor.leading_zeros() == 0);

        let (recip_lo, recip_hi) = split_double_word(DoubleWord::MAX / extend_word(divisor));
        debug_assert!(recip_hi == 1);

        FastDiv21 {
            divisor,
            reciprocal: recip_lo,
        }
    }

    /// Divide a value.
    /// The result must fit in a single word.
    fn div_rem(&self, dividend: DoubleWord) -> (Word, Word) {
        let (_, dividend_hi) = split_double_word(dividend);
        // Approximate quotient: it may be too small by at most 3.
        // self.reciprocal + (1<<BITS) is approximately (1<<(2*BITS)) / self.divisor.
        let (_, mul_hi) =
            split_double_word(extend_word(self.reciprocal) * extend_word(dividend_hi));
        let mut quotient = mul_hi + dividend_hi;
        let mut remainder = dividend - extend_word(self.divisor) * extend_word(quotient);
        while remainder >= extend_word(self.divisor) {
            quotient += 1;
            remainder -= extend_word(self.divisor);
        }
        let (rem_lo, _) = split_double_word(remainder);
        (quotient, rem_lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_by_word_in_place_empty() {
        let mut a = [];
        let rem = div_by_word_in_place(&mut a, 7);
        assert_eq!(rem, 0);
    }

    #[test]
    fn test_rem_by_word_empty() {
        let a = [];
        let rem = rem_by_word(&a, 7);
        assert_eq!(rem, 0);
    }
}
