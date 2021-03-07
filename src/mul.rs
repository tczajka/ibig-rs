use crate::{
    add,
    buffer::Buffer,
    primitive::{double_word, extend_word, split_double_word, SignedWord, Word},
    sign::Sign::{self, *},
};
use core::mem;
use static_assertions::const_assert;

/// If smaller length <= MAX_LEN_SIMPLE, simple multiplication can be used.
const MAX_LEN_SIMPLE: usize = 24;
const_assert!(MAX_LEN_SIMPLE <= simple::MAX_SMALLER_LEN);
const_assert!(MAX_LEN_SIMPLE + 1 >= karatsuba::MIN_LEN);

/// If smaller length <= this, Karatsuba multiplication can be used.
const MAX_LEN_KARATSUBA: usize = 192;
const_assert!(MAX_LEN_KARATSUBA + 1 >= toom_3::MIN_LEN);

mod karatsuba;
mod simple;
mod toom_3;

/// Multiply a word sequence by a `Word` in place.
///
/// Returns carry.
pub(crate) fn mul_word_in_place(words: &mut [Word], rhs: Word) -> Word {
    mul_word_in_place_with_carry(words, rhs, 0)
}

/// Multiply a word sequence by a `Word` in place with carry in.
///
/// Returns carry.
pub(crate) fn mul_word_in_place_with_carry(words: &mut [Word], rhs: Word, mut carry: Word) -> Word {
    for a in words {
        // a * b + carry <= MAX * MAX + MAX < DoubleWord::MAX
        let (v_lo, v_hi) =
            split_double_word(extend_word(*a) * extend_word(rhs) + extend_word(carry));
        *a = v_lo;
        carry = v_hi;
    }
    carry
}

/// words += mult * rhs
///
/// Returns carry.
fn add_mul_word_same_len_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() == rhs.len());
    let mut carry: Word = 0;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        // a + mult * b + carry <= MAX * MAX + 2 * MAX <= DoubleWord::MAX
        let (v_lo, v_hi) = split_double_word(
            extend_word(*a) + extend_word(carry) + extend_word(mult) * extend_word(*b),
        );
        *a = v_lo;
        carry = v_hi;
    }
    carry
}

/// words += mult * rhs
///
/// Returns carry.
fn add_mul_word_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() >= rhs.len());
    let n = rhs.len();
    let mut carry = add_mul_word_same_len_in_place(&mut words[..n], mult, rhs);
    if words.len() > n {
        carry = Word::from(add::add_word_in_place(&mut words[n..], carry));
    }
    carry
}

/// words -= mult * rhs
///
/// Returns borrow.
pub(crate) fn sub_mul_word_same_len_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() == rhs.len());
    // carry is in -Word::MAX..0
    // carry_plus_max = carry + Word::MAX
    let mut carry_plus_max = Word::MAX;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        // Compute val = a - mult * b + carry_plus_max - MAX + (MAX << BITS)
        // val >= 0 - MAX * MAX - MAX + MAX*(MAX+1) = 0
        // val <= MAX - 0 + MAX - MAX + (MAX<<BITS) = DoubleWord::MAX
        // This fits exactly in DoubleWord!
        // We have to be careful to calculate in the correct order to avoid overflow.
        let v = extend_word(*a)
            + extend_word(carry_plus_max)
            + (double_word(0, Word::MAX) - extend_word(Word::MAX))
            - extend_word(mult) * extend_word(*b);
        let (v_lo, v_hi) = split_double_word(v);
        *a = v_lo;
        carry_plus_max = v_hi;
    }
    Word::MAX - carry_plus_max
}

/// words -= mult * rhs
///
/// Returns borrow.
fn sub_mul_word_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() >= rhs.len());
    let n = rhs.len();
    let mut borrow = sub_mul_word_same_len_in_place(&mut words[..n], mult, rhs);
    if words.len() > n {
        borrow = Word::from(add::sub_word_in_place(&mut words[n..], borrow));
    }
    borrow
}

/// Temporary buffer required for multiplication.
/// n is the length of the smaller factor in words.
pub(crate) fn allocate_temp_mul_buffer(n: usize) -> Buffer {
    let temp_len = if n <= MAX_LEN_SIMPLE {
        0
    } else if n <= MAX_LEN_KARATSUBA {
        karatsuba::temp_buffer_len(n)
    } else {
        toom_3::temp_buffer_len(n).max(karatsuba::temp_buffer_len(MAX_LEN_KARATSUBA))
    };

    let mut buffer = Buffer::allocate_no_extra(temp_len);
    buffer.push_zeros(temp_len);
    buffer
}

/// c = a * b
fn multiply_same_len(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) {
    c.fill(0);
    let overflow = add_signed_mul_same_len(c, Positive, a, b, temp);
    assert!(overflow == 0);
}

/// c += sign * a * b
///
/// Returns carry.
pub(crate) fn add_signed_mul<'a>(
    c: &mut [Word],
    sign: Sign,
    mut a: &'a [Word],
    mut b: &'a [Word],
    temp: &mut [Word],
) -> SignedWord {
    debug_assert!(c.len() == a.len() + b.len());

    if a.len() < b.len() {
        mem::swap(&mut a, &mut b);
    }

    if b.len() <= MAX_LEN_SIMPLE {
        simple::add_signed_mul(c, sign, a, b)
    } else if b.len() <= MAX_LEN_KARATSUBA {
        karatsuba::add_signed_mul(c, sign, a, b, temp)
    } else {
        toom_3::add_signed_mul(c, sign, a, b, temp)
    }
}

/// c += sign * a * b
///
/// Returns carry.
fn add_signed_mul_same_len(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    temp: &mut [Word],
) -> SignedWord {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);

    if n <= MAX_LEN_SIMPLE {
        simple::add_signed_mul(c, sign, a, b)
    } else if n <= MAX_LEN_KARATSUBA {
        karatsuba::add_signed_mul_same_len(c, sign, a, b, temp)
    } else {
        toom_3::add_signed_mul_same_len(c, sign, a, b, temp)
    }
}

/// c += sign * a * b
///
/// Splits into multiplies of length b.len(), and one final short multiply.
///
/// Returns carry.
fn add_signed_mul_split_into_same_len<F>(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &[Word],
    b: &[Word],
    temp: &mut [Word],
    f_add_signed_mul_same_len: F,
) -> SignedWord
where
    F: Fn(&mut [Word], Sign, &[Word], &[Word], &mut [Word]) -> SignedWord,
{
    let mut carry: SignedWord = 0;
    let n = b.len();
    let mut carry_n: SignedWord = 0; // at c[n]
    while a.len() >= n {
        // Propagate carry.
        carry_n = add::add_signed_word_in_place(&mut c[n..2 * n], carry_n);
        let (a_lo, a_hi) = a.split_at(n);
        carry_n += f_add_signed_mul_same_len(&mut c[..2 * n], sign, a_lo, b, temp);
        a = a_hi;
        c = &mut c[n..];
    }
    carry += add::add_signed_word_in_place(&mut c[n..], carry_n);
    carry += add_signed_mul(c, sign, b, a, temp);
    debug_assert!(carry.abs() <= 1);
    carry
}
