use crate::{
    ibig::IBig,
};
use alloc::borrow::Cow;
use core::{
    ops::Neg,
};

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
