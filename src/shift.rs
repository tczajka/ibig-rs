use crate::{
    buffer::Buffer,
    primitive::{Word, WORD_BITS},
    ubig::{Repr::*, UBig},
};
use core::{convert::TryInto, ops::Shl};

macro_rules! impl_shl_primitive_unsigned {
    (impl Shl<$a:ty> for $b:ty) => {
        impl Shl<$a> for $b {
            type Output = $b;

            #[inline]
            fn shl(self, rhs: $a) -> $b {
                self.shl_unsigned(rhs)
            }
        }

        impl Shl<&$a> for $b {
            type Output = $b;

            #[inline]
            fn shl(self, rhs: &$a) -> $b {
                self.shl_unsigned(*rhs)
            }
        }

        impl Shl<$a> for &$b {
            type Output = $b;

            #[inline]
            fn shl(self, rhs: $a) -> $b {
                self.shl_ref_unsigned(rhs)
            }
        }

        impl Shl<&$a> for &$b {
            type Output = $b;

            #[inline]
            fn shl(self, rhs: &$a) -> $b {
                self.shl_ref_unsigned(*rhs)
            }
        }
    };
}

impl_shl_primitive_unsigned!(impl Shl<u8> for UBig);
impl_shl_primitive_unsigned!(impl Shl<u16> for UBig);
impl_shl_primitive_unsigned!(impl Shl<u32> for UBig);
impl_shl_primitive_unsigned!(impl Shl<u64> for UBig);
impl_shl_primitive_unsigned!(impl Shl<u128> for UBig);
impl_shl_primitive_unsigned!(impl Shl<usize> for UBig);

impl Shl<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: UBig) -> UBig {
        self.shl_unsigned(rhs)
    }
}

impl Shl<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: &UBig) -> UBig {
        self.shl_unsigned(rhs)
    }
}

impl Shl<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: UBig) -> UBig {
        self.shl_ref_unsigned(rhs)
    }
}

impl Shl<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: &UBig) -> UBig {
        self.shl_ref_unsigned(rhs)
    }
}

impl UBig {
    /// Shift left by an unsigned type.
    fn shl_unsigned<T>(self, rhs: T) -> UBig
    where
        T: TryInto<usize>,
    {
        if self.is_zero() {
            self
        } else {
            match TryInto::<usize>::try_into(rhs) {
                Ok(rhs_usize) => self.shl_usize(rhs_usize),
                Err(_) => Buffer::too_large(),
            }
        }
    }

    /// Shift left reference by an unsigned type.
    fn shl_ref_unsigned<T>(&self, rhs: T) -> UBig
    where
        T: TryInto<usize>,
    {
        if self.is_zero() {
            UBig::from_word(0)
        } else {
            match TryInto::<usize>::try_into(rhs) {
                Ok(rhs_usize) => self.shl_ref_usize(rhs_usize),
                Err(_) => Buffer::too_large(),
            }
        }
    }

    /// Shift left by `usize` bits.
    fn shl_usize(self, rhs: usize) -> UBig {
        debug_assert!(!self.is_zero());

        match self.into_repr() {
            Small(word) => UBig::shl_small_usize(word, rhs),
            Large(buffer) => UBig::shl_large_usize(buffer, rhs),
        }
    }

    /// Shift left reference by `usize` bits.
    fn shl_ref_usize(&self, rhs: usize) -> UBig {
        debug_assert!(!self.is_zero());

        match self.repr() {
            Small(word) => UBig::shl_small_usize(*word, rhs),
            Large(buffer) => UBig::shl_large_ref_usize(buffer, rhs),
        }
    }

    /// Shift left one non-zero `Word` by `usize` bits.
    fn shl_small_usize(word: Word, rhs: usize) -> UBig {
        debug_assert!(word != 0);

        if rhs <= word.leading_zeros() as usize {
            UBig::from_word(word << rhs)
        } else {
            let shift_words = rhs / WORD_BITS as usize;
            let shift_bits = (rhs % WORD_BITS as usize) as u32;
            if shift_bits == 0 {
                UBig::shl_small_words(word, shift_words)
            } else {
                UBig::shl_small_words_bits(word, shift_words, shift_bits)
            }
        }
    }

    /// Shift left one non-zero `Word` by a positive number of words.
    fn shl_small_words(word: Word, shift_words: usize) -> UBig {
        debug_assert!(word != 0 && shift_words != 0);

        let mut buffer = Buffer::allocate(shift_words + 1);
        buffer.push_zeros(shift_words);
        buffer.push(word);
        buffer.into()
    }

    /// Shift left one non-zero `Word` by a number of bits non-divisible by `WORD_BITS`.
    fn shl_small_words_bits(word: Word, shift_words: usize, shift_bits: u32) -> UBig {
        debug_assert!(shift_bits > 0 && shift_bits < WORD_BITS);

        let mut buffer = Buffer::allocate(shift_words + 2);
        buffer.push_zeros(shift_words);
        buffer.push(word << shift_bits);
        buffer.push(word >> (WORD_BITS - shift_bits));
        buffer.into()
    }

    /// Shift left `buffer` by `rhs` bits.
    fn shl_large_usize(buffer: Buffer, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        let shift_bits = (rhs % WORD_BITS as usize) as u32;
        if shift_bits == 0 {
            UBig::shl_large_words(buffer, shift_words)
        } else {
            UBig::shl_large_words_bits(buffer, shift_words, shift_bits)
        }
    }

    /// Shift left `buffer` by full words.
    fn shl_large_words(mut buffer: Buffer, shift_words: usize) -> UBig {
        if buffer.len() + shift_words > buffer.capacity() {
            UBig::shl_large_ref_words(&buffer, shift_words)
        } else {
            buffer.push_zeros(shift_words);
            for i in (shift_words..buffer.len()).rev() {
                buffer[i] = buffer[i - shift_words];
            }
            for x in buffer[0..shift_words].iter_mut().rev() {
                *x = 0;
            }
            buffer.into()
        }
    }

    /// Shift left `buffer` by a number of bits non-divisible by `WORD_BITS`.
    fn shl_large_words_bits(mut buffer: Buffer, shift_words: usize, shift_bits: u32) -> UBig {
        debug_assert!(shift_bits > 0 && shift_bits < WORD_BITS);
        debug_assert!(buffer.len() >= 2);

        let old_len = buffer.len();
        let new_len = old_len + shift_words + 1;
        if new_len > buffer.capacity() {
            UBig::shl_large_ref_words_bits(&buffer, shift_words, shift_bits)
        } else {
            buffer.push_zeros(shift_words + 1);
            for i in (0..old_len).rev() {
                let word = buffer[i];
                buffer[i + shift_words + 1] |= word >> (WORD_BITS - shift_bits);
                buffer[i + shift_words] = word << shift_bits;
            }
            for x in buffer[0..shift_words].iter_mut().rev() {
                *x = 0;
            }
            buffer.into()
        }
    }

    /// Shift left large number of words by `rhs` bits.
    fn shl_large_ref_usize(words: &[Word], rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        let shift_bits = (rhs % WORD_BITS as usize) as u32;
        if shift_bits == 0 {
            UBig::shl_large_ref_words(words, shift_words)
        } else {
            UBig::shl_large_ref_words_bits(words, shift_words, shift_bits)
        }
    }

    /// Shift left `words` by full words.
    fn shl_large_ref_words(words: &[Word], shift_words: usize) -> UBig {
        let new_len = words.len() + shift_words;
        let mut buffer = Buffer::allocate(new_len);
        buffer.push_zeros(new_len);
        buffer[shift_words..].copy_from_slice(words);
        buffer.into()
    }

    /// Shift left `words` by a number of bits non-divisible by `WORD_BITS`.
    fn shl_large_ref_words_bits(words: &[Word], shift_words: usize, shift_bits: u32) -> UBig {
        debug_assert!(shift_bits > 0 && shift_bits < WORD_BITS);
        debug_assert!(words.len() >= 2);

        let new_len = words.len() + shift_words + 1;
        let mut buffer = Buffer::allocate(new_len);
        buffer.push_zeros(new_len);
        for (i, word) in words.iter().enumerate() {
            buffer[i + shift_words] |= word << shift_bits;
            buffer[i + shift_words + 1] = word >> (WORD_BITS - shift_bits);
        }
        buffer.into()
    }
}
