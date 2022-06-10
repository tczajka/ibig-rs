use crate::{
    add,
    arch::word::Word,
    gcd,
    memory::{self, MemoryAllocation},
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
    shift,
    sign::Sign,
};

impl<'a> Modulo<'a> {
    /// Multiplicative inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// // A Mersenne prime.
    /// let p = ubig!(2).pow(127) - ubig!(1);
    /// let ring = ModuloRing::new(&p);
    /// // Fermat's little theorem: a^(p-2) = a^-1 (mod p)
    /// let a = ring.from(123);
    /// let ainv = a.inv().unwrap();
    /// assert_eq!(ainv, a.pow(&(p - ubig!(2))));
    /// assert_eq!((a * ainv).residue(), ubig!(1));
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
            let (g, _, coeff) =
                gcd::xgcd_word_by_word(self.ring().modulus(), self.raw().residue(self.ring()), true);
            if g == 1 {
                let coeff = if coeff < 0 {
                    self.ring().modulus() - (-coeff) as u64
                } else {
                    coeff as u64
                };
                Some(ModuloSmall::new(
                    ModuloSmallRaw::from_word(coeff, self.ring()),
                    self.ring(),
                ))
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
        // copy modulus
        let normalized_modulus = self.ring().normalized_modulus();
        let mut buf_mod = normalized_modulus.to_vec();

        // calculate memory requirement with additional memory for results of the extended gcd
        let mem_requirement = memory::add_layout(
            gcd::memory_requirement_exact(normalized_modulus.len(), self.normalized_value().len()),
            // note that normalized_modulus.len() >= self.normalized_value().len()
            memory::array_layout::<Word>(2 * self.normalized_value().len()),
        );
        let mut allocation = MemoryAllocation::new(mem_requirement);
        let mut memory = allocation.memory();

        // copy residue
        let (val, mut memory) = memory.allocate_slice_copy(self.normalized_value());
        let (g, mut memory) = memory.allocate_slice_fill(val.len(), 0);

        // use extended GCD to find the inverse
        let (_, rhs_sign) = gcd::xgcd_in_place(&mut buf_mod, val, g, true, &mut memory);

        // check if the GCD result is 1 (considering the shift of normalization)
        if g[0] != 1 << self.ring().shift() || g[1..].iter().any(|v| v != &0) {
            return None;
        }

        // normalize and return
        assert!(shift::shl_in_place(&mut buf_mod, self.ring().shift()) == 0);
        if rhs_sign == Sign::Negative {
            assert!(!add::sub_same_len_in_place_swap(
                normalized_modulus,
                &mut buf_mod
            ));
        }
        Some(ModuloLarge::new(buf_mod, self.ring()))
    }
}
