use crate::{
    arch::word::Word,
    math,
    memory::{self, MemoryAllocation},
    modular::modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall},
    primitive::{double_word, split_double_word, WORD_BITS, WORD_BITS_USIZE},
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
        let mut bit = exp.bit_len() - 1;
        while bit != 0 {
            bit -= 1;
            val.square_in_place();
            if exp_words[bit / WORD_BITS_USIZE] & (1 << (bit % WORD_BITS_USIZE)) != 0 {
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

        let n = self.ring().normalized_modulus().len();
        let window_len = ModuloLarge::choose_pow_window_len(exp.bit_len());

        // Precomputed table of small odd powers up to 2^window_len, starting from self^3.
        let table_words = ((1usize << (window_len - 1)) - 1)
            .checked_mul(n)
            .unwrap_or_else(|| memory::panic_out_of_memory());

        let memory_requirement = memory::add_layout(
            memory::array_layout::<Word>(table_words),
            self.ring().mul_memory_requirement(),
        );
        let mut allocation = MemoryAllocation::new(memory_requirement);
        let mut memory = allocation.memory();
        let (table, mut memory) = memory.allocate_slice_fill::<Word>(table_words, 0);

        // val = self^2
        let mut val = self.clone();
        val.mul_in_place(self, &mut memory);

        // self^(2*i+1) = self^(2*i-1) * val
        for i in 1..(1 << (window_len - 1)) {
            let (prev, cur) = if i == 1 {
                (self.normalized_value(), &mut table[0..n])
            } else {
                let (prev, cur) = (&mut table[(i - 2) * n..i * n]).split_at_mut(n);
                (&*prev, cur)
            };
            cur.copy_from_slice(self.ring().mul_normalized_values(
                prev,
                val.normalized_value(),
                &mut memory,
            ));
        }

        let exp_words = exp.as_words();
        // We already have self^2 in val.
        // exp.bit_len() >= 2 because exp >= 2.
        let mut bit = exp.bit_len() - 2;

        loop {
            // val = self ^ exp[bit..] ignoring the lowest bit
            let word_idx = bit / WORD_BITS_USIZE;
            let bit_idx = (bit % WORD_BITS_USIZE) as u32;
            let cur_word = exp_words[word_idx];
            if cur_word & (1 << bit_idx) != 0 {
                let next_word = if word_idx == 0 {
                    0
                } else {
                    exp_words[word_idx - 1]
                };
                // Get a window of window_len bits, with top bit of 1.
                let (mut window, _) = split_double_word(
                    double_word(next_word, cur_word) >> (bit_idx + 1 + WORD_BITS - window_len),
                );
                window &= math::ones::<Word>(window_len);
                // Shift right to make the window odd.
                let num_bits = window_len - window.trailing_zeros();
                window >>= window_len - num_bits;
                // val := val^2^(num_bits-1)
                for _ in 0..num_bits - 1 {
                    val.square_in_place(&mut memory);
                }
                bit -= (num_bits as usize) - 1;
                // Now val = self ^ exp[bit..] ignoring the num_bits lowest bits.
                // val = val * self^window from precomputed table.
                debug_assert!(window & 1 == 1);
                let entry_idx = (window >> 1) as usize;
                let entry = if entry_idx == 0 {
                    self.normalized_value()
                } else {
                    &table[(entry_idx - 1) * n..entry_idx * n]
                };
                val.mul_normalized_value_in_place(entry, &mut memory);
            }
            // val = self ^ exp[bit..]
            if bit == 0 {
                break;
            }
            bit -= 1;
            val.square_in_place(&mut memory);
        }
        val
    }

    fn choose_pow_window_len(n: usize) -> u32 {
        // This won't overflow because cost(3) is already approximately usize::MAX / 4
        // and it can only grow by a factor of 2.
        let cost = |window_size| (1usize << (window_size - 1)) - 1 + n / (window_size + 1);
        let mut window_size = 1;
        let mut c = cost(window_size);
        while window_size < WORD_BITS_USIZE {
            let c2 = cost(window_size + 1);
            if c <= c2 {
                break;
            }
            window_size += 1;
            c = c2;
        }
        window_size as u32
    }
}
