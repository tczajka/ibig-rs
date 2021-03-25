//! Modular addition and subtraction.

use crate::{
    add, cmp,
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
};
use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

impl<'a> Neg for Modulo<'a> {
    type Output = Modulo<'a>;

    fn neg(mut self) -> Modulo<'a> {
        match self.repr_mut() {
            ModuloRepr::Small(self_small) => self_small.negate_in_place(),
            ModuloRepr::Large(self_large) => self_large.negate_in_place(),
        }
        self
    }
}

impl<'a> Neg for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn neg(self) -> Modulo<'a> {
        self.clone().neg()
    }
}

impl<'a> Add<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn add(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.add(&rhs)
    }
}

impl<'a> Add<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn add(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.add_assign(rhs);
        self
    }
}

impl<'a> Add<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn add(self, rhs: Modulo<'a>) -> Modulo<'a> {
        rhs.add(self)
    }
}

impl<'a> Add<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn add(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().add(rhs)
    }
}

impl<'a> AddAssign<Modulo<'a>> for Modulo<'a> {
    fn add_assign(&mut self, rhs: Modulo<'a>) {
        self.add_assign(&rhs)
    }
}

impl<'a> AddAssign<&Modulo<'a>> for Modulo<'a> {
    fn add_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.add_in_place(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.add_in_place(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl<'a> Sub<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn sub(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.sub(&rhs)
    }
}

impl<'a> Sub<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn sub(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.sub_assign(rhs);
        self
    }
}

impl<'a> Sub<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn sub(self, mut rhs: Modulo<'a>) -> Modulo<'a> {
        match (self.repr(), rhs.repr_mut()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.sub_in_place_swap(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.sub_in_place_swap(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
        rhs
    }
}

impl<'a> Sub<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn sub(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().sub(rhs)
    }
}

impl<'a> SubAssign<Modulo<'a>> for Modulo<'a> {
    fn sub_assign(&mut self, rhs: Modulo<'a>) {
        self.sub_assign(&rhs)
    }
}

impl<'a> SubAssign<&Modulo<'a>> for Modulo<'a> {
    fn sub_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.sub_in_place(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.sub_in_place(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl<'a> ModuloSmall<'a> {
    /// self = -self
    fn negate_in_place(&mut self) {
        let ring = self.ring();
        let val = match self.normalized_value() {
            0 => 0,
            x => ring.normalized_modulus() - x,
        };
        self.set_normalized_value(val)
    }

    /// self += rhs
    fn add_in_place(&mut self, rhs: &ModuloSmall<'a>) {
        self.check_same_ring(rhs);
        let (mut val, overflow) = self
            .normalized_value()
            .overflowing_add(rhs.normalized_value());
        let m = self.ring().normalized_modulus();
        if overflow || val >= m {
            let (v, overflow2) = val.overflowing_sub(m);
            debug_assert_eq!(overflow, overflow2);
            val = v;
        }
        self.set_normalized_value(val)
    }

    /// self -= rhs
    fn sub_in_place(&mut self, rhs: &ModuloSmall<'a>) {
        self.check_same_ring(rhs);
        let (mut val, overflow) = self
            .normalized_value()
            .overflowing_sub(rhs.normalized_value());
        if overflow {
            let m = self.ring().normalized_modulus();
            let (v, overflow2) = val.overflowing_add(m);
            debug_assert!(overflow2);
            val = v;
        }
        self.set_normalized_value(val)
    }

    /// rhs = self - rhs
    fn sub_in_place_swap(&self, rhs: &mut ModuloSmall<'a>) {
        self.check_same_ring(rhs);
        let (mut val, overflow) = self
            .normalized_value()
            .overflowing_sub(rhs.normalized_value());
        if overflow {
            let m = self.ring().normalized_modulus();
            let (v, overflow2) = val.overflowing_add(m);
            debug_assert!(overflow2);
            val = v;
        }
        rhs.set_normalized_value(val)
    }
}

impl<'a> ModuloLarge<'a> {
    /// self = -self
    fn negate_in_place(&mut self) {
        self.modify_normalized_value(|words, ring| {
            if !words.iter().all(|w| *w == 0) {
                let overflow = add::sub_same_len_in_place_swap(ring.normalized_modulus(), words);
                assert!(!overflow);
            }
        });
    }

    /// self += rhs
    fn add_in_place(&mut self, rhs: &ModuloLarge<'a>) {
        self.check_same_ring(rhs);
        let rhs_words = rhs.normalized_value();
        self.modify_normalized_value(|words, ring| {
            let modulus = ring.normalized_modulus();
            let overflow = add::add_same_len_in_place(words, rhs_words);
            if overflow || cmp::cmp_same_len(words, modulus) >= Ordering::Equal {
                let overflow2 = add::sub_same_len_in_place(words, modulus);
                debug_assert_eq!(overflow, overflow2);
            }
        });
    }

    /// self -= rhs
    fn sub_in_place(&mut self, rhs: &ModuloLarge<'a>) {
        self.check_same_ring(rhs);
        let rhs_words = rhs.normalized_value();
        self.modify_normalized_value(|words, ring| {
            let modulus = ring.normalized_modulus();
            let overflow = add::sub_same_len_in_place(words, rhs_words);
            if overflow {
                let overflow2 = add::add_same_len_in_place(words, modulus);
                debug_assert!(overflow2);
            }
        });
    }

    /// rhs = self - rhs
    fn sub_in_place_swap(&self, rhs: &mut ModuloLarge<'a>) {
        self.check_same_ring(rhs);
        let words = self.normalized_value();
        rhs.modify_normalized_value(|rhs_words, ring| {
            let modulus = ring.normalized_modulus();
            let overflow = add::sub_same_len_in_place_swap(words, rhs_words);
            if overflow {
                let overflow2 = add::add_same_len_in_place(rhs_words, modulus);
                debug_assert!(overflow2);
            }
        });
    }
}
