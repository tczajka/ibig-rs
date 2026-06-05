//! [`proptest`](::proptest) strategies for generating big integers.

use crate::repr::{DIGIT_BITS_USIZE, Digits};
use crate::{IBig, UBig};
use ::proptest::collection::vec;
use ::proptest::prelude::*;
use ibig_core::Digit;
use unative::proptest::unative_in_range;

/// A strategy that generates a [`UBig`] of at most `n` bits.
///
/// The number of bits `k` is chosen uniformly in `0..=n`, then a uniformly random `k`-bit value
/// is generated. This spreads the generated values across magnitudes.
pub fn ubig_up_to_bits(n: usize) -> impl Strategy<Value = UBig> {
    (0..=n).prop_flat_map(ubig_with_bits)
}

/// A strategy that generates a uniformly random `n`-bit [`UBig`].
fn ubig_with_bits(n: usize) -> BoxedStrategy<UBig> {
    if n == 0 {
        return Just(UBig::ZERO).boxed();
    }
    let num_digits = n.div_ceil(DIGIT_BITS_USIZE);
    // The most-significant digit holds the top `top_bits` bits; the rest are full random digits.
    let n_top = n - (num_digits - 1) * DIGIT_BITS_USIZE;
    let top_bit = Digit::from_u8(1) << (n_top - 1);
    (
        unative_in_range(Digit::ZERO..top_bit),
        // big-endian digits so shrinking does big steps first
        vec(any::<Digit>(), num_digits - 1),
    )
        .prop_map(move |(top, mut digits)| {
            digits.reverse(); // little-endian
            digits.push(top | top_bit);
            UBig::from_digits(Digits::from_vec(digits))
        })
        .boxed()
}

/// A strategy that generates an [`IBig`] of at most `n` two's complement bits.
///
/// The number of bits `k` is chosen uniformly in `1..=n`, then we produce a uniformly
/// random value that needs exactly `k` two's complement bits.
///
/// # Panics
///
/// Panics if `n` is 0.
pub fn ibig_up_to_bits(n: usize) -> impl Strategy<Value = IBig> {
    (1..=n).prop_flat_map(ibig_with_bits)
}

/// A strategy that generates a uniformly random `n`-bit two's complement [`IBig`].
fn ibig_with_bits(n: usize) -> BoxedStrategy<IBig> {
    assert_ne!(n, 0);
    // An `n`-bit two's complement value is an `(n - 1)`-bit magnitude (whose top bit is set, so
    // the bit below the sign is the opposite of a `0` sign bit); negating it bitwise flips both
    // the sign bit and the bit below it, giving the negative counterpart.
    (any::<bool>(), ubig_with_bits(n - 1))
        .prop_map(|(negative, magnitude)| {
            let value = IBig::from(magnitude);
            if negative { !value } else { value }
        })
        .boxed()
}
