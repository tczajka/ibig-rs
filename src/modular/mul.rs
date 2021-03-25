use crate::{
    arch::word::Word,
    div,
    memory::{self, MemoryAllocation},
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
    mul,
    primitive::extend_word,
    shift,
    sign::Sign::Positive,
};
use core::ops::{Mul, MulAssign};

impl<'a> Mul<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.mul(&rhs)
    }
}

impl<'a> Mul<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.mul_assign(rhs);
        self
    }
}

impl<'a> Mul<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        rhs.mul(self)
    }
}

impl<'a> Mul<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    fn mul(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().mul(rhs)
    }
}

impl<'a> MulAssign<Modulo<'a>> for Modulo<'a> {
    fn mul_assign(&mut self, rhs: Modulo<'a>) {
        self.mul_assign(&rhs)
    }
}

impl<'a> MulAssign<&Modulo<'a>> for Modulo<'a> {
    fn mul_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.mul_in_place(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.mul_in_place(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl<'a> ModuloSmall<'a> {
    /// self *= rhs
    fn mul_in_place(&mut self, rhs: &ModuloSmall<'a>) {
        self.check_same_ring(rhs);
        let ring = self.ring();
        let self_val = self.normalized_value() >> ring.shift();
        let rhs_val = rhs.normalized_value();
        let product = extend_word(self_val) * extend_word(rhs_val);
        let (_, product) = ring.fast_div().div_rem(product);
        self.set_normalized_value(product);
    }
}

impl<'a> ModuloLarge<'a> {
    /// self *= rhs
    fn mul_in_place(&mut self, rhs: &ModuloLarge<'a>) {
        self.check_same_ring(rhs);
        self.modify_normalized_value(|words, ring| {
            let modulus = ring.normalized_modulus();
            let n = modulus.len();

            let memory_requirement = memory::add_layout(
                memory::array_layout::<Word>(2 * n),
                memory::max_layout(
                    mul::memory_requirement_exact(n),
                    div::memory_requirement_exact(2 * n, n),
                ),
            );
            let mut allocation = MemoryAllocation::new(memory_requirement);
            let mut memory = allocation.memory();

            shift::shr_in_place(words, ring.shift());
            let (product, mut memory) = memory.allocate_slice_fill::<Word>(2 * n, 0);
            let overflow = mul::add_signed_mul_same_len(
                product,
                Positive,
                words,
                rhs.normalized_value(),
                &mut memory,
            );
            assert_eq!(overflow, 0);

            let _overflow =
                div::div_rem_in_place(product, modulus, *ring.fast_div_top(), &mut memory);
            words.copy_from_slice(&product[..n]);
        });
    }
}
