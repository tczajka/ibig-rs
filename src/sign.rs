use crate::{
    ibig::IBig,
    ubig::{Repr::*, UBig},
};
use core::ops::Neg;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl UBig {
    /// Is it zero?
    pub(crate) fn is_zero(&self) -> bool {
        match self.repr() {
            Small(0) => true,
            _ => false,
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
    pub fn signum(&self) -> IBig {
        match self.sign() {
            Positive => {
                if self.magnitude().is_zero() {
                    IBig::from(0u8)
                } else {
                    IBig::from(1u8)
                }
            }
            Negative => IBig::from(-1i8),
        }
    }

    /// Is the number smaller than 0?
    ///
    /// # Examples
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-5).is_negative(), true);
    /// assert_eq!(ibig!(0).is_negative(), false);
    /// assert_eq!(ibig!(5).is_negative(), false);
    /// ```
    pub fn is_negative(&self) -> bool {
        self.sign() == Negative
    }

    /// Is the number greater than 0?
    ///
    /// # Examples
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-5).is_positive(), false);
    /// assert_eq!(ibig!(0).is_positive(), false);
    /// assert_eq!(ibig!(5).is_positive(), true);
    /// ```
    pub fn is_positive(&self) -> bool {
        self.sign() == Positive && !self.magnitude().is_zero()
    }
}

impl Neg for IBig {
    type Output = IBig;

    fn neg(self) -> IBig {
        let (sign, mag) = self.into_sign_magnitude();
        IBig::from_sign_magnitude(-sign, mag)
    }
}

impl Neg for &IBig {
    type Output = IBig;

    fn neg(self) -> IBig {
        self.clone().neg()
    }
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

impl Abs for IBig {
    type Output = IBig;

    fn abs(self) -> IBig {
        IBig::from(self.unsigned_abs())
    }
}

impl Abs for &IBig {
    type Output = IBig;

    fn abs(self) -> IBig {
        IBig::from(self.unsigned_abs())
    }
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

    fn unsigned_abs(self) -> UBig {
        let (_, mag) = self.into_sign_magnitude();
        mag
    }
}

impl UnsignedAbs for &IBig {
    type Output = UBig;

    fn unsigned_abs(self) -> UBig {
        self.magnitude().clone()
    }
}
