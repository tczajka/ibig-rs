use crate::{
    ibig::IBig,
    primitive::Sign::{self, *},
};
use core::ops::Neg;

impl Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Sign {
        match self {
            Positive => Negative,
            Negative => Positive,
        }
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
        -self.clone()
    }
}
