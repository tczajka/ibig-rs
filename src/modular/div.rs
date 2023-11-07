use crate::{ibig::IBig, modular::modulo::Modulo, ops::RemEuclid, ubig::UBig};
use core::ops::{Div, DivAssign};

impl Modulo {
    /// Inverse.
    ///
    /// Returns `None` if there is no unique inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(10));
    /// assert_eq!(ring.from(7).inverse(), Some(ring.from(3)));
    /// assert_eq!(ring.from(2).inverse(), None);
    /// ```
    pub fn inverse(&self) -> Option<Modulo> {
        let a = self.residue();
        let b = self.ring().modulus();
        let (gcd, x, _) = a.extended_gcd(&b);
        if gcd == UBig::from_word(1) {
            // TODO: Get rid of redundant remainder computations.
            let res: UBig = x.rem_euclid(IBig::from(b)).try_into().unwrap();
            Some(self.ring().from_ubig(res))
        } else {
            None
        }
    }
}

impl Div<Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn div(self, rhs: Modulo) -> Modulo {
        self.div(&rhs)
    }
}

impl Div<&Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn div(mut self, rhs: &Modulo) -> Modulo {
        self.div_assign(rhs);
        self
    }
}

impl Div<Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn div(self, rhs: Modulo) -> Modulo {
        self.div(&rhs)
    }
}

impl Div<&Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn div(self, rhs: &Modulo) -> Modulo {
        let inv_rhs = rhs.inverse.expect("Division by a non-invertible Modulo");
        self * inv_rhs
    }
}

impl DivAssign<Modulo> for Modulo {
    #[inline]
    fn div_assign(&mut self, rhs: Modulo) {
        self.div_assign(&rhs)
    }
}

impl<'a> DivAssign<&Modulo> for Modulo {
    #[inline]
    fn div_assign(&mut self, rhs: &Modulo) {
        let inv_rhs = rhs.inverse.expect("Division by a non-invertible Modulo");
        self.mul_assign(inv_rhs);
    }
}
