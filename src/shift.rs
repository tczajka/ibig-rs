use crate::{
    buffer::Buffer,
    ibig::IBig,
    primitive::{PrimitiveSigned, Word, WORD_BITS},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    convert::TryInto,
    ops::{Shl, ShlAssign},
};

macro_rules! impl_ubig_shl_primitive_unsigned {
    ($a:ty) => {
        impl Shl<$a> for UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: $a) -> UBig {
                self.shl_unsigned(rhs)
            }
        }

        impl Shl<&$a> for UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: &$a) -> UBig {
                self.shl_unsigned(*rhs)
            }
        }

        impl Shl<$a> for &UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: $a) -> UBig {
                self.shl_ref_unsigned(rhs)
            }
        }

        impl Shl<&$a> for &UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: &$a) -> UBig {
                self.shl_ref_unsigned(*rhs)
            }
        }
    };
}

impl_ubig_shl_primitive_unsigned!(u8);
impl_ubig_shl_primitive_unsigned!(u16);
impl_ubig_shl_primitive_unsigned!(u32);
impl_ubig_shl_primitive_unsigned!(u64);
impl_ubig_shl_primitive_unsigned!(u128);
impl_ubig_shl_primitive_unsigned!(usize);

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
                Err(_) => Buffer::panic_too_large(),
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
                Err(_) => Buffer::panic_too_large(),
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
        if buffer.will_reallocate(buffer.len() + shift_words) {
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
        if buffer.will_reallocate(new_len) {
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

fn panic_shift_negative() -> ! {
    panic!("Shift by negative amount")
}

macro_rules! impl_ubig_shl_primitive_signed {
    ($a:ty) => {
        impl Shl<$a> for UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: $a) -> UBig {
                self.shl_signed(rhs)
            }
        }

        impl Shl<&$a> for UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: &$a) -> UBig {
                self.shl_signed(*rhs)
            }
        }

        impl Shl<$a> for &UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: $a) -> UBig {
                self.shl_ref_signed(rhs)
            }
        }

        impl Shl<&$a> for &UBig {
            type Output = UBig;

            #[inline]
            fn shl(self, rhs: &$a) -> UBig {
                self.shl_ref_signed(*rhs)
            }
        }
    };
}

impl_ubig_shl_primitive_signed!(i8);
impl_ubig_shl_primitive_signed!(i16);
impl_ubig_shl_primitive_signed!(i32);
impl_ubig_shl_primitive_signed!(i64);
impl_ubig_shl_primitive_signed!(i128);
impl_ubig_shl_primitive_signed!(isize);

impl Shl<IBig> for UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: IBig) -> UBig {
        self.shl(&rhs)
    }
}

impl Shl<&IBig> for UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shl(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

impl Shl<IBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: IBig) -> UBig {
        self.shl(&rhs)
    }
}

impl Shl<&IBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shl(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

impl UBig {
    /// Shift left by a signed type.
    fn shl_signed<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        match rhs.to_sign_magnitude() {
            (Positive, mag) => self.shl_unsigned(mag),
            (Negative, _) => panic_shift_negative(),
        }
    }

    /// Shift left reference by a signed type.
    fn shl_ref_signed<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        match rhs.to_sign_magnitude() {
            (Positive, mag) => self.shl_ref_unsigned(mag),
            (Negative, _) => panic_shift_negative(),
        }
    }
}

macro_rules! impl_ibig_shl {
    ($a:ty) => {
        impl Shl<$a> for IBig {
            type Output = IBig;

            #[inline]
            fn shl(self, rhs: $a) -> IBig {
                self.shl_impl(rhs)
            }
        }

        impl Shl<&$a> for IBig {
            type Output = IBig;

            #[inline]
            fn shl(self, rhs: &$a) -> IBig {
                self.shl_impl(rhs)
            }
        }

        impl Shl<$a> for &IBig {
            type Output = IBig;

            #[inline]
            fn shl(self, rhs: $a) -> IBig {
                self.shl_ref_impl(rhs)
            }
        }

        impl Shl<&$a> for &IBig {
            type Output = IBig;

            #[inline]
            fn shl(self, rhs: &$a) -> IBig {
                self.shl_ref_impl(rhs)
            }
        }
    };
}

impl_ibig_shl!(u8);
impl_ibig_shl!(u16);
impl_ibig_shl!(u32);
impl_ibig_shl!(u64);
impl_ibig_shl!(u128);
impl_ibig_shl!(usize);
impl_ibig_shl!(UBig);
impl_ibig_shl!(i8);
impl_ibig_shl!(i16);
impl_ibig_shl!(i32);
impl_ibig_shl!(i64);
impl_ibig_shl!(i128);
impl_ibig_shl!(isize);
impl_ibig_shl!(IBig);

impl IBig {
    /// Shift left.
    fn shl_impl<T>(self, rhs: T) -> IBig
    where
        UBig: Shl<T, Output = UBig>,
    {
        let (sign, mag) = self.into_sign_magnitude();
        IBig::from_sign_magnitude(sign, mag.shl(rhs))
    }

    /// Shift reference left.
    fn shl_ref_impl<'a, T>(&'a self, rhs: T) -> IBig
    where
        &'a UBig: Shl<T, Output = UBig>,
    {
        IBig::from_sign_magnitude(self.sign(), self.magnitude().shl(rhs))
    }
}

macro_rules! impl_shl_assign {
    ($a:ty, $b:ty) => {
        impl ShlAssign<$b> for $a {
            #[inline]
            fn shl_assign(&mut self, rhs: $b) {
                *self = core::mem::take(self) << rhs;
            }
        }

        impl ShlAssign<&$b> for $a {
            #[inline]
            fn shl_assign(&mut self, rhs: &$b) {
                *self = core::mem::take(self) << rhs;
            }
        }
    };
}

impl_shl_assign!(UBig, u8);
impl_shl_assign!(UBig, u16);
impl_shl_assign!(UBig, u32);
impl_shl_assign!(UBig, u64);
impl_shl_assign!(UBig, u128);
impl_shl_assign!(UBig, usize);
impl_shl_assign!(UBig, UBig);
impl_shl_assign!(UBig, i8);
impl_shl_assign!(UBig, i16);
impl_shl_assign!(UBig, i32);
impl_shl_assign!(UBig, i64);
impl_shl_assign!(UBig, i128);
impl_shl_assign!(UBig, isize);
impl_shl_assign!(UBig, IBig);
impl_shl_assign!(IBig, u8);
impl_shl_assign!(IBig, u16);
impl_shl_assign!(IBig, u32);
impl_shl_assign!(IBig, u64);
impl_shl_assign!(IBig, u128);
impl_shl_assign!(IBig, usize);
impl_shl_assign!(IBig, UBig);
impl_shl_assign!(IBig, i8);
impl_shl_assign!(IBig, i16);
impl_shl_assign!(IBig, i32);
impl_shl_assign!(IBig, i64);
impl_shl_assign!(IBig, i128);
impl_shl_assign!(IBig, isize);
impl_shl_assign!(IBig, IBig);
