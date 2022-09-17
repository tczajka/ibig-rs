use crate::{
    arch::word::Word,
    ibig::IBig,
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
    ops::RemEuclid,
    ubig::UBig,
};
use core::convert::TryInto;

impl<'a> Modulo<'a> {
    /// Inverse.
    ///
    /// Returns `None` if there is no unique inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(10));
    /// assert_eq!(ring.from(7).inverse(), Some(ring.from(3)));
    /// assert_eq!(ring.from(2).inverse(), None);
    /// ```
    pub fn inverse(&self) -> Option<Modulo<'a>> {
        match self.repr() {
            ModuloRepr::Small(self_small) => self_small.inverse().map(Into::into),
            ModuloRepr::Large(self_large) => self_large.inverse().map(Into::into),
        }
    }
}

impl<'a> ModuloSmall<'a> {
    /// Inverse.
    fn inverse(&self) -> Option<ModuloSmall<'a>> {
        let a = self.residue();
        let b = self.ring().modulus();
        // TODO: Optimized `extended_gcd` for `Word`s.
        // x * a + _ * b == gcd
        let (gcd, x, _) = UBig::from(a).extended_gcd(&UBig::from(b));
        if gcd == UBig::from_word(1) {
            let res: Word = x.rem_euclid(IBig::from(b)).try_into().unwrap();
            Some(ModuloSmall::new(
                ModuloSmallRaw::from_word(res, self.ring()),
                self.ring(),
            ))
        } else {
            None
        }
    }
}

impl<'a> ModuloLarge<'a> {
    /// Inverse.
    fn inverse(&self) -> Option<ModuloLarge<'a>> {
        let a = self.residue();
        let b = self.ring().modulus();
        let (gcd, x, _) = a.extended_gcd(&b);
        if gcd == UBig::from_word(1) {
            // TODO: Get rid of redundant remainder computations.
            let res: UBig = x.rem_euclid(IBig::from(b)).try_into().unwrap();
            Some(ModuloLarge::from_ubig(res, self.ring()))
        } else {
            None
        }
    }
}
