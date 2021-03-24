//! Helper functions for multiplication algorithms.

use crate::{
    add,
    arch::word::{SignedWord, Word},
    memory::Memory,
    mul,
    sign::Sign,
};

/// c += sign * a * b
///
/// Splits into multiplies of length b.len(), and one final short multiply.
///
/// Returns carry.
pub(crate) fn add_signed_mul_split_into_same_len<F>(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &[Word],
    b: &[Word],
    memory: &mut Memory,
    f_add_signed_mul_same_len: F,
) -> SignedWord
where
    F: Fn(&mut [Word], Sign, &[Word], &[Word], &mut Memory) -> SignedWord,
{
    let mut carry: SignedWord = 0;
    let n = b.len();
    let mut carry_n: SignedWord = 0; // at c[n]
    while a.len() >= n {
        let (a_lo, a_hi) = a.split_at(n);
        // Propagate carry.
        carry_n = add::add_signed_word_in_place(&mut c[n..2 * n], carry_n);
        carry_n += f_add_signed_mul_same_len(&mut c[..2 * n], sign, a_lo, b, memory);
        a = a_hi;
        c = &mut c[n..];
    }
    carry += add::add_signed_word_in_place(&mut c[n..], carry_n);
    carry += mul::add_signed_mul(c, sign, b, a, memory);
    debug_assert!(carry.abs() <= 1);
    carry
}

/// c += sign * a * b
///
/// Splits a into chunks of chunk_len, and up to 2 * chunk_len - 1 for the last one.
///
/// Returns carry.
pub(crate) fn add_signed_mul_split_into_chunks<F>(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &[Word],
    b: &[Word],
    chunk_len: usize,
    f_add_signed_mul_chunk: F,
) -> SignedWord
where
    F: Fn(&mut [Word], Sign, &[Word], &[Word]) -> SignedWord,
{
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= chunk_len);

    let n = b.len();
    let mut carry_n = 0; // at c[n]
    while a.len() >= 2 * chunk_len {
        let (a_lo, a_hi) = a.split_at(chunk_len);
        // Propagate carry_n
        carry_n = add::add_signed_word_in_place(&mut c[n..chunk_len + n], carry_n);
        carry_n += f_add_signed_mul_chunk(&mut c[..chunk_len + n], sign, a_lo, b);
        a = a_hi;
        c = &mut c[chunk_len..];
    }
    debug_assert!(a.len() >= b.len() && a.len() < 2 * chunk_len);
    // Propagate carry_n
    let mut carry = add::add_signed_word_in_place(&mut c[n..], carry_n);
    carry += f_add_signed_mul_chunk(c, sign, a, b);
    carry
}
