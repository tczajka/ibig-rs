use crate::{
    ibig::IBig,
    ubig::{Repr::*, UBig},
};
use alloc::borrow::Cow;
use core::ops::Neg;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

use Sign::*;

impl Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Sign {
        match self {
            Positive => Negative,
            Negative => Positive,
        }
    }
}

impl IBig {
    /// A number representing the sign of `self`.
    ///
    /// * -1 if the number is negative
    /// * 0 if the number is zero
    /// * 1 if the number is positive
    ///
    /// # Examples
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-500).signum(), ibig!(-1));
    /// ```
    #[inline]
    pub fn signum(&self) -> IBig {
        match self.sign() {
            Positive => {
                if let Small(0) = self.magnitude().repr() {
                    IBig::from_sign_magnitude(Positive, UBig::from_word(0))
                } else {
                    IBig::from_sign_magnitude(Positive, UBig::from_word(1))
                }
            }
            Negative => IBig::from_sign_magnitude(Negative, UBig::from_word(1)),
        }
    }
}

impl_unary_operator!(impl Neg for IBig, neg, neg_cow);

fn neg_cow(x: Cow<IBig>) -> IBig {
    let (sign, mag) = x.into_owned().into_sign_magnitude();
    IBig::from_sign_magnitude(-sign, mag)
}

/// Absolute value.
///
/// # Examples
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ibig!(-5).abs(), ibig!(5));
/// ```
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

impl_unary_operator!(impl Abs for IBig, abs, abs_cow);

fn abs_cow(x: Cow<IBig>) -> IBig {
    let (_, mag) = x.into_owned().into_sign_magnitude();
    IBig::from_sign_magnitude(Positive, mag)
}

/// Unsigned absolute value.
///
/// # Examples
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ibig!(-5).unsigned_abs(), ubig!(5));
/// ```
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

impl UnsignedAbs for IBig {
    type Output = UBig;

    #[inline]
    fn unsigned_abs(self) -> UBig {
        let (_, mag) = self.into_sign_magnitude();
        mag
    }
}

impl UnsignedAbs for &IBig {
    type Output = UBig;

    #[inline]
    fn unsigned_abs(self) -> UBig {
        self.magnitude().clone()
    }
}
