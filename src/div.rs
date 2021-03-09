//! Division functions.

use crate::{
    primitive::{double_word, extend_word, Word},
    shift,
};
use fast_divisor::FastDivisorNormalized;

mod divide_conquer;
mod fast_divisor;
mod simple;

/// If divisor or quotient is at most this length, use the simple division algorithm.
const MAX_LEN_SIMPLE: usize = 32;

/// words = words / rhs
///
/// rhs must be non-zero
///
/// Returns words % rhs.
pub(crate) fn div_by_word_in_place(words: &mut [Word], rhs: Word) -> Word {
    debug_assert!(rhs != 0);
    if words.is_empty() {
        return 0;
    }
    if rhs.is_power_of_two() {
        let sh = rhs.trailing_zeros();
        let rem = shift::shr_in_place(words, sh);
        return rem;
    }

    let shift = rhs.leading_zeros();
    let mut rem = shift::shl_in_place(words, shift);
    let fast_div_rhs_top = FastDivisorNormalized::new(rhs << shift);

    for word in words.iter_mut().rev() {
        let a = double_word(*word, rem);
        let (q, r) = fast_div_rhs_top.div_rem(a);
        *word = q;
        rem = r;
    }
    rem >> shift
}

/// words % rhs
pub(crate) fn rem_by_word(words: &[Word], rhs: Word) -> Word {
    debug_assert!(rhs != 0);
    if words.is_empty() {
        return 0;
    }
    if rhs.is_power_of_two() {
        return words[0] & (rhs - 1);
    }

    let shift = rhs.leading_zeros();
    let fast_div_rhs = FastDivisorNormalized::new(rhs << shift);

    let mut rem: Word = 0;
    for word in words.iter().rev() {
        let a = double_word(*word, rem);
        let (_, r) = fast_div_rhs.div_rem(a);
        rem = r;
    }
    let a = extend_word(rem) << shift;
    let (_, rem) = fast_div_rhs.div_rem(a);
    rem >> shift
}

/// Divide lhs by rhs, replacing the top words of lhs by the quotient and the
/// bottom words of lhs by the remainder.
///
/// rhs must have at least 2 words and the top bit must be 1.
///
/// lhs = [lhs / rhs, lhs % rhs]
///
/// Returns carry in the quotient. It is at most 1 because rhs is normalized.
pub(crate) fn div_rem_in_place(lhs: &mut [Word], rhs: &[Word]) -> bool {
    assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
    let fast_div_rhs_top = FastDivisorNormalized::new(*rhs.last().unwrap());

    if rhs.len() <= MAX_LEN_SIMPLE || lhs.len() - rhs.len() <= MAX_LEN_SIMPLE {
        simple::div_rem_in_place(lhs, rhs, fast_div_rhs_top)
    } else {
        divide_conquer::div_rem_in_place(lhs, rhs, fast_div_rhs_top)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_by_word_in_place_empty() {
        let mut a = [];
        let rem = div_by_word_in_place(&mut a, 7);
        assert_eq!(rem, 0);
    }

    #[test]
    fn test_rem_by_word_empty() {
        let a = [];
        let rem = rem_by_word(&a, 7);
        assert_eq!(rem, 0);
    }
}
