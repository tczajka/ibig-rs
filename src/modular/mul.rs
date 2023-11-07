use crate::modular::modulo::Modulo;
use core::ops::{Mul, MulAssign};

impl Mul<Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn mul(mut self, rhs: Modulo) -> Modulo {
        self.mul_assign(rhs);
        self
    }
}

impl Mul<&Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn mul(mut self, rhs: &Modulo) -> Modulo {
        self.mul_assign(rhs);
        self
    }
}

impl Mul<Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn mul(self, rhs: Modulo) -> Modulo {
        rhs.mul(self)
    }
}

impl Mul<&Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn mul(self, rhs: &Modulo) -> Modulo {
        self.check_same_ring(rhs);
        let mut val = (self.normalized_value() >> self.ring().shift()) * rhs.normalized_value();
        // TODO: Optimize using fast_div_top.
        val %= self.ring().normalized_modulus();
        self.ring().from_normalized_value(val)
    }
}

impl MulAssign<Modulo> for Modulo {
    #[inline]
    fn mul_assign(&mut self, rhs: Modulo) {
        self.check_same_ring(rhs);
        let mut val =
            (self.take_normalized_value() >> self.ring().shift()) * rhs.take_normalized_value();
        // TODO: Optimize using fast_div_top.
        val %= self.ring().normalized_modulus();
        self.set_normalized_value(val);
    }
}

impl MulAssign<&Modulo> for Modulo {
    #[inline]
    fn mul_assign(&mut self, rhs: &Modulo) {
        self.check_same_ring(rhs);
        let mut val =
            (self.take_normalized_value() >> self.ring().shift()) * rhs.normalized_value();
        // TODO: Optimize using fast_div_top.
        val %= self.ring().normalized_modulus();
        self.set_normalized_value(val);
    }
}
