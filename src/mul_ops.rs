//! Multiplication operators.

use crate::{
    buffer::Buffer,
    ibig::IBig,
    mul,
    primitive::{extend_word, Word},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Mul, MulAssign},
};

impl Mul<UBig> for UBig {
    type Output = UBig;

    fn mul(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(&buffer0, &buffer1),
        }
    }
}

impl Mul<&UBig> for UBig {
    type Output = UBig;

    fn mul(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::mul_word(word0, *word1),
                Large(buffer1) => UBig::mul_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::mul_large_word(buffer0, *word1),
                Large(buffer1) => UBig::mul_large(&buffer0, buffer1),
            },
        }
    }
}

impl Mul<UBig> for &UBig {
    type Output = UBig;

    fn mul(self, rhs: UBig) -> UBig {
        rhs.mul(self)
    }
}

impl Mul<&UBig> for &UBig {
    type Output = UBig;

    fn mul(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(buffer0, buffer1),
        }
    }
}

impl MulAssign<UBig> for UBig {
    fn mul_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&UBig> for UBig {
    fn mul_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl Mul<IBig> for IBig {
    type Output = IBig;

    fn mul(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<&IBig> for IBig {
    type Output = IBig;

    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<IBig> for &IBig {
    type Output = IBig;

    fn mul(self, rhs: IBig) -> IBig {
        rhs.mul(self)
    }
}

impl Mul<&IBig> for &IBig {
    type Output = IBig;

    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl MulAssign<IBig> for IBig {
    fn mul_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&IBig> for IBig {
    fn mul_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    fn mul(self, rhs: Sign) -> Sign {
        match (self, rhs) {
            (Positive, Positive) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
            (Negative, Negative) => Positive,
        }
    }
}

impl MulAssign<Sign> for Sign {
    fn mul_assign(&mut self, rhs: Sign) {
        *self = *self * rhs;
    }
}

impl UBig {
    /// Multiply two `Word`s.
    fn mul_word(a: Word, b: Word) -> UBig {
        match a.checked_mul(b) {
            Some(c) => UBig::from_word(c),
            None => UBig::from(extend_word(a) * extend_word(b)),
        }
    }

    /// Multiply a large number by a `Word`.
    fn mul_large_word(mut buffer: Buffer, a: Word) -> UBig {
        match a {
            0 => UBig::from_word(0),
            1 => buffer.into(),
            _ => {
                let carry = mul::mul_word_in_place(&mut buffer, a);
                if carry != 0 {
                    buffer.push_may_reallocate(carry);
                }
                buffer.into()
            }
        }
    }

    /// Multiply two large numbers.
    fn mul_large(lhs: &[Word], rhs: &[Word]) -> UBig {
        debug_assert!(lhs.len() >= 2 && rhs.len() >= 2);

        let mut buffer = Buffer::allocate(lhs.len() + rhs.len());
        buffer.push_zeros(lhs.len() + rhs.len());

        let mut temp = mul::allocate_temp_mul_buffer(lhs.len().min(rhs.len()));
        //        mul::multiply(&mut buffer, lhs, rhs, &mut temp);
        mul::add_signed_mul(&mut buffer, Positive, lhs, rhs, &mut temp);
        buffer.into()
    }
}
