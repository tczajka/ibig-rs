use crate::{
    math,
    memory::MemoryAllocation,
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
    primitive::WORD_BITS,
    ubig::{Repr::*, UBig},
};

impl Modulo<'_> {
    /// Exponentiation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{prelude::*, modular::ModuloRing};
    /// // A Mersenne prime.
    /// let p = ubig!(2).pow(607) - ubig!(1);
    /// let ring = ModuloRing::new(&p);
    /// // Fermat's little theorem: a^(p-1) = 1 (mod p)
    /// let a = ring.from(123);
    /// assert_eq!(a.pow(&(p - ubig!(1))), ring.from(1));
    /// ```
    pub fn pow(&self, exp: &UBig) -> Modulo {
        // Special cases.
        match exp.repr() {
            // self^0 == 1
            Small(0) => {}
            // self^1 == self
            Small(1) => return self.clone(),
            // self^2 == self * self
            Small(2) => return self * self,
            // exp > 2 => fall through to slower algorithm
            _ => {}
        }

        match self.repr() {
            ModuloRepr::Small(self_small) => self_small.pow(exp).into(),
            ModuloRepr::Large(self_large) => self_large.pow(exp).into(),
        }
    }
}

impl ModuloSmall<'_> {
    /// Exponentiation.
    fn pow(&self, exp: &UBig) -> ModuloSmall {
        match exp.repr() {
            // self^0 == 1
            Small(0) => ModuloSmall::from_ubig(&UBig::from_word(1), self.ring()),
            // self^1 == self
            Small(1) => self.clone(),
            // self^2 == self * self
            Small(2) => {
                let mut a = self.clone();
                a.mul_in_place(self);
                a
            }
            _ => self.pow_nontrivial(exp),
        }
    }

    fn pow_nontrivial(&self, exp: &UBig) -> ModuloSmall {
        debug_assert!(*exp >= UBig::from_word(3));

        let exp_words = exp.as_words();
        let mut val = self.clone();
        let mut word_idx = exp_words.len() - 1;
        let mut word = exp_words[word_idx];
        let mut bit = math::bit_len(word) - 1;
        loop {
            // val = self ^ exp[(word_idx,bit)..]
            if bit == 0 {
                if word_idx == 0 {
                    break;
                }
                word_idx -= 1;
                word = exp_words[word_idx];
                bit = WORD_BITS;
            }
            bit -= 1;
            val.square_in_place();
            if word & (1 << bit) != 0 {
                val.mul_in_place(self);
            }
        }
        val
    }
}

impl ModuloLarge<'_> {
    fn pow(&self, exp: &UBig) -> ModuloLarge {
        match exp.repr() {
            // self^0 == 1
            Small(0) => ModuloLarge::from_ubig(UBig::from_word(1), self.ring()),
            // self^1 == self
            Small(1) => self.clone(),
            _ => self.pow_nontrivial(exp),
        }
    }

    fn pow_nontrivial(&self, exp: &UBig) -> ModuloLarge {
        debug_assert!(*exp >= UBig::from_word(2));

        let memory_requirement = self.ring().mul_memory_requirement();
        let mut allocation = MemoryAllocation::new(memory_requirement);
        let mut memory = allocation.memory();

        let exp_words = exp.as_words();
        let mut val = self.clone();

        let mut word_idx = exp_words.len() - 1;
        let mut word = exp_words[word_idx];
        let mut bit = math::bit_len(word) - 1;
        loop {
            // val = self ^ exp[(word_idx,bit)..]
            if bit == 0 {
                if word_idx == 0 {
                    break;
                }
                word_idx -= 1;
                word = exp_words[word_idx];
                bit = WORD_BITS;
            }
            bit -= 1;
            val.square_in_place(&mut memory);
            if word & (1 << bit) != 0 {
                val.mul_in_place(self, &mut memory);
            }
        }
        val
    }
}
