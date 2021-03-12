//! Addition and subtraction operators.

use crate::{
    add,
    arch::Word,
    buffer::Buffer,
    ibig::IBig,
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
        UBig::from_ibig_after_sub(IBig::sub_ubig_val_val(self, rhs))
    }
}

impl Sub<&UBig> for UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_after_sub(IBig::sub_ubig_val_ref(self, rhs))
    }
}

impl Sub<UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: UBig) -> UBig {
        UBig::from_ibig_after_sub(-IBig::sub_ubig_val_ref(rhs, self))
    }
}

impl Sub<&UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_after_sub(IBig::sub_ubig_ref_ref(self, rhs))
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

    fn from_ibig_after_sub(x: IBig) -> UBig {
        match UBig::try_from(x) {
            Ok(v) => v,
            Err(_) => panic!("UBig subtraction overflow"),
        }
    }

    fn sub_large_word(mut lhs: Buffer, rhs: Word) -> UBig {
        let overflow = add::sub_word_in_place(&mut lhs, rhs);
        assert!(!overflow);
        lhs.into()
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
}
