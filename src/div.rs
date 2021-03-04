use crate::{
    add,
    buffer::Buffer,
    ibig::IBig,
    mul,
    primitive::{double_word, extend_word, split_double_word, DoubleWord, Word, WORD_BITS},
    shift,
    sign::Abs,
    ubig::{Repr::*, UBig},
};
use core::{
    iter, mem,
    ops::{Div, DivAssign, Rem, RemAssign},
};

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
        let rem = div_rem_by_word_in_place(&mut buffer, rhs);
        (buffer.into(), UBig::from_word(rem))
    }

    /// `lhs / rhs`
    fn div_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let _ = UBig::div_normalize(&mut lhs, &mut rhs);
        div_rem_in_place(&mut lhs, &rhs);
        UBig::shr_large_words(lhs, rhs.len())
    }

    /// `lhs % rhs`
    fn rem_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let sh = UBig::div_normalize(&mut lhs, &mut rhs);
        div_rem_in_place(&mut lhs, &rhs);
        lhs.truncate(rhs.len());
        shift::shr_in_place(&mut lhs, sh);
        lhs.into()
    }

    /// `(lhs / rhs, lhs % rhs)`
    fn div_rem_large(mut lhs: Buffer, mut rhs: Buffer) -> (UBig, UBig) {
        let sh = UBig::div_normalize(&mut lhs, &mut rhs);
        div_rem_in_place(&mut lhs, &rhs);
        let q = UBig::shr_large_ref_words(&lhs, rhs.len());
        lhs.truncate(rhs.len());
        shift::shr_in_place(&mut lhs, sh);
        (q, lhs.into())
    }

    /// Normalizes large arguments for division by shifting them left:
    /// * lhs as least as long as rhs
    /// * top words of lhs smaller than rhs.
    /// * top bit of rhs is 1
    ///
    /// Returns shift size.
    fn div_normalize(lhs: &mut Buffer, rhs: &mut Buffer) -> u32 {
        assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let sh = rhs.last().unwrap().leading_zeros();
        assert!(sh != WORD_BITS);
        let rhs_carry = shift::shl_in_place(rhs, sh);
        assert!(rhs_carry == 0);
        let lhs_carry = shift::shl_in_place(lhs, sh);
        if lhs_carry != 0 || lhs.last().unwrap() >= rhs.last().unwrap() {
            lhs.push_may_reallocate(lhs_carry);
        }
        sh
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
pub(crate) fn div_rem_by_word_in_place(words: &mut [Word], rhs: Word) -> Word {
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
    let fast_division = FastDivision::by(rhs << shift);

    for word in words.iter_mut().rev() {
        let a = double_word(*word, rem);
        let (q, r) = fast_division.div_rem(a);
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
    let fast_division = FastDivision::by(rhs << shift);

    if shift == 0 {
        let mut rem: Word = 0;
        for word in words.iter().rev() {
            let a = double_word(*word, rem);
            let (_, r) = fast_division.div_rem(a);
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
            let (_, r) = fast_division.div_rem(a);
            rem = r;
        }
        rem >> shift
    }
}

/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// rhs must have top bit of 1.
/// Inputs must be such that the quotient fits, i.e. top words of lhs must be smaller than rhs.
///
/// lhs = [lhs / rhs, lhs % rhs]
fn div_rem_in_place(lhs: &mut [Word], rhs: &[Word]) {
    assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
    debug_assert!(rhs.last().unwrap().leading_zeros() == 0);
    div_rem_in_place_simple(lhs, rhs);
}

/// Simple division algorithm in place.
///
/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// rhs must have top bit of 1.
/// Inputs must be such that the quotient fits, i.e. top words of lhs must be smaller than rhs.
///
/// lhs = [lhs / rhs, lhs % rhs]
fn div_rem_in_place_simple(lhs: &mut [Word], rhs: &[Word]) {
    // The Art of Computer Programming, algorithm 4.3.1D.

    let n = rhs.len();
    assert!(n >= 2);
    let rhs0 = rhs[n - 1];
    let rhs1 = rhs[n - 2];
    debug_assert!(rhs0.leading_zeros() == 0);

    let fast_division_rhs0 = FastDivision::by(rhs0);

    let mut lhs_len = lhs.len();
    assert!(lhs_len >= n);

    while lhs_len > n {
        let lhs0 = lhs[lhs_len - 1];
        let lhs1 = lhs[lhs_len - 2];
        let lhs2 = lhs[lhs_len - 3];
        let lhs01 = double_word(lhs1, lhs0);

        // Approximate the next word of quotient by
        // q = floor([lhs0, lhs1] / rhs0)
        // r = remainder (or None if overflow)

        // exact_q = floor([lhs0, lhs1, ...] / [rhs0, ...])
        // [lhs0, lhs1, ...] / [rhs0, ...] < ([lhs0, lhs1] + [0..1)) / rhs0
        // exact_q <= floor(([lhs0, lhs1] + [0..1)) / rhs0) = q
        //
        // B = WORD_BITS, rhs0 >= 2^(B-1)
        //
        // [lhs0, lhs1, ...] / [rhs0, ...] > [lhs0, lhs1] / (rhs0 + 1)
        //   = [lhs0, lhs1] / rhs0 * (1 - 1 / (rhs0+1))
        //   >= q * (1 - 1/(rhs0+1)) = q - (q / (rhs0+1))
        //   >= q - (2^B+1) / 2^(B-1) > q-2
        // exact_q >= q-2
        //
        // Therefore q is never too small and at most 2 too large.

        // Then improve the approximation:
        // q' = min(floor([lhs0, lhs1, lhs2] / [rhs0, rhs1]), Word::MAX)
        // q'-1 <= exact_q <= q' <= q
        // Most of the time exact_q = q'.
        // (by the same reasoning as above, except with 2B instead of B).

        // We must decrease q at most twice.
        // [lhs0, lhs1] = q * rhs0 + r
        //
        // q must be decreased if:
        // q-1 >= floor([lhs0, lhs1, lhs2] / [rhs0, rhs1])
        // q > [lhs0, lhs1, lhs2] / [rhs0, rhs1]
        // q * [rhs0, rhs1] > [lhs0, lhs1, lhs2]
        // q * rhs1 > [r, lhs2]
        let mut q = if lhs0 < rhs0 {
            let (mut q, mut r) = fast_division_rhs0.div_rem(lhs01);
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
}

/// Fast repeated division by a given value.
struct FastDivision {
    divisor: Word,
    reciprocal: Word,
}

impl FastDivision {
    /// Initialize from a given normalized divisor.
    fn by(divisor: Word) -> FastDivision {
        debug_assert!(divisor.leading_zeros() == 0);

        let (recip_lo, recip_hi) = split_double_word(DoubleWord::MAX / extend_word(divisor));
        debug_assert!(recip_hi == 1);

        FastDivision {
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
    fn test_div_rem_by_word_in_place_empty() {
        let mut a = [];
        let rem = div_rem_by_word_in_place(&mut a, 7);
        assert_eq!(rem, 0);
    }

    #[test]
    fn test_rem_by_word_empty() {
        let a = [];
        let rem = rem_by_word(&a, 7);
        assert_eq!(rem, 0);
    }
}
