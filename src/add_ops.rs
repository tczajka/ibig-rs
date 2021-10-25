//! Addition and subtraction operators.

use crate::{
    add,
    arch::word::Word,
    buffer::Buffer,
    ibig::IBig,
    primitive::{PrimitiveSigned, PrimitiveUnsigned},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    convert::TryFrom,
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

impl Add<UBig> for UBig {
    type Output = UBig;

    fn add(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0, &buffer1)
                } else {
                    UBig::add_large(buffer1, &buffer0)
                }
            }
        }
    }
}

impl Add<&UBig> for UBig {
    type Output = UBig;

    fn add(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::add_word(word0, *word1),
                Large(buffer1) => UBig::add_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::add_large_word(buffer0, *word1),
                Large(buffer1) => UBig::add_large(buffer0, buffer1),
            },
        }
    }
}

impl Add<UBig> for &UBig {
    type Output = UBig;

    fn add(self, rhs: UBig) -> UBig {
        rhs.add(self)
    }
}

impl Add<&UBig> for &UBig {
    type Output = UBig;

    fn add(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0.clone(), buffer1)
                } else {
                    UBig::add_large(buffer1.clone(), buffer0)
                }
            }
        }
    }
}

impl AddAssign<UBig> for UBig {
    fn add_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&UBig> for UBig {
    fn add_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl Sub<UBig> for UBig {
    type Output = UBig;

    fn sub(self, rhs: UBig) -> UBig {
        UBig::from_ibig_panic_on_overflow(IBig::sub_ubig_val_val(self, rhs))
    }
}

impl Sub<&UBig> for UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_panic_on_overflow(IBig::sub_ubig_val_ref(self, rhs))
    }
}

impl Sub<UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: UBig) -> UBig {
        UBig::from_ibig_panic_on_overflow(-IBig::sub_ubig_val_ref(rhs, self))
    }
}

impl Sub<&UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_panic_on_overflow(IBig::sub_ubig_ref_ref(self, rhs))
    }
}

impl SubAssign<UBig> for UBig {
    fn sub_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&UBig> for UBig {
    fn sub_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl Add<IBig> for IBig {
    type Output = IBig;

    fn add(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_val(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_val_val(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<&IBig> for IBig {
    type Output = IBig;

    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Positive) => -IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<IBig> for &IBig {
    type Output = IBig;

    fn add(self, rhs: IBig) -> IBig {
        rhs.add(self)
    }
}

impl Add<&IBig> for &IBig {
    type Output = IBig;

    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_ref_ref(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl AddAssign<IBig> for IBig {
    fn add_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&IBig> for IBig {
    fn add_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl Sub<IBig> for IBig {
    type Output = IBig;

    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for IBig {
    type Output = IBig;

    fn sub(self, rhs: &IBig) -> IBig {
        -(-self + rhs)
    }
}

impl Sub<IBig> for &IBig {
    type Output = IBig;

    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for &IBig {
    type Output = IBig;

    fn sub(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Positive, Negative) => IBig::from(mag0 + mag1),
            (Negative, Positive) => -IBig::from(mag0 + mag1),
            (Negative, Negative) => IBig::sub_ubig_ref_ref(mag1, mag0),
        }
    }
}

impl SubAssign<IBig> for IBig {
    fn sub_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&IBig> for IBig {
    fn sub_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) - rhs;
    }
}

macro_rules! impl_ubig_with_unsigned {
    ($t:ty) => {
        impl Add<$t> for UBig {
            type Output = UBig;

            fn add(self, rhs: $t) -> UBig {
                self.add_unsigned(rhs)
            }
        }

        impl Add<&$t> for UBig {
            type Output = UBig;

            fn add(self, rhs: &$t) -> UBig {
                self.add_unsigned(*rhs)
            }
        }

        impl Add<$t> for &UBig {
            type Output = UBig;

            fn add(self, rhs: $t) -> UBig {
                self.ref_add_unsigned(rhs)
            }
        }

        impl Add<&$t> for &UBig {
            type Output = UBig;

            fn add(self, rhs: &$t) -> UBig {
                self.ref_add_unsigned(*rhs)
            }
        }

        impl AddAssign<$t> for UBig {
            fn add_assign(&mut self, rhs: $t) {
                self.add_assign_unsigned(rhs)
            }
        }

        impl AddAssign<&$t> for UBig {
            fn add_assign(&mut self, rhs: &$t) {
                self.add_assign_unsigned(*rhs)
            }
        }

        impl Add<UBig> for $t {
            type Output = UBig;

            fn add(self, rhs: UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<&UBig> for $t {
            type Output = UBig;

            fn add(self, rhs: &UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<UBig> for &$t {
            type Output = UBig;

            fn add(self, rhs: UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<&UBig> for &$t {
            type Output = UBig;

            fn add(self, rhs: &UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Sub<$t> for UBig {
            type Output = UBig;

            fn sub(self, rhs: $t) -> UBig {
                self.sub_unsigned(rhs)
            }
        }

        impl Sub<&$t> for UBig {
            type Output = UBig;

            fn sub(self, rhs: &$t) -> UBig {
                self.sub_unsigned(*rhs)
            }
        }

        impl Sub<$t> for &UBig {
            type Output = UBig;

            fn sub(self, rhs: $t) -> UBig {
                self.ref_sub_unsigned(rhs)
            }
        }

        impl Sub<&$t> for &UBig {
            type Output = UBig;

            fn sub(self, rhs: &$t) -> UBig {
                self.ref_sub_unsigned(*rhs)
            }
        }

        impl SubAssign<$t> for UBig {
            fn sub_assign(&mut self, rhs: $t) {
                self.sub_assign_unsigned(rhs)
            }
        }

        impl SubAssign<&$t> for UBig {
            fn sub_assign(&mut self, rhs: &$t) {
                self.sub_assign_unsigned(*rhs)
            }
        }
    };
}

impl_ubig_with_unsigned!(u8);
impl_ubig_with_unsigned!(u16);
impl_ubig_with_unsigned!(u32);
impl_ubig_with_unsigned!(u64);
impl_ubig_with_unsigned!(u128);
impl_ubig_with_unsigned!(usize);

macro_rules! impl_ubig_with_signed {
    ($t:ty) => {
        impl Add<$t> for UBig {
            type Output = UBig;

            fn add(self, rhs: $t) -> UBig {
                self.add_signed(rhs)
            }
        }

        impl Add<&$t> for UBig {
            type Output = UBig;

            fn add(self, rhs: &$t) -> UBig {
                self.add_signed(*rhs)
            }
        }

        impl Add<$t> for &UBig {
            type Output = UBig;

            fn add(self, rhs: $t) -> UBig {
                self.ref_add_signed(rhs)
            }
        }

        impl Add<&$t> for &UBig {
            type Output = UBig;

            fn add(self, rhs: &$t) -> UBig {
                self.ref_add_signed(*rhs)
            }
        }

        impl AddAssign<$t> for UBig {
            fn add_assign(&mut self, rhs: $t) {
                self.add_assign_signed(rhs)
            }
        }

        impl AddAssign<&$t> for UBig {
            fn add_assign(&mut self, rhs: &$t) {
                self.add_assign_signed(*rhs)
            }
        }

        impl Add<UBig> for $t {
            type Output = UBig;

            fn add(self, rhs: UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<&UBig> for $t {
            type Output = UBig;

            fn add(self, rhs: &UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<UBig> for &$t {
            type Output = UBig;

            fn add(self, rhs: UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Add<&UBig> for &$t {
            type Output = UBig;

            fn add(self, rhs: &UBig) -> UBig {
                rhs.add(self)
            }
        }

        impl Sub<$t> for UBig {
            type Output = UBig;

            fn sub(self, rhs: $t) -> UBig {
                self.sub_signed(rhs)
            }
        }

        impl Sub<&$t> for UBig {
            type Output = UBig;

            fn sub(self, rhs: &$t) -> UBig {
                self.sub_signed(*rhs)
            }
        }

        impl Sub<$t> for &UBig {
            type Output = UBig;

            fn sub(self, rhs: $t) -> UBig {
                self.ref_sub_signed(rhs)
            }
        }

        impl Sub<&$t> for &UBig {
            type Output = UBig;

            fn sub(self, rhs: &$t) -> UBig {
                self.ref_sub_signed(*rhs)
            }
        }

        impl SubAssign<$t> for UBig {
            fn sub_assign(&mut self, rhs: $t) {
                self.sub_assign_signed(rhs)
            }
        }

        impl SubAssign<&$t> for UBig {
            fn sub_assign(&mut self, rhs: &$t) {
                self.sub_assign_signed(*rhs)
            }
        }
    };
}

impl_ubig_with_signed!(i8);
impl_ubig_with_signed!(i16);
impl_ubig_with_signed!(i32);
impl_ubig_with_signed!(i64);
impl_ubig_with_signed!(i128);
impl_ubig_with_signed!(isize);

macro_rules! impl_ibig_with_primitive {
    ($t:ty) => {
        impl Add<$t> for IBig {
            type Output = IBig;

            fn add(self, rhs: $t) -> IBig {
                self.add_primitive(rhs)
            }
        }

        impl Add<&$t> for IBig {
            type Output = IBig;

            fn add(self, rhs: &$t) -> IBig {
                self.add_primitive(*rhs)
            }
        }

        impl Add<$t> for &IBig {
            type Output = IBig;

            fn add(self, rhs: $t) -> IBig {
                self.ref_add_primitive(rhs)
            }
        }

        impl Add<&$t> for &IBig {
            type Output = IBig;

            fn add(self, rhs: &$t) -> IBig {
                self.ref_add_primitive(*rhs)
            }
        }

        impl AddAssign<$t> for IBig {
            fn add_assign(&mut self, rhs: $t) {
                self.add_assign_primitive(rhs)
            }
        }

        impl AddAssign<&$t> for IBig {
            fn add_assign(&mut self, rhs: &$t) {
                self.add_assign_primitive(*rhs)
            }
        }

        impl Add<IBig> for $t {
            type Output = IBig;

            fn add(self, rhs: IBig) -> IBig {
                rhs.add(self)
            }
        }

        impl Add<&IBig> for $t {
            type Output = IBig;

            fn add(self, rhs: &IBig) -> IBig {
                rhs.add(self)
            }
        }

        impl Add<IBig> for &$t {
            type Output = IBig;

            fn add(self, rhs: IBig) -> IBig {
                rhs.add(self)
            }
        }

        impl Add<&IBig> for &$t {
            type Output = IBig;

            fn add(self, rhs: &IBig) -> IBig {
                rhs.add(self)
            }
        }

        impl Sub<$t> for IBig {
            type Output = IBig;

            fn sub(self, rhs: $t) -> IBig {
                self.sub_primitive(rhs)
            }
        }

        impl Sub<&$t> for IBig {
            type Output = IBig;

            fn sub(self, rhs: &$t) -> IBig {
                self.sub_primitive(*rhs)
            }
        }

        impl Sub<$t> for &IBig {
            type Output = IBig;

            fn sub(self, rhs: $t) -> IBig {
                self.ref_sub_primitive(rhs)
            }
        }

        impl Sub<&$t> for &IBig {
            type Output = IBig;

            fn sub(self, rhs: &$t) -> IBig {
                self.ref_sub_primitive(*rhs)
            }
        }

        impl SubAssign<$t> for IBig {
            fn sub_assign(&mut self, rhs: $t) {
                self.sub_assign_primitive(rhs)
            }
        }

        impl SubAssign<&$t> for IBig {
            fn sub_assign(&mut self, rhs: &$t) {
                self.sub_assign_primitive(*rhs)
            }
        }

        impl Sub<IBig> for $t {
            type Output = IBig;

            fn sub(self, rhs: IBig) -> IBig {
                rhs.sub_from_primitive(self)
            }
        }

        impl Sub<&IBig> for $t {
            type Output = IBig;

            fn sub(self, rhs: &IBig) -> IBig {
                rhs.ref_sub_from_primitive(self)
            }
        }

        impl Sub<IBig> for &$t {
            type Output = IBig;

            fn sub(self, rhs: IBig) -> IBig {
                rhs.sub_from_primitive(*self)
            }
        }

        impl Sub<&IBig> for &$t {
            type Output = IBig;

            fn sub(self, rhs: &IBig) -> IBig {
                rhs.ref_sub_from_primitive(*self)
            }
        }
    };
}

impl_ibig_with_primitive!(u8);
impl_ibig_with_primitive!(u16);
impl_ibig_with_primitive!(u32);
impl_ibig_with_primitive!(u64);
impl_ibig_with_primitive!(u128);
impl_ibig_with_primitive!(usize);
impl_ibig_with_primitive!(i8);
impl_ibig_with_primitive!(i16);
impl_ibig_with_primitive!(i32);
impl_ibig_with_primitive!(i64);
impl_ibig_with_primitive!(i128);
impl_ibig_with_primitive!(isize);

impl UBig {
    /// Add two `Word`s.
    fn add_word(a: Word, b: Word) -> UBig {
        let (res, overflow) = a.overflowing_add(b);
        if overflow {
            let mut buffer = Buffer::allocate(2);
            buffer.push(res);
            buffer.push(1);
            buffer.into()
        } else {
            UBig::from_word(res)
        }
    }

    /// Add a large number to a `Word`.
    fn add_large_word(mut buffer: Buffer, rhs: Word) -> UBig {
        debug_assert!(buffer.len() >= 2);
        if add::add_word_in_place(&mut buffer, rhs) {
            buffer.push_may_reallocate(1);
        }
        buffer.into()
    }

    /// Add two large numbers.
    fn add_large(mut buffer: Buffer, rhs: &[Word]) -> UBig {
        let n = buffer.len().min(rhs.len());
        let overflow = add::add_same_len_in_place(&mut buffer[..n], &rhs[..n]);
        if rhs.len() > n {
            buffer.ensure_capacity(rhs.len());
            buffer.extend(&rhs[n..]);
        }
        if overflow && add::add_one_in_place(&mut buffer[n..]) {
            buffer.push_may_reallocate(1);
        }
        buffer.into()
    }

    fn from_ibig_panic_on_overflow(x: IBig) -> UBig {
        match UBig::try_from(x) {
            Ok(v) => v,
            Err(_) => panic!("UBig overflow"),
        }
    }

    fn sub_large_word(mut lhs: Buffer, rhs: Word) -> UBig {
        let overflow = add::sub_word_in_place(&mut lhs, rhs);
        assert!(!overflow);
        lhs.into()
    }

    fn add_unsigned<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveUnsigned,
    {
        self + UBig::from_unsigned(rhs)
    }

    fn ref_add_unsigned<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveUnsigned,
    {
        self + UBig::from_unsigned(rhs)
    }

    fn add_assign_unsigned<T>(&mut self, rhs: T)
    where
        T: PrimitiveUnsigned,
    {
        *self += UBig::from_unsigned(rhs)
    }

    fn sub_unsigned<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveUnsigned,
    {
        self - UBig::from_unsigned(rhs)
    }

    fn ref_sub_unsigned<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveUnsigned,
    {
        self - UBig::from_unsigned(rhs)
    }

    fn sub_assign_unsigned<T>(&mut self, rhs: T)
    where
        T: PrimitiveUnsigned,
    {
        *self -= UBig::from_unsigned(rhs)
    }

    fn add_signed<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        UBig::from_ibig_panic_on_overflow(IBig::from(self) + IBig::from_signed(rhs))
    }

    fn ref_add_signed<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        UBig::from_ibig_panic_on_overflow(IBig::from(self) + IBig::from_signed(rhs))
    }

    fn add_assign_signed<T>(&mut self, rhs: T)
    where
        T: PrimitiveSigned,
    {
        *self = mem::take(self).add_signed(rhs)
    }

    fn sub_signed<T>(self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        UBig::from_ibig_panic_on_overflow(IBig::from(self) - IBig::from_signed(rhs))
    }

    fn ref_sub_signed<T>(&self, rhs: T) -> UBig
    where
        T: PrimitiveSigned,
    {
        UBig::from_ibig_panic_on_overflow(IBig::from(self) - IBig::from_signed(rhs))
    }

    fn sub_assign_signed<T>(&mut self, rhs: T)
    where
        T: PrimitiveSigned,
    {
        *self = mem::take(self).sub_signed(rhs)
    }
}

impl IBig {
    fn sub_ubig_val_val(lhs: UBig, rhs: UBig) -> IBig {
        match (lhs.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => IBig::sub_word_word(word0, word1),
            (Small(word0), Large(buffer1)) => -IBig::sub_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => IBig::sub_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    IBig::sub_large(buffer0, &buffer1)
                } else {
                    -IBig::sub_large(buffer1, &buffer0)
                }
            }
        }
    }

    fn sub_ubig_val_ref(lhs: UBig, rhs: &UBig) -> IBig {
        match lhs.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => IBig::sub_word_word(word0, *word1),
                Large(buffer1) => -IBig::sub_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => IBig::sub_large_word(buffer0, *word1),
                Large(buffer1) => IBig::sub_large(buffer0, buffer1),
            },
        }
    }

    fn sub_ubig_ref_ref(lhs: &UBig, rhs: &UBig) -> IBig {
        match (lhs.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => IBig::sub_word_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => -IBig::sub_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => IBig::sub_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    IBig::sub_large(buffer0.clone(), buffer1)
                } else {
                    -IBig::sub_large(buffer1.clone(), buffer0)
                }
            }
        }
    }

    fn sub_word_word(lhs: Word, rhs: Word) -> IBig {
        if lhs >= rhs {
            IBig::from(lhs - rhs)
        } else {
            -IBig::from(rhs - lhs)
        }
    }

    fn sub_large_word(lhs: Buffer, rhs: Word) -> IBig {
        UBig::sub_large_word(lhs, rhs).into()
    }

    fn sub_large(mut lhs: Buffer, rhs: &[Word]) -> IBig {
        if lhs.len() >= rhs.len() {
            let sign = add::sub_in_place_with_sign(&mut lhs, rhs);
            IBig::from_sign_magnitude(sign, lhs.into())
        } else {
            let n = lhs.len();
            let borrow = add::sub_same_len_in_place_swap(&rhs[..n], &mut lhs);
            lhs.ensure_capacity(rhs.len());
            lhs.extend(&rhs[n..]);
            if borrow {
                let overflow = add::sub_one_in_place(&mut lhs[n..]);
                assert!(!overflow);
            }
            IBig::from_sign_magnitude(Negative, lhs.into())
        }
    }

    fn add_primitive<T>(self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        self + IBig::from(rhs)
    }

    fn ref_add_primitive<T>(&self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        self + IBig::from(rhs)
    }

    fn add_assign_primitive<T>(&mut self, rhs: T)
    where
        IBig: From<T>,
    {
        *self += IBig::from(rhs)
    }

    fn sub_primitive<T>(self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        self - IBig::from(rhs)
    }

    fn ref_sub_primitive<T>(&self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        self - IBig::from(rhs)
    }

    fn sub_assign_primitive<T>(&mut self, rhs: T)
    where
        IBig: From<T>,
    {
        *self -= IBig::from(rhs)
    }

    fn sub_from_primitive<T>(self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        IBig::from(rhs) - self
    }

    fn ref_sub_from_primitive<T>(&self, rhs: T) -> IBig
    where
        IBig: From<T>,
    {
        IBig::from(rhs) - self
    }
}
