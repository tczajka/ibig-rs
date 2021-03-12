//! Implement num-traits traits.

use crate::{ibig::IBig, parse::ParseError, sign::Abs, ubig::UBig};
use num_traits;

impl num_traits::Zero for UBig {
    fn zero() -> Self {
        Self::from(0u8)
    }

    fn is_zero(&self) -> bool {
        *self == Self::from(0u8)
    }
}

impl num_traits::Zero for IBig {
    fn zero() -> Self {
        Self::from(0u8)
    }

    fn is_zero(&self) -> bool {
        *self == Self::from(0u8)
    }
}

impl num_traits::One for UBig {
    fn one() -> Self {
        Self::from(1u8)
    }
}

impl num_traits::One for IBig {
    fn one() -> Self {
        Self::from(1u8)
    }
}

impl num_traits::Pow<usize> for UBig {
    type Output = UBig;

    fn pow(self, rhs: usize) -> UBig {
        (&self).pow(rhs)
    }
}

impl num_traits::Pow<usize> for &UBig {
    type Output = UBig;

    fn pow(self, rhs: usize) -> UBig {
        self.pow(rhs)
    }
}

impl num_traits::Pow<usize> for IBig {
    type Output = IBig;

    fn pow(self, rhs: usize) -> IBig {
        (&self).pow(rhs)
    }
}

impl num_traits::Pow<usize> for &IBig {
    type Output = IBig;

    fn pow(self, rhs: usize) -> IBig {
        self.pow(rhs)
    }
}

impl num_traits::Unsigned for UBig {}

impl num_traits::Signed for IBig {
    fn abs(&self) -> Self {
        Abs::abs(self)
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Abs::abs(self - other)
    }

    fn signum(&self) -> Self {
        self.signum()
    }

    fn is_positive(&self) -> bool {
        *self > IBig::from(0u8)
    }

    fn is_negative(&self) -> bool {
        *self < IBig::from(0u8)
    }
}

impl num_traits::Num for UBig {
    type FromStrRadixErr = ParseError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseError> {
        Self::from_str_radix(s, radix)
    }
}

impl num_traits::Num for IBig {
    type FromStrRadixErr = ParseError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseError> {
        Self::from_str_radix(s, radix)
    }
}
