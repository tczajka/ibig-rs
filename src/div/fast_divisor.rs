use crate::primitive::{extend_word, split_double_word, DoubleWord, Word};

/// Fast repeated division by a given normalized Word.
#[derive(Clone, Copy)]
pub(crate) struct FastDivisorNormalized {
    divisor: Word,
    reciprocal: Word,
}

impl FastDivisorNormalized {
    /// Initialize from a given normalized divisor.
    ///
    /// divisor must have top bit of 1
    pub(crate) const fn new(divisor: Word) -> FastDivisorNormalized {
        let (recip_lo, _) = split_double_word(DoubleWord::MAX / extend_word(divisor));

        FastDivisorNormalized {
            divisor,
            reciprocal: recip_lo,
        }
    }

    /// Divide a value.
    /// The result must fit in a single word.
    pub(crate) fn div_rem(&self, dividend: DoubleWord) -> (Word, Word) {
        let (_, dividend_hi) = split_double_word(dividend);
        // Approximate quotient: it may be too small by at most 3.
        // self.reciprocal + (1<<BITS) is approximately (1<<(2*BITS)) / self.divisor.
        let (_, mul_hi) =
            split_double_word(extend_word(self.reciprocal) * extend_word(dividend_hi));
        let mut quotient = mul_hi + dividend_hi;
        let mut remainder = dividend - extend_word(self.divisor) * extend_word(quotient);
        while remainder >= extend_word(self.divisor) {
            quotient += 1;
            remainder -= extend_word(self.divisor);
        }
        let (rem_lo, _) = split_double_word(remainder);
        (quotient, rem_lo)
    }
}

/// Fast repeated division by a given Word.
#[derive(Clone, Copy)]
pub(crate) struct FastDivisor {
    pub(crate) normalized: FastDivisorNormalized,
    pub(crate) shift: u32,
}

impl FastDivisor {
    /// Initialize from a given divisor.
    pub(crate) const fn new(divisor: Word) -> FastDivisor {
        let shift = divisor.leading_zeros();

        FastDivisor {
            normalized: FastDivisorNormalized::new(divisor << shift),
            shift,
        }
    }

    /// Divide a value.
    pub(crate) fn div_rem(&self, dividend: Word) -> (Word, Word) {
        let (q, r) = self.normalized.div_rem(extend_word(dividend) << self.shift);
        (q, r >> self.shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_fast_divisor() {
        let mut rng = StdRng::seed_from_u64(1);
        for _ in 0..1000000 {
            let a = rng.gen();
            let b = rng.gen_range(1..=Word::MAX);
            let fast_div = FastDivisor::new(b);
            let (q, r) = fast_div.div_rem(a);
            assert_eq!(q, a / b);
            assert_eq!(r, a % b);
        }
    }
}
