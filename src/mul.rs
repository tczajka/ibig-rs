use crate::{
    buffer::Buffer,
    ibig::IBig,
    primitive::{extend_word, split_double_word, Word},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Mul, MulAssign},
};

impl Mul<UBig> for UBig {
    type Output = UBig;

    #[inline]
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

    #[inline]
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

    #[inline]
    fn mul(self, rhs: UBig) -> UBig {
        rhs.mul(self)
    }
}

impl Mul<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
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
    #[inline]
    fn mul_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&UBig> for UBig {
    #[inline]
    fn mul_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) * rhs;
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
                let carry = mul_word_in_place(&mut buffer, a);
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
        let (a, b) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        UBig::mul_simple(a, b)
    }

    /// Simple multiplication algorithm.
    fn mul_simple(lhs: &[Word], rhs: &[Word]) -> UBig {
        debug_assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let mut buffer = Buffer::allocate(lhs.len() + rhs.len());
        buffer.push_zeros(rhs.len());
        for (i, m) in lhs.iter().enumerate() {
            let carry = add_mul_word_in_place_same_len(&mut buffer[i..], *m, rhs);
            buffer.push(carry);
        }
        buffer.into()
    }
}

/// Multiply a word sequence by a `Word` in place.
///
/// Returns carry.
fn mul_word_in_place(words: &mut [Word], rhs: Word) -> Word {
    let mut carry: Word = 0;
    for a in words {
        // a * b + carry <= MAX * MAX + MAX < DoubleWord::MAX
        let (v0, v1) = split_double_word(extend_word(*a) * extend_word(rhs) + extend_word(carry));
        *a = v0;
        carry = v1;
    }
    carry
}

/// words += mult * rhs
///
/// Returns carry.
fn add_mul_word_in_place_same_len(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    debug_assert!(words.len() == rhs.len());
    let mut carry: Word = 0;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        // a + mult * b + carry <= MAX * MAX + 2 * MAX <= DoubleWord::MAX
        let (v0, v1) = split_double_word(
            extend_word(*a) + extend_word(carry) + extend_word(mult) * extend_word(*b),
        );
        *a = v0;
        carry = v1;
    }
    carry
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

impl Mul<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: IBig) -> IBig {
        rhs.mul(self)
    }
}

impl Mul<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl MulAssign<IBig> for IBig {
    #[inline]
    fn mul_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&IBig> for IBig {
    #[inline]
    fn mul_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) * rhs;
    }
}
