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
    pub(crate) fn new(divisor: Word) -> FastDivisorNormalized {
        debug_assert!(divisor.leading_zeros() == 0);

        let (recip_lo, recip_hi) = split_double_word(DoubleWord::MAX / extend_word(divisor));
        debug_assert!(recip_hi == 1);

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
