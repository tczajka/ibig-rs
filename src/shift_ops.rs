//! Bit shift operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, PrimitiveSigned, WORD_BITS_USIZE},
    shift,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    convert::TryInto,
    mem,
    ops::{Shl, ShlAssign, Shr, ShrAssign},
};

macro_rules! impl_shl_unsigned {
    ($t:ty, $a:ty) => {
        impl Shl<$a> for $t {
            type Output = $t;

            fn shl(self, rhs: $a) -> $t {
                self.shl_unsigned(rhs)
            }
        }

        impl Shl<&$a> for $t {
            type Output = $t;

            fn shl(self, rhs: &$a) -> $t {
                self.shl_unsigned(*rhs)
            }
        }

        impl Shl<$a> for &$t {
            type Output = $t;

            fn shl(self, rhs: $a) -> $t {
                self.shl_ref_unsigned(rhs)
            }
        }

        impl Shl<&$a> for &$t {
            type Output = $t;

            fn shl(self, rhs: &$a) -> $t {
                self.shl_ref_unsigned(*rhs)
            }
        }
    };
}

macro_rules! impl_shl_all_unsigned {
    ($t: ty) => {
        impl_shl_unsigned!($t, u8);
        impl_shl_unsigned!($t, u16);
        impl_shl_unsigned!($t, u32);
        impl_shl_unsigned!($t, u64);
        impl_shl_unsigned!($t, u128);
        impl_shl_unsigned!($t, usize);
    };
}

macro_rules! impl_shl_ubig {
    ($t:ty) => {
        impl Shl<UBig> for $t {
            type Output = $t;

            fn shl(self, rhs: UBig) -> $t {
                self.shl_unsigned(&rhs)
            }
        }

        impl Shl<&UBig> for $t {
            type Output = $t;

            fn shl(self, rhs: &UBig) -> $t {
                self.shl_unsigned(rhs)
            }
        }

        impl Shl<UBig> for &$t {
            type Output = $t;

            fn shl(self, rhs: UBig) -> $t {
                self.shl_ref_unsigned(&rhs)
            }
        }

        impl Shl<&UBig> for &$t {
            type Output = $t;

            fn shl(self, rhs: &UBig) -> $t {
                self.shl_ref_unsigned(rhs)
            }
        }
    };
}

macro_rules! impl_shl_signed {
    ($t:ty, $a:ty) => {
        impl Shl<$a> for $t {
            type Output = $t;

            fn shl(self, rhs: $a) -> $t {
                self.shl_signed(rhs)
            }
        }

        impl Shl<&$a> for $t {
            type Output = $t;

            fn shl(self, rhs: &$a) -> $t {
                self.shl_signed(*rhs)
            }
        }

        impl Shl<$a> for &$t {
            type Output = $t;

            fn shl(self, rhs: $a) -> $t {
                self.shl_ref_signed(rhs)
            }
        }

        impl Shl<&$a> for &$t {
            type Output = $t;

            fn shl(self, rhs: &$a) -> $t {
                self.shl_ref_signed(*rhs)
            }
        }
    };
}

macro_rules! impl_shl_all_signed {
    ($t: ty) => {
        impl_shl_signed!($t, i8);
        impl_shl_signed!($t, i16);
        impl_shl_signed!($t, i32);
        impl_shl_signed!($t, i64);
        impl_shl_signed!($t, i128);
        impl_shl_signed!($t, isize);

        impl $t {
            /// Shift left by a signed type.
            fn shl_signed<T: PrimitiveSigned>(self, rhs: T) -> $t {
                match rhs.to_sign_magnitude() {
                    (Positive, mag) => self.shl_unsigned(mag),
                    (Negative, _) => panic_shift_negative(),
                }
            }

            /// Shift reference left by a signed type.
            fn shl_ref_signed<T: PrimitiveSigned>(&self, rhs: T) -> $t {
                match rhs.to_sign_magnitude() {
                    (Positive, mag) => self.shl_ref_unsigned(mag),
                    (Negative, _) => panic_shift_negative(),
                }
            }
        }
    };
}

macro_rules! impl_shl_ibig {
    ($t:ty) => {
        impl Shl<IBig> for $t {
            type Output = $t;

            fn shl(self, rhs: IBig) -> $t {
                self.shl_ibig(&rhs)
            }
        }

        impl Shl<&IBig> for $t {
            type Output = $t;

            fn shl(self, rhs: &IBig) -> $t {
                self.shl_ibig(rhs)
            }
        }

        impl Shl<IBig> for &$t {
            type Output = $t;

            fn shl(self, rhs: IBig) -> $t {
                self.shl_ref_ibig(&rhs)
            }
        }

        impl Shl<&IBig> for &$t {
            type Output = $t;

            fn shl(self, rhs: &IBig) -> $t {
                self.shl_ref_ibig(rhs)
            }
        }

        impl $t {
            /// Shift left by IBig.
            fn shl_ibig(self, rhs: &IBig) -> $t {
                match rhs.sign() {
                    Positive => self.shl_unsigned(rhs.magnitude()),
                    Negative => panic_shift_negative(),
                }
            }

            /// Shift reference left by IBig.
            fn shl_ref_ibig(&self, rhs: &IBig) -> $t {
                match rhs.sign() {
                    Positive => self.shl_ref_unsigned(rhs.magnitude()),
                    Negative => panic_shift_negative(),
                }
            }
        }
    };
}

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

macro_rules! impl_shl {
    ($t:ty) => {
        impl_shl_all_unsigned!($t);
        impl_shl_ubig!($t);
        impl_shl_all_signed!($t);
        impl_shl_ibig!($t);

        impl_shl_assign!($t, u8);
        impl_shl_assign!($t, u16);
        impl_shl_assign!($t, u32);
        impl_shl_assign!($t, u64);
        impl_shl_assign!($t, u128);
        impl_shl_assign!($t, usize);
        impl_shl_assign!($t, UBig);
        impl_shl_assign!($t, i8);
        impl_shl_assign!($t, i16);
        impl_shl_assign!($t, i32);
        impl_shl_assign!($t, i64);
        impl_shl_assign!($t, i128);
        impl_shl_assign!($t, isize);
        impl_shl_assign!($t, IBig);
    };
}

impl_shl!(UBig);
impl_shl!(IBig);

macro_rules! impl_shr_unsigned {
    ($t:ty, $a:ty) => {
        impl Shr<$a> for $t {
            type Output = $t;

            fn shr(self, rhs: $a) -> $t {
                self.shr_unsigned(rhs)
            }
        }

        impl Shr<&$a> for $t {
            type Output = $t;

            fn shr(self, rhs: &$a) -> $t {
                self.shr_unsigned(*rhs)
            }
        }

        impl Shr<$a> for &$t {
            type Output = $t;

            fn shr(self, rhs: $a) -> $t {
                self.shr_ref_unsigned(rhs)
            }
        }

        impl Shr<&$a> for &$t {
            type Output = $t;

            fn shr(self, rhs: &$a) -> $t {
                self.shr_ref_unsigned(*rhs)
            }
        }
    };
}

macro_rules! impl_shr_all_unsigned {
    ($t: ty) => {
        impl_shr_unsigned!($t, u8);
        impl_shr_unsigned!($t, u16);
        impl_shr_unsigned!($t, u32);
        impl_shr_unsigned!($t, u64);
        impl_shr_unsigned!($t, u128);
        impl_shr_unsigned!($t, usize);
    };
}

macro_rules! impl_shr_ubig {
    ($t:ty) => {
        impl Shr<UBig> for $t {
            type Output = $t;

            fn shr(self, rhs: UBig) -> $t {
                self.shr_unsigned(&rhs)
            }
        }

        impl Shr<&UBig> for $t {
            type Output = $t;

            fn shr(self, rhs: &UBig) -> $t {
                self.shr_unsigned(rhs)
            }
        }

        impl Shr<UBig> for &$t {
            type Output = $t;

            fn shr(self, rhs: UBig) -> $t {
                self.shr_ref_unsigned(&rhs)
            }
        }

        impl Shr<&UBig> for &$t {
            type Output = $t;

            fn shr(self, rhs: &UBig) -> $t {
                self.shr_ref_unsigned(rhs)
            }
        }
    };
}

macro_rules! impl_shr_signed {
    ($t:ty, $a:ty) => {
        impl Shr<$a> for $t {
            type Output = $t;

            fn shr(self, rhs: $a) -> $t {
                self.shr_signed(rhs)
            }
        }

        impl Shr<&$a> for $t {
            type Output = $t;

            fn shr(self, rhs: &$a) -> $t {
                self.shr_signed(*rhs)
            }
        }

        impl Shr<$a> for &$t {
            type Output = $t;

            fn shr(self, rhs: $a) -> $t {
                self.shr_ref_signed(rhs)
            }
        }

        impl Shr<&$a> for &$t {
            type Output = $t;

            fn shr(self, rhs: &$a) -> $t {
                self.shr_ref_signed(*rhs)
            }
        }
    };
}

macro_rules! impl_shr_all_signed {
    ($t: ty) => {
        impl_shr_signed!($t, i8);
        impl_shr_signed!($t, i16);
        impl_shr_signed!($t, i32);
        impl_shr_signed!($t, i64);
        impl_shr_signed!($t, i128);
        impl_shr_signed!($t, isize);

        impl $t {
            /// Shift right by a signed type.
            fn shr_signed<T: PrimitiveSigned>(self, rhs: T) -> $t {
                match rhs.to_sign_magnitude() {
                    (Positive, mag) => self.shr_unsigned(mag),
                    (Negative, _) => panic_shift_negative(),
                }
            }

            /// Shift reference right by a signed type.
            fn shr_ref_signed<T: PrimitiveSigned>(&self, rhs: T) -> $t {
                match rhs.to_sign_magnitude() {
                    (Positive, mag) => self.shr_ref_unsigned(mag),
                    (Negative, _) => panic_shift_negative(),
                }
            }
        }
    };
}

macro_rules! impl_shr_ibig {
    ($t:ty) => {
        impl Shr<IBig> for $t {
            type Output = $t;

            fn shr(self, rhs: IBig) -> $t {
                self.shr_ibig(&rhs)
            }
        }

        impl Shr<&IBig> for $t {
            type Output = $t;

            fn shr(self, rhs: &IBig) -> $t {
                self.shr_ibig(rhs)
            }
        }

        impl Shr<IBig> for &$t {
            type Output = $t;

            fn shr(self, rhs: IBig) -> $t {
                self.shr_ref_ibig(&rhs)
            }
        }

        impl Shr<&IBig> for &$t {
            type Output = $t;

            fn shr(self, rhs: &IBig) -> $t {
                self.shr_ref_ibig(rhs)
            }
        }

        impl $t {
            /// Shift right by IBig.
            fn shr_ibig(self, rhs: &IBig) -> $t {
                match rhs.sign() {
                    Positive => self.shr_unsigned(rhs.magnitude()),
                    Negative => panic_shift_negative(),
                }
            }

            /// Shift reference right by IBig.
            fn shr_ref_ibig(&self, rhs: &IBig) -> $t {
                match rhs.sign() {
                    Positive => self.shr_ref_unsigned(rhs.magnitude()),
                    Negative => panic_shift_negative(),
                }
            }
        }
    };
}

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

macro_rules! impl_shr {
    ($t:ty) => {
        impl_shr_all_unsigned!($t);
        impl_shr_ubig!($t);
        impl_shr_all_signed!($t);
        impl_shr_ibig!($t);

        impl_shr_assign!($t, u8);
        impl_shr_assign!($t, u16);
        impl_shr_assign!($t, u32);
        impl_shr_assign!($t, u64);
        impl_shr_assign!($t, u128);
        impl_shr_assign!($t, usize);
        impl_shr_assign!($t, UBig);
        impl_shr_assign!($t, i8);
        impl_shr_assign!($t, i16);
        impl_shr_assign!($t, i32);
        impl_shr_assign!($t, i64);
        impl_shr_assign!($t, i128);
        impl_shr_assign!($t, isize);
        impl_shr_assign!($t, IBig);
    };
}

impl_shr!(UBig);
impl_shr!(IBig);

impl UBig {
    /// Shift left by an unsigned type.
    fn shl_unsigned<T: TryInto<usize>>(self, rhs: T) -> UBig {
        if self == UBig::from_word(0) {
            self
        } else {
            let rhs_usize = rhs.try_into().unwrap_or_else(|_| Buffer::panic_too_large());
            self.shl_usize(rhs_usize)
        }
    }

    /// Shift reference left by an unsigned type.
    fn shl_ref_unsigned<T: TryInto<usize>>(&self, rhs: T) -> UBig {
        if *self == UBig::from_word(0) {
            UBig::from_word(0)
        } else {
            let rhs_usize = rhs.try_into().unwrap_or_else(|_| Buffer::panic_too_large());
            self.shl_ref_usize(rhs_usize)
        }
    }

    /// Shift left by `usize` bits.
    fn shl_usize(self, rhs: usize) -> UBig {
        debug_assert!(self != UBig::from_word(0));

        match self.into_repr() {
            Small(word) => UBig::shl_word_usize(word, rhs),
            Large(buffer) => UBig::shl_large_usize(buffer, rhs),
        }
    }

    /// Shift left reference by `usize` bits.
    fn shl_ref_usize(&self, rhs: usize) -> UBig {
        debug_assert!(*self != UBig::from_word(0));

        match self.repr() {
            Small(word) => UBig::shl_word_usize(*word, rhs),
            Large(buffer) => UBig::shl_ref_large_usize(buffer, rhs),
        }
    }

    /// Shift left one non-zero `Word` by `usize` bits.
    fn shl_word_usize(word: Word, rhs: usize) -> UBig {
        debug_assert!(word != 0);

        if rhs <= WORD_BITS_USIZE {
            UBig::from(extend_word(word) << rhs)
        } else {
            UBig::shl_word_usize_slow(word, rhs)
        }
    }

    /// Shift left one non-zero `Word` by `usize` bits.
    fn shl_word_usize_slow(word: Word, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;
        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;
        let (lo, hi) = split_double_word(extend_word(word) << shift_bits);
        let mut buffer = Buffer::allocate(shift_words + 2);
        buffer.push_zeros(shift_words);
        buffer.push(lo);
        buffer.push(hi);
        buffer.into()
    }

    /// Shift left `buffer` by `rhs` bits.
    fn shl_large_usize(mut buffer: Buffer, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;

        if buffer.capacity() < buffer.len() + shift_words + 1 {
            return UBig::shl_ref_large_usize(&buffer, rhs);
        }

        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;
        let carry = shift::shl_in_place(&mut buffer, shift_bits);
        buffer.push(carry);
        buffer.push_zeros_front(shift_words);
        buffer.into()
    }

    /// Shift left large number of words by `rhs` bits.
    fn shl_ref_large_usize(words: &[Word], rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;
        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;

        let mut buffer = Buffer::allocate(shift_words + words.len() + 1);
        buffer.push_zeros(shift_words);
        buffer.extend(words);
        let carry = shift::shl_in_place(&mut buffer[shift_words..], shift_bits);
        buffer.push(carry);
        buffer.into()
    }

    /// Shift right by an unsigned type.
    fn shr_unsigned<T: TryInto<usize>>(self, rhs: T) -> UBig {
        match rhs.try_into() {
            Ok(rhs_usize) => self.shr_usize(rhs_usize),
            Err(_) => UBig::from_word(0),
        }
    }

    /// Shift reference right by an unsigned type.
    fn shr_ref_unsigned<T: TryInto<usize>>(&self, rhs: T) -> UBig {
        match rhs.try_into() {
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
        let word = if rhs < WORD_BITS_USIZE {
            word >> rhs
        } else {
            0
        };
        UBig::from_word(word)
    }

    /// Shift right `buffer` by `rhs` bits.
    fn shr_large_usize(mut buffer: Buffer, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;
        if shift_words >= buffer.len() {
            return UBig::from_word(0);
        }
        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;
        buffer.erase_front(shift_words);
        shift::shr_in_place(&mut buffer, shift_bits);
        buffer.into()
    }

    /// Shift right large number of words by `rhs` bits.
    fn shr_large_ref_usize(words: &[Word], rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;
        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;

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
}

impl IBig {
    /// Shift left by an unsigned type.
    fn shl_unsigned<T: TryInto<usize>>(self, rhs: T) -> IBig {
        let (sign, mag) = self.into_sign_magnitude();
        IBig::from_sign_magnitude(sign, mag.shl_unsigned(rhs))
    }

    /// Shift reference left by an unsigned type.
    fn shl_ref_unsigned<T: TryInto<usize>>(&self, rhs: T) -> IBig {
        let (sign, mag) = (self.sign(), self.magnitude());
        IBig::from_sign_magnitude(sign, mag.shl_ref_unsigned(rhs))
    }

    /// Shift right by an unsigned type
    fn shr_unsigned<T: TryInto<usize>>(self, rhs: T) -> IBig {
        match rhs.try_into() {
            Ok(rhs_usize) => self.shr_usize(rhs_usize),
            Err(_) => match self.sign() {
                Positive => IBig::from(0u8),
                Negative => IBig::from(-1i8),
            },
        }
    }

    /// Shift reference right by an unsigned type
    fn shr_ref_unsigned<T: TryInto<usize>>(&self, rhs: T) -> IBig {
        match rhs.try_into() {
            Ok(rhs_usize) => self.shr_ref_usize(rhs_usize),
            Err(_) => match self.sign() {
                Positive => IBig::from(0u8),
                Negative => IBig::from(-1i8),
            },
        }
    }

    /// Shift right by usize.
    fn shr_usize(self, rhs: usize) -> IBig {
        let (sign, mag) = self.into_sign_magnitude();
        match sign {
            Positive => IBig::from(mag.shr_usize(rhs)),
            Negative => {
                let b = mag.are_low_bits_nonzero(rhs);
                -IBig::from(mag.shr_usize(rhs)) - IBig::from(b)
            }
        }
    }

    /// Shift reference right by usize.
    fn shr_ref_usize(&self, rhs: usize) -> IBig {
        let (sign, mag) = (self.sign(), self.magnitude());
        match sign {
            Positive => IBig::from(mag.shr_ref_usize(rhs)),
            Negative => {
                let b = mag.are_low_bits_nonzero(rhs);
                -IBig::from(mag.shr_ref_usize(rhs)) - IBig::from(b)
            }
        }
    }
}

fn panic_shift_negative() -> ! {
    panic!("Shift by negative amount")
}
