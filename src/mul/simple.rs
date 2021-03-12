//! Simple multiplication algorithm.

use crate::{
    add,
    arch::{self, SignedWord, Word},
    mul,
    sign::Sign::{self, *},
};

/// Split larger length into chunks of CHUNK_LEN..2 * CHUNK_LEN for memory locality.
const CHUNK_LEN: usize = 1024;

/// Max supported smaller factor length.
pub(crate) const MAX_SMALLER_LEN: usize = CHUNK_LEN;

/// c += sign * a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
pub(crate) fn add_signed_mul(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &[Word],
    b: &[Word],
) -> SignedWord {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_SMALLER_LEN);

    let n = b.len();
    let mut carry_n = 0; // at c[n]
    while a.len() >= 2 * CHUNK_LEN {
        // Propagate carry_n
        carry_n = add::add_signed_word_in_place(&mut c[n..CHUNK_LEN + n], carry_n);
        carry_n += add_signed_mul_chunk(&mut c[..CHUNK_LEN + n], sign, &a[..CHUNK_LEN], b);
        a = &a[CHUNK_LEN..];
        c = &mut c[CHUNK_LEN..];
    }
    debug_assert!(a.len() >= b.len() && a.len() < 2 * CHUNK_LEN);
    // Propagate carry_n
    let mut carry = add::add_signed_word_in_place(&mut c[n..], carry_n);
    carry += add_signed_mul_chunk(c, sign, a, b);
    carry
}

/// c += sign * a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_signed_mul_chunk(c: &mut [Word], sign: Sign, a: &[Word], b: &[Word]) -> SignedWord {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(a.len() < 2 * CHUNK_LEN);

    match sign {
        Positive => SignedWord::from(add_mul_chunk(c, a, b)),
        Negative => -SignedWord::from(sub_mul_chunk(c, a, b)),
    }
}

/// c += a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_mul_chunk(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(a.len() < 2 * CHUNK_LEN);
    let mut carry = false;
    for (i, m) in b.iter().enumerate() {
        let carry_word = mul::add_mul_word_same_len_in_place(&mut c[i..i + a.len()], *m, a);
        let (carry_word, carry_next) = arch::add_with_carry(c[i + a.len()], carry_word, carry);
        c[i + a.len()] = carry_word;
        carry = carry_next;
    }
    carry
}

/// c -= a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns borrow.
fn sub_mul_chunk(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(a.len() < 2 * CHUNK_LEN);
    let mut borrow = false;
    for (i, m) in b.iter().enumerate() {
        let borrow_word = mul::sub_mul_word_same_len_in_place(&mut c[i..i + a.len()], *m, a);
        let (borrow_word, borrow_next) = arch::sub_with_borrow(c[i + a.len()], borrow_word, borrow);
        c[i + a.len()] = borrow_word;
        borrow = borrow_next;
    }
    borrow
}
