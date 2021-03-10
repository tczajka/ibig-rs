//! Divide by a prearranged Word quickly using multiplication by the reciprocal.

use crate::primitive::{double_word, extend_word, split_double_word, DoubleWord, Word, WORD_BITS};

/// Divide a Word by a prearranged divisor.
///
/// Granlund, Montgomerry "Division by Invariant Integers using Multiplication"
/// Algorithm 4.1.
#[derive(Clone, Copy)]
pub(crate) struct FastDivideSmall {
    // 2..=Word::MAX
    divisor: Word,
    // ceil log_2 divisor - 1
    shift: u32,
    // m = floor (2^(N+shift) / divisor) - 2^N + 1
    m: Word,
}

impl FastDivideSmall {
    pub(crate) const fn new(divisor: Word) -> Self {
        // assert!(divisor > 1);
        // Asserts don't work in const functions.

        let len = WORD_BITS - (divisor - 1).leading_zeros();
        // 2^(len-1) < divisor <= 2^len
        //
        // Calculate:
        // m = floor(2^(N+len) / divisor) + 1 - 2^N
        //   = floor(2^N * (2^len - divisor) / divisor) + 1
        //   = floor(2^N * (2^len-1 - (divisor-1)) / divisor) + 1
        // m >= 2^N + 1 - 2^N > 0
        // m <= 2^(N+len) / (2^(len-1) + 1) + 1 - 2^N
        //    = (2^(N+len) - 2^(N+len-1) - 2^N + 2^(len-1) + 1) / (2^(len-1) + 1)
        //    < 2^(N+len-1) / 2^(len-1) = 2^N
        // So m fits in a Word.
        //
        // Note:
        // divisor * (2^N + m) = divisor * floor(2^(N+len) / divisor + 1)
        // = 2^(N+len) + (1 ..= 2^len)
        let (lo, _hi) = split_double_word(
            double_word(0, (Word::MAX >> (WORD_BITS - len)) - (divisor - 1)) / extend_word(divisor),
        );
        // assert!(_hi == 0);
        FastDivideSmall {
            divisor,
            shift: len - 1,
            m: lo + 1,
        }
    }

    /// ( n / divisor, n % divisor)
    pub(crate) fn div_rem(&self, n: Word) -> (Word, Word) {
        // q = floor( (m + 2^N) * n / 2^(N+shift) )
        //
        // Let divisor * (2^N + m) = 2^(N+shift) + k, 1 <= k <= 2^shift
        //
        // (m + 2^N) * n / 2^(N+shift) = ( (2^(N+shift)+k) / divisor ) * (n / 2^(N+shift))
        // = k * n / (divisor * 2^(N+shift)) + n / divisor
        // On the one hand, this is >= n / divisor
        // On the other hand, this is:
        // <= (2^shift * (2^N-1) / 2^(N+shift) + n) / divisor
        // = (n + 1 - 2^-N) / divisor < (n + 1) / divisor
        //
        // Therefore the floor is always exact q.

        // t = m * n / 2^N
        let (_, t) = split_double_word(extend_word(self.m) * extend_word(n));
        // q = (t + n) / 2^shift = (t + (n - t)/2) / 2^(shift-1)
        let q = (t + ((n - t) >> 1)) >> self.shift;
        let r = n - q * self.divisor;
        (q, r)
    }

    pub(crate) const fn dummy() -> Self {
        Self::new(2)
    }
}

/// Fast repeated division by a given normalized Word.
#[derive(Clone, Copy)]
pub(crate) struct FastDivideNormalized {
    divisor: Word,
    reciprocal: Word,
}

impl FastDivideNormalized {
    /// Initialize from a given normalized divisor.
    ///
    /// divisor must have top bit of 1
    pub(crate) const fn new(divisor: Word) -> Self {
        let (recip_lo, _) = split_double_word(DoubleWord::MAX / extend_word(divisor));

        FastDivideNormalized {
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
pub(crate) struct FastDivide {
    pub(crate) normalized: FastDivideNormalized,
    pub(crate) shift: u32,
}

impl FastDivide {
    /// Initialize from a given divisor.
    pub(crate) const fn new(divisor: Word) -> Self {
        let shift = divisor.leading_zeros();

        FastDivide {
            normalized: FastDivideNormalized::new(divisor << shift),
            shift,
        }
    }

    pub(crate) const fn dummy() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_fast_divide_small() {
        let mut rng = StdRng::seed_from_u64(1);
        for _ in 0..1000000 {
            let d_bits = rng.gen_range(2..=WORD_BITS);
            let d = rng.gen_range(Word::MAX / 2 + 1..=Word::MAX) >> (WORD_BITS - d_bits);
            let fast_div = FastDivideSmall::new(d);
            let n = rng.gen();
            let (q, r) = fast_div.div_rem(n);
            dbg!(d, n, q, r);
            assert_eq!(q, n / d);
            assert_eq!(r, n % d);
        }
    }

    #[test]
    fn test_fast_divide_normalized() {
        let mut rng = StdRng::seed_from_u64(1);
        for _ in 0..1000000 {
            let d = rng.gen_range(Word::MAX / 2 + 1..=Word::MAX);
            let q = rng.gen();
            let r = rng.gen_range(0..d);
            let a = extend_word(q) * extend_word(d) + extend_word(r);
            let fast_div = FastDivideNormalized::new(d);
            assert_eq!(fast_div.div_rem(a), (q, r));
        }
    }
}
