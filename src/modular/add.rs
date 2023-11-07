//! Modular addition and subtraction.

use crate::{modular::modulo::Modulo, UBig};
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

impl Neg for Modulo {
    type Output = Modulo;

    #[inline]
    fn neg(mut self) -> Modulo {
        if self.value() == UBig::from_word(0) {
            self
        } else {
            self.ring()
                .from_normalized_value(self.ring().normalized_modulus() - self.normalized_value())
        }
    }
}

impl Neg for &Modulo {
    type Output = Modulo;

    #[inline]
    fn neg(self) -> Modulo {
        self.clone().neg()
    }
}

impl Add<Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn add(mut self, rhs: Modulo) -> Modulo {
        self.add_assign(rhs);
        self
    }
}

impl Add<&Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn add(mut self, rhs: &Modulo) -> Modulo {
        self.add_assign(rhs);
        self
    }
}

impl Add<Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn add(self, rhs: Modulo) -> Modulo {
        rhs.add(self)
    }
}

impl Add<&Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn add(self, rhs: &Modulo) -> Modulo {
        self.check_same_ring(rhs);
        let mut val = self.normalized_value() + rhs.normalized_value();
        if val >= self.ring().normalized_modulus() {
            val -= self.ring().normalized_modulus();
        }
        self.ring().from_normalized_value(val)
    }
}

impl AddAssign<Modulo> for Modulo {
    #[inline]
    fn add_assign(&mut self, rhs: Modulo) {
        self.check_same_ring(rhs);
        let mut val = self.take_normalized_value() + rhs.take_normalized_value();
        if val >= self.ring().normalized_modulus() {
            val -= self.ring().normalized_modulus();
        }
        self.set_normalized_value(val);
    }
}

impl AddAssign<&Modulo> for Modulo {
    #[inline]
    fn add_assign(&mut self, rhs: &Modulo) {
        self.check_same_ring(rhs);
        let mut val = self.take_normalized_value() + rhs.normalized_value();
        if val >= self.ring().normalized_modulus() {
            val -= self.ring().normalized_modulus();
        }
        self.set_normalized_value(val);
    }
}

impl Sub<Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn sub(mut self, rhs: Modulo) -> Modulo {
        self.sub_assign(rhs);
        self
    }
}

impl Sub<Modulo> for Modulo {
    type Output = Modulo;

    #[inline]
    fn sub(mut self, rhs: &Modulo) -> Modulo {
        self.sub_assign(rhs);
        self
    }
}

impl Sub<Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn sub(self, rhs: Modulo) -> Modulo {
        self.check_same_ring(rhs);
        let val = self.normalized_value();
        let rhs_val = rhs.take_normalized_value();
        let val = if val < rhs_val {
            val + (self.ring().normalized_modulus() - rhs_val)
        } else {
            val - rhs_val
        };
        rhs.set_normalized_value(val);
        rhs
    }
}

impl Sub<&Modulo> for &Modulo {
    type Output = Modulo;

    #[inline]
    fn sub(self, rhs: &Modulo) -> Modulo {
        self.check_same_ring(rhs);
        let val = self.normalized_value();
        let rhs = rhs.normalized_value();
        let val = if val < rhs {
            val + (self.ring().normalized_modulus() - rhs)
        } else {
            val - rhs
        };
        self.ring().from_normalized_value(val)
    }
}

impl SubAssign<Modulo> for Modulo {
    #[inline]
    fn sub_assign(&mut self, rhs: Modulo) {
        self.check_same_ring(rhs);
        let mut val = self.take_normalized_value();
        let rhs = rhs.take_normalized_value();
        if val < rhs {
            val += self.ring().normalized_modulus();
        }
        val -= rhs;
        self.set_normalized_value(val);
    }
}

impl SubAssign<&Modulo> for Modulo {
    #[inline]
    fn sub_assign(&mut self, rhs: &Modulo) {
        self.check_same_ring(rhs);
        let mut val = self.take_normalized_value();
        let rhs = rhs.normalized_value();
        if val < rhs {
            val += self.ring().normalized_modulus();
        }
        val -= rhs;
        self.set_normalized_value(val);
    }
}
