//! Modular addition and subtraction.

use crate::{
    add,
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
};
use core::ops::Neg;

impl Neg for Modulo<'_> {
    type Output = Self;

    fn neg(mut self) -> Self {
        match self.repr_mut() {
            ModuloRepr::Small(self_small) => self_small.negate_in_place(),
            ModuloRepr::Large(self_large) => self_large.negate_in_place(),
        }
        self
    }
}

impl ModuloSmall<'_> {
    fn negate_in_place(&mut self) {
        let ring = self.ring();
        let val = match self.normalized_value() {
            0 => 0,
            x => ring.normalized_modulus() - x,
        };
        self.set_normalized_value(val)
    }
}

impl ModuloLarge<'_> {
    fn negate_in_place(&mut self) {
        self.modify_normalized_value(|words, ring| {
            if !words.iter().all(|w| *w == 0) {
                let overflow = add::sub_same_len_in_place_swap(ring.normalized_modulus(), words);
                assert!(!overflow);
            }
        });
    }
}
