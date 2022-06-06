use crate::{
    arch::word::Word,
    math,
    memory::{self, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::ModuloRingSmall,
    },
    primitive::{double_word, split_double_word, PrimitiveUnsigned, WORD_BITS, WORD_BITS_USIZE},
    ubig::{Repr::*, UBig},
    gcd::xgcd_word_by_word
};

impl<'a> Modulo<'a> {
    /// Multiplicative inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// // A Mersenne prime.
    /// let p = ubig!(2).pow(607) - ubig!(1);
    /// let ring = ModuloRing::new(&p);
    /// // Fermat's little theorem: a^(p-2) = a^-1 (mod p)
    /// let a = ring.from(123);
    /// assert_eq!(a.inv().unwrap(), a.pow(&(p - ubig!(2))));
    /// ```
    #[inline]
    pub fn inv(&self) -> Option<Modulo<'a>> {
        match self.repr() {
            ModuloRepr::Small(self_small) => self_small.inv().map(Into::into),
            ModuloRepr::Large(self_large) => self_large.inv().map(Into::into),
        }
    }
}

impl<'a> ModuloSmall<'a> {
    /// Inverse.
    #[inline]
    fn inv(&self) -> Option<ModuloSmall<'a>> {
        if self.raw().normalized() == 0 {
            None
        } else {
            let (g, _, coeff) = xgcd_word_by_word(self.ring().modulus(), self.raw().normalized(), true);
            assert!(coeff > 0);
            if g == 1 {
                Some(ModuloSmall::new(ModuloSmallRaw::from_normalized(coeff as u64), self.ring()))
            } else {
                None
            }
        }
    }
}

impl<'a> ModuloLarge<'a> {
    /// Inverse.
    #[inline]
    fn inv(&self) -> Option<ModuloLarge<'a>> {
        unimplemented!()
    }
}
