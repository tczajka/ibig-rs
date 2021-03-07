use crate::{
    buffer::Buffer,
    div,
    ibig::IBig,
    primitive::{Word, WORD_BITS},
    shift,
    sign::Abs,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
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
        UBig::from_word(div::rem_by_word(lhs, rhs))
    }

    /// (buffer / rhs, buffer % rhs)
    fn div_rem_large_word(mut buffer: Buffer, rhs: Word) -> (UBig, UBig) {
        if rhs == 0 {
            panic_divide_by_0();
        }
        let rem = div::div_by_word_in_place(&mut buffer, rhs);
        (buffer.into(), UBig::from_word(rem))
    }

    /// `lhs / rhs`
    fn div_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let (_shift, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let carry = div::div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
        if carry {
            lhs.push_may_reallocate(1);
        }
        UBig::shr_large_words(lhs, rhs.len())
    }

    /// `lhs % rhs`
    fn rem_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let (sh, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let _carry = div::div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
        let n = rhs.len();
        rhs.copy_from_slice(&lhs[..n]);
        shift::shr_in_place(&mut rhs, sh);
        rhs.into()
    }

    /// `(lhs / rhs, lhs % rhs)`
    fn div_rem_large(mut lhs: Buffer, mut rhs: Buffer) -> (UBig, UBig) {
        let (sh, fast_div_rhs_top) = UBig::div_normalize(&mut lhs, &mut rhs);
        let carry = div::div_rem_in_place(&mut lhs, &rhs, fast_div_rhs_top);
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
    fn div_normalize(lhs: &mut Buffer, rhs: &mut [Word]) -> (u32, div::FastDiv21) {
        assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let sh = rhs.last().unwrap().leading_zeros();
        assert!(sh != WORD_BITS);
        let rhs_carry = shift::shl_in_place(rhs, sh);
        assert!(rhs_carry == 0);
        let lhs_carry = shift::shl_in_place(lhs, sh);
        if lhs_carry != 0 {
            lhs.push_may_reallocate(lhs_carry);
        }
        (sh, div::FastDiv21::by(*rhs.last().unwrap()))
    }
}

fn panic_divide_by_0() -> ! {
    panic!("divide by 0")
}
