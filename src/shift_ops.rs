//! Bit shift operators.

use crate::{
    buffer::Buffer,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, PrimitiveSigned, Word, WORD_BITS},
    shift,
    sign::{Sign::*, UnsignedAbs},
    ubig::{Repr::*, UBig},
};
use core::{
    convert::TryInto,
    mem,
    ops::{Shl, ShlAssign, Shr, ShrAssign},
};

macro_rules! impl_ubig_shl_primitive_unsigned {
    ($a:ty) => {
        impl Shl<$a> for UBig {
            type Output = UBig;

            fn shl(self, rhs: $a) -> UBig {
                self.shl_unsigned(rhs)
            }
        }

        impl Shl<&$a> for UBig {
            type Output = UBig;

            fn shl(self, rhs: &$a) -> UBig {
                self.shl_unsigned(*rhs)
            }
        }

        impl Shl<$a> for &UBig {
            type Output = UBig;

            fn shl(self, rhs: $a) -> UBig {
                self.shl_ref_unsigned(rhs)
            }
        }

        impl Shl<&$a> for &UBig {
            type Output = UBig;

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

    fn shl(self, rhs: UBig) -> UBig {
        self.shl_unsigned(rhs)
    }
}

impl Shl<&UBig> for UBig {
    type Output = UBig;

    fn shl(self, rhs: &UBig) -> UBig {
        self.shl_unsigned(rhs)
    }
}

impl Shl<UBig> for &UBig {
    type Output = UBig;

    fn shl(self, rhs: UBig) -> UBig {
        self.shl_ref_unsigned(rhs)
    }
}

impl Shl<&UBig> for &UBig {
    type Output = UBig;

    fn shl(self, rhs: &UBig) -> UBig {
        self.shl_ref_unsigned(rhs)
    }
}

macro_rules! impl_ubig_shl_primitive_signed {
    ($a:ty) => {
        impl Shl<$a> for UBig {
            type Output = UBig;

            fn shl(self, rhs: $a) -> UBig {
                self.shl_signed(rhs)
            }
        }

        impl Shl<&$a> for UBig {
            type Output = UBig;

            fn shl(self, rhs: &$a) -> UBig {
                self.shl_signed(*rhs)
            }
        }

        impl Shl<$a> for &UBig {
            type Output = UBig;

            fn shl(self, rhs: $a) -> UBig {
                self.shl_ref_signed(rhs)
            }
        }

        impl Shl<&$a> for &UBig {
            type Output = UBig;

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

    fn shl(self, rhs: IBig) -> UBig {
        self.shl(&rhs)
    }
}

impl Shl<&IBig> for UBig {
    type Output = UBig;

    fn shl(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shl(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

impl Shl<IBig> for &UBig {
    type Output = UBig;

    fn shl(self, rhs: IBig) -> UBig {
        self.shl(&rhs)
    }
}

impl Shl<&IBig> for &UBig {
    type Output = UBig;

    fn shl(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shl(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

macro_rules! impl_ibig_shl {
    ($a:ty) => {
        impl Shl<$a> for IBig {
            type Output = IBig;

            fn shl(self, rhs: $a) -> IBig {
                self.shl_impl(rhs)
            }
        }

        impl Shl<&$a> for IBig {
            type Output = IBig;

            fn shl(self, rhs: &$a) -> IBig {
                self.shl_impl(rhs)
            }
        }

        impl Shl<$a> for &IBig {
            type Output = IBig;

            fn shl(self, rhs: $a) -> IBig {
                self.shl_ref_impl(rhs)
            }
        }

        impl Shl<&$a> for &IBig {
            type Output = IBig;

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

macro_rules! impl_shl_assign {
    ($a:ty, $b:ty) => {
        impl ShlAssign<$b> for $a {
            fn shl_assign(&mut self, rhs: $b) {
                *self = mem::take(self) << rhs;
            }
        }

        impl ShlAssign<&$b> for $a {
            fn shl_assign(&mut self, rhs: &$b) {
                *self = mem::take(self) << rhs;
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

macro_rules! impl_ubig_shr_primitive_unsigned {
    ($a:ty) => {
        impl Shr<$a> for UBig {
            type Output = UBig;

            fn shr(self, rhs: $a) -> UBig {
                self.shr_unsigned(rhs)
            }
        }

        impl Shr<&$a> for UBig {
            type Output = UBig;

            fn shr(self, rhs: &$a) -> UBig {
                self.shr_unsigned(*rhs)
            }
        }

        impl Shr<$a> for &UBig {
            type Output = UBig;

            fn shr(self, rhs: $a) -> UBig {
                self.shr_ref_unsigned(rhs)
            }
        }

        impl Shr<&$a> for &UBig {
            type Output = UBig;

            fn shr(self, rhs: &$a) -> UBig {
                self.shr_ref_unsigned(*rhs)
            }
        }
    };
}

impl_ubig_shr_primitive_unsigned!(u8);
impl_ubig_shr_primitive_unsigned!(u16);
impl_ubig_shr_primitive_unsigned!(u32);
impl_ubig_shr_primitive_unsigned!(u64);
impl_ubig_shr_primitive_unsigned!(u128);
impl_ubig_shr_primitive_unsigned!(usize);

impl Shr<UBig> for UBig {
    type Output = UBig;

    fn shr(self, rhs: UBig) -> UBig {
        self.shr_unsigned(rhs)
    }
}

impl Shr<&UBig> for UBig {
    type Output = UBig;

    fn shr(self, rhs: &UBig) -> UBig {
        self.shr_unsigned(rhs)
    }
}

impl Shr<UBig> for &UBig {
    type Output = UBig;

    fn shr(self, rhs: UBig) -> UBig {
        self.shr_ref_unsigned(rhs)
    }
}

impl Shr<&UBig> for &UBig {
    type Output = UBig;

    fn shr(self, rhs: &UBig) -> UBig {
        self.shr_ref_unsigned(rhs)
    }
}

macro_rules! impl_ubig_shr_primitive_signed {
    ($a:ty) => {
        impl Shr<$a> for UBig {
            type Output = UBig;

            fn shr(self, rhs: $a) -> UBig {
                self.shr_signed(rhs)
            }
        }

        impl Shr<&$a> for UBig {
            type Output = UBig;

            fn shr(self, rhs: &$a) -> UBig {
                self.shr_signed(*rhs)
            }
        }

        impl Shr<$a> for &UBig {
            type Output = UBig;

            fn shr(self, rhs: $a) -> UBig {
                self.shr_ref_signed(rhs)
            }
        }

        impl Shr<&$a> for &UBig {
            type Output = UBig;

            fn shr(self, rhs: &$a) -> UBig {
                self.shr_ref_signed(*rhs)
            }
        }
    };
}

impl_ubig_shr_primitive_signed!(i8);
impl_ubig_shr_primitive_signed!(i16);
impl_ubig_shr_primitive_signed!(i32);
impl_ubig_shr_primitive_signed!(i64);
impl_ubig_shr_primitive_signed!(i128);
impl_ubig_shr_primitive_signed!(isize);

impl Shr<IBig> for UBig {
    type Output = UBig;

    fn shr(self, rhs: IBig) -> UBig {
        self.shr(&rhs)
    }
}

impl Shr<&IBig> for UBig {
    type Output = UBig;

    fn shr(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shr(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

impl Shr<IBig> for &UBig {
    type Output = UBig;

    fn shr(self, rhs: IBig) -> UBig {
        self.shr(&rhs)
    }
}

impl Shr<&IBig> for &UBig {
    type Output = UBig;

    fn shr(self, rhs: &IBig) -> UBig {
        match rhs.sign() {
            Positive => self.shr(rhs.magnitude()),
            Negative => panic_shift_negative(),
        }
    }
}

macro_rules! impl_ibig_shr {
    ($a:ty) => {
        impl Shr<$a> for IBig {
            type Output = IBig;

            fn shr(self, rhs: $a) -> IBig {
                self.shr_impl(rhs)
            }
        }

        impl Shr<&$a> for IBig {
            type Output = IBig;

            fn shr(self, rhs: &$a) -> IBig {
                self.shr_impl(rhs)
            }
        }

        impl Shr<$a> for &IBig {
            type Output = IBig;

            fn shr(self, rhs: $a) -> IBig {
                self.shr_ref_impl(rhs)
            }
        }

        impl Shr<&$a> for &IBig {
            type Output = IBig;

            fn shr(self, rhs: &$a) -> IBig {
                self.shr_ref_impl(rhs)
            }
        }
    };
}

impl_ibig_shr!(u8);
impl_ibig_shr!(u16);
impl_ibig_shr!(u32);
impl_ibig_shr!(u64);
impl_ibig_shr!(u128);
impl_ibig_shr!(usize);
impl_ibig_shr!(UBig);
impl_ibig_shr!(i8);
impl_ibig_shr!(i16);
impl_ibig_shr!(i32);
impl_ibig_shr!(i64);
impl_ibig_shr!(i128);
impl_ibig_shr!(isize);
impl_ibig_shr!(IBig);

macro_rules! impl_shr_assign {
    ($a:ty, $b:ty) => {
        impl ShrAssign<$b> for $a {
            fn shr_assign(&mut self, rhs: $b) {
                *self = mem::take(self) >> rhs;
            }
        }

        impl ShrAssign<&$b> for $a {
            fn shr_assign(&mut self, rhs: &$b) {
                *self = mem::take(self) >> rhs;
            }
        }
    };
}

impl_shr_assign!(UBig, u8);
impl_shr_assign!(UBig, u16);
impl_shr_assign!(UBig, u32);
impl_shr_assign!(UBig, u64);
impl_shr_assign!(UBig, u128);
impl_shr_assign!(UBig, usize);
impl_shr_assign!(UBig, UBig);
impl_shr_assign!(UBig, i8);
impl_shr_assign!(UBig, i16);
impl_shr_assign!(UBig, i32);
impl_shr_assign!(UBig, i64);
impl_shr_assign!(UBig, i128);
impl_shr_assign!(UBig, isize);
impl_shr_assign!(UBig, IBig);
impl_shr_assign!(IBig, u8);
impl_shr_assign!(IBig, u16);
impl_shr_assign!(IBig, u32);
impl_shr_assign!(IBig, u64);
impl_shr_assign!(IBig, u128);
impl_shr_assign!(IBig, usize);
impl_shr_assign!(IBig, UBig);
impl_shr_assign!(IBig, i8);
impl_shr_assign!(IBig, i16);
impl_shr_assign!(IBig, i32);
impl_shr_assign!(IBig, i64);
impl_shr_assign!(IBig, i128);
impl_shr_assign!(IBig, isize);
impl_shr_assign!(IBig, IBig);

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
            Small(word) => UBig::shl_word_usize(word, rhs),
            Large(buffer) => UBig::shl_large_usize(buffer, rhs),
        }
    }

    /// Shift left reference by `usize` bits.
    fn shl_ref_usize(&self, rhs: usize) -> UBig {
        debug_assert!(!self.is_zero());

        match self.repr() {
            Small(word) => UBig::shl_word_usize(*word, rhs),
            Large(buffer) => UBig::shl_large_ref_usize(buffer, rhs),
        }
    }

    /// Shift left one non-zero `Word` by `usize` bits.
    fn shl_word_usize(word: Word, rhs: usize) -> UBig {
        debug_assert!(word != 0);

        if rhs <= WORD_BITS as usize {
            UBig::from(extend_word(word) << rhs)
        } else {
            UBig::shl_word_usize_slow(word, rhs)
        }
    }

    /// Shift left one non-zero `Word` by `usize` bits.
    fn shl_word_usize_slow(word: Word, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        let shift_bits = (rhs % WORD_BITS as usize) as u32;
        let (lo, hi) = split_double_word(extend_word(word) << shift_bits);
        let mut buffer = Buffer::allocate(shift_words + 2);
        buffer.push_zeros(shift_words);
        buffer.push(lo);
        buffer.push(hi);
        buffer.into()
    }

    /// Shift left `buffer` by `rhs` bits.
    fn shl_large_usize(mut buffer: Buffer, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;

        if buffer.capacity() < buffer.len() + shift_words + 1 {
            return UBig::shl_large_ref_usize(&buffer, rhs);
        }

        let shift_bits = (rhs % WORD_BITS as usize) as u32;
        let carry = shift::shl_in_place(&mut buffer, shift_bits);
        buffer.push(carry);
        buffer.push_zeros_front(shift_words);
        buffer.into()
    }

    /// Shift left large number of words by `rhs` bits.
    fn shl_large_ref_usize(words: &[Word], rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        let shift_bits = (rhs % WORD_BITS as usize) as u32;

        let mut buffer = Buffer::allocate(shift_words + words.len() + 1);
        buffer.push_zeros(shift_words);
        buffer.extend(words);
        let carry = shift::shl_in_place(&mut buffer[shift_words..], shift_bits);
        buffer.push(carry);
        buffer.into()
    }

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

    /// Shift right by an unsigned type.
    fn shr_unsigned<T>(self, rhs: T) -> UBig
    where
        T: TryInto<usize>,
    {
        match TryInto::<usize>::try_into(rhs) {
            Ok(rhs_usize) => self.shr_usize(rhs_usize),
            Err(_) => UBig::from_word(0),
        }
    }

    /// Shift right reference by an unsigned type.
    fn shr_ref_unsigned<T>(&self, rhs: T) -> UBig
    where
        T: TryInto<usize>,
    {
        match TryInto::<usize>::try_into(rhs) {
            Ok(rhs_usize) => self.shr_ref_usize(rhs_usize),
            Err(_) => UBig::from_word(0),
        }
    }

    /// Shift right by `usize` bits.
    fn shr_usize(self, rhs: usize) -> UBig {
        match self.into_repr() {
            Small(word) => UBig::shr_word_usize(word, rhs),
            Large(buffer) => UBig::shr_large_usize(buffer, rhs),
        }
    }

    /// Shift right reference by `usize` bits.
    fn shr_ref_usize(&self, rhs: usize) -> UBig {
        match self.repr() {
            Small(word) => UBig::shr_word_usize(*word, rhs),
            Large(buffer) => UBig::shr_large_ref_usize(buffer, rhs),
        }
    }

    /// Shift right one `Word` by `usize` bits.
    fn shr_word_usize(word: Word, rhs: usize) -> UBig {
        let word = if rhs < (WORD_BITS as usize) {
            word >> rhs
        } else {
            0
        };
        UBig::from_word(word)
    }

    /// Shift right `buffer` by `rhs` bits.
    fn shr_large_usize(mut buffer: Buffer, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        if shift_words >= buffer.len() {
            return UBig::from_word(0);
        }
        let shift_bits = (rhs % WORD_BITS as usize) as u32;
        buffer.erase_front(shift_words);
        shift::shr_in_place(&mut buffer, shift_bits);
        buffer.into()
    }

    /// Shift right large number of words by `rhs` bits.
    fn shr_large_ref_usize(words: &[Word], rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS as usize;
        let shift_bits = (rhs % WORD_BITS as usize) as u32;

        let words = &words[shift_words.min(words.len())..];

        match words {
            [] => UBig::from_word(0),
            &[w] => UBig::from_word(w >> shift_bits),
            &[lo, hi] => UBig::from(double_word(lo, hi) >> shift_bits),
            _ => {
                let mut buffer = Buffer::allocate(words.len());
                buffer.extend(words);
                shift::shr_in_place(&mut buffer, shift_bits);
                buffer.into()
            }
        }
    }

    /// Shift right by a signed type.
    fn shr_signed<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        match rhs.to_sign_magnitude() {
            (Positive, mag) => self.shr_unsigned(mag),
            (Negative, _) => panic_shift_negative(),
        }
    }

    /// Shift right reference by a signed type.
    fn shr_ref_signed<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        match rhs.to_sign_magnitude() {
            (Positive, mag) => self.shr_ref_unsigned(mag),
            (Negative, _) => panic_shift_negative(),
        }
    }
}

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

    /// Shift right.
    fn shr_impl<T>(self, rhs: T) -> IBig
    where
        UBig: Shr<T, Output = UBig>,
    {
        match self.sign() {
            Positive => IBig::from(self.unsigned_abs() >> rhs),
            Negative => !IBig::from((!self).unsigned_abs() >> rhs),
        }
    }

    /// Shift reference right.
    fn shr_ref_impl<'a, T>(&'a self, rhs: T) -> IBig
    where
        UBig: Shr<T, Output = UBig>,
        &'a UBig: Shr<T, Output = UBig>,
    {
        match self.sign() {
            Positive => IBig::from(self.magnitude() >> rhs),
            Negative => !IBig::from((!self).unsigned_abs() >> rhs),
        }
    }
}

fn panic_shift_negative() -> ! {
    panic!("Shift by negative amount")
}
