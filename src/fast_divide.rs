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

/// Divide a DoubleWord by a prearranged divisor.
///
/// Assumes quotient fits in a Word.
///
/// Granlund, Montgomerry "Division by Invariant Integers using Multiplication"
/// Algorithm 8.1.
#[derive(Clone, Copy)]
pub(crate) struct FastDivideNormalized {
    // Top bit must be 1.
    divisor: Word,
    // floor ((2^2N - 1) / divisor) - 2^N
    m: Word,
}

impl FastDivideNormalized {
    /// Initialize from a given normalized divisor.
    ///
    /// divisor must have top bit of 1
    pub(crate) const fn new(divisor: Word) -> Self {
        // assert!(divisor.leading_zeros() == 0);
        // Asserts don't work in const functions.
        let (m, _hi) = split_double_word(DoubleWord::MAX / extend_word(divisor));
        // assert!(_hi == 1);

        // Note:
        // m > 0
        // (m + 2^N) * divisor < 2^2N
        // (m + 2^N) * divisor >= 2^2N - divisor

        FastDivideNormalized { divisor, m }
    }

    /// (n / divisor, n % divisor)
    /// The result must fit in a single word.
    pub(crate) fn div_rem(&self, n: DoubleWord) -> (Word, Word) {
        // Let d = divisor.
        // Let (2^N + m) * d = 2^2N - k, 0 < k <= d
        // Approximate 2^N * quotient is:
        // 2^N * n / d <= (2^N + m) * n / 2^N = n + n*m / 2^N
        //
        // Let [n_hi, n_lo] = n.
        // b = top bit of n_lo
        //
        // Calculate:
        // big_q = n + (n_hi + b) * m + b * (d - 2^N)
        //
        // big_q >= n + b * (d - 2^N) >= n - b * 2^(N-1) >= 0
        // big_q = n_hi * (m + 2^N) + n_lo + b * (m - 2^N + divisor)
        //       <= (d-1) * (m + 2^N) + (2^N-1) + (m - 2^N + 2^N - 1)
        //       = d * (m + 2^N) - 2 < 2^2N
        // So big_q fits in DoubleWord.
        //
        // Let big_q = (q_hi, q_lo).
        //
        // We will use q_hi as the quotient approximation.
        //
        // remainder = n - q_hi * d
        // = (n * 2^N - big_q * d + q_lo * d) / 2^N
        // = (n * 2^N - (n_hi * (m+2^N) + n_lo + b(m-2^N+d)) * d + q_lo * d) / 2^N
        // = (n * 2^N - ((n_hi+b) * (m+2^N) + n_lo + b(d - 2^(N+1))) * d + q_lo*d) / 2^N
        // = (n * 2^N - (n_hi+b) * (m+2^N)d - n_lo * d - b * d * (d - 2^(N+1)) + q_lo * d) / 2^N
        // = (n_hi * 2^2N + n_lo * 2^N - (n_hi+b) * (2^2N-k) - n_lo * d - b*d*(d-2^(N+1)) + q_lo*d)/2^N
        // = (n_lo * (2^N-d) + (n_hi+b)*k - b*2^2N - b*d*(d-2^(N+1)) + q_lo*d) / 2^N
        // = (n_lo * (2^N-d) + (n_hi+b)*k - b*(2^2N + d^2 - d*2^(N+1)) + q_lo*d) / 2^N
        // = (n_lo * (2^N-d) + (n_hi+b)*k - b*(2^N-d)^2 + q_lo*d) / 2^N
        // = ((n_hi+b)*k + q_lo*d)/2^N + (1-d*2^-N)(n_lo + b*(d-2^N))
        // = ((n_hi+b)*k + q_lo*d)/2^N + (1-d*2^-N)((n_lo-b*2^(N-1)) + b * (d-2^(N-1)))
        //
        // remainder >= 0
        //
        // On the other hand:
        // remainder <= ((d-1+1)*d + (2^N-1)*d)/2^N + (1-d*2^-N)(2^(N-1)-1 + (d-2^(N-1)))
        // < d^2/2^N + d + (1-d*2^-N)*d = 2*d
        //
        // Therefore the q_hi may only be 1 too small.

        let (n_lo, n_hi) = split_double_word(n);
        // debug_assert!(n_hi < d);
        let b = n_lo >> (WORD_BITS - 1);
        // Spread bit b on all bits.
        let b_spread = (0 as Word).wrapping_sub(b);

        // big_q = n + (n_hi + b) * m + b * (d - 2^N)
        // adjustment = n_lo + b * (d - 2^N) >= 0
        // q = q_hi is the high word of big_q
        let adjustment = n_lo.wrapping_add(b_spread & self.divisor);
        let (_, x_hi) = split_double_word(
            extend_word(n_hi + b) * extend_word(self.m) + extend_word(adjustment),
        );
        let q = n_hi + x_hi;

        // q2 = 2^N - (q+1)
        let q2 = Word::MAX - q;

        // rem = n - (q+1)*d = n - 2^N * d + q2 * d
        // We are keeping it modulo 2^2N.
        // If rem >= 0, then the result is (q+1, rem).
        // If rem < 0, then the result is (q, rem + d).
        let rem = n
            .wrapping_sub(double_word(0, self.divisor))
            .wrapping_add(extend_word(q2) * extend_word(self.divisor));

        let (rem_lo, rem_hi) = split_double_word(rem);
        // rem_hi is 0 or -1
        // quotient = q+1 + rem_hi = rem_hi - q2 (mod 2^N)
        let quotient = rem_hi.wrapping_sub(q2);
        // remainder = rem + d * (-rem_hi)
        let remainder = rem_lo.wrapping_add(self.divisor & rem_hi);

        (quotient, remainder)
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
