//! Greatest Common Divisor
use crate::{
    arch::word::Word,
    primitive::{double_word, extend_word},
};

/// Single word gcd, requires a > 0 and b > 0
pub(crate) fn gcd_word_by_word(a: Word, b: Word) -> Word {
    debug_assert!(a > 0 && b > 0);

    // find common factors of 2
    let shift = (a | b).trailing_zeros();
    let mut a = a >> a.trailing_zeros();
    let mut b = b >> b.trailing_zeros();

    // the binary GCD algorithm
    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a << shift
}

// TODO: also implement gcd for double word
