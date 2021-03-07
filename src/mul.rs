use crate::{
    add,
    buffer::Buffer,
    div,
    primitive::{double_word, extend_word, split_double_word, SignedWord, Word},
    shift,
    sign::Sign::{self, *},
};
use core::mem;
use static_assertions::const_assert;

/// If smaller length <= MAX_LEN_SIMPLE, simple multiplication can be used.
const MAX_LEN_SIMPLE: usize = 24;

/// Split larger length into chunks of MUL_SIMPLE_CHUNK..2 * MUL_SIMPLE_CHUNK for memory
/// locality.
const MUL_SIMPLE_CHUNK: usize = 1024;

/// If smaller length <= this, Karatsuba multiplication can be used.
const MAX_LEN_KARATSUBA: usize = 192;

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
        // Karatsuba multiplication.
        //
        // We prove by induction that f(n) <= 2n + 2 log_2 (n-1).
        //
        // Base case: f(2) = 0.
        // For n > 2:
        // f(n) = 2ceil(n/2) + f(ceil(n/2))
        //      <= n+1 + n+1 + 2log ((n+1)/2-1)
        //       = 2n + 2log (n-1)
        //
        // Use 2n + ceil log_2 n.
        2 * n + 2 * (n.next_power_of_two().trailing_zeros() as usize)
    } else {
        // We prove by induction that f(n) <= 4n + 20(log_3 (n-2.5)).
        // Base case, f(3)=0, OK.
        // For n > 3:
        // f(n)  = 8(ceil(n/3)+1) + f(ceil(n/3)+1)
        //      <= 8*(n+5)/3 + 4*(n+5)/3 + 20 log_3 ((n+5)/3-2.5)
        //       = 4n + 20 + 20 log_3 ((n+5)/3-2.5)
        //       = 4n + 20 log_3 (n-2.5)
        //
        // 20 log_3 (n-2.5) <= 20 log_3 n = 20 log_2 n / log_2 3 < 13 log_2 n
        // So we use 4n + 13 ceil log_2 n.
        //
        // Note: this has to be at least as much as Karatsuba (for recursive calls).
        4 * n + 13 * (n.next_power_of_two().trailing_zeros() as usize)
    };

    let mut buffer = Buffer::allocate_no_extra(temp_len);
    buffer.push_zeros(temp_len);
    buffer
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
        add_signed_mul_simple(c, sign, a, b)
    } else if b.len() <= MAX_LEN_KARATSUBA {
        add_signed_mul_karatsuba(c, sign, a, b, temp)
    } else {
        add_signed_mul_toom_3(c, sign, a, b, temp)
    }
}

/// c += sign * a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_signed_mul_simple(mut c: &mut [Word], sign: Sign, mut a: &[Word], b: &[Word]) -> SignedWord {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_LEN_SIMPLE);
    const_assert!(MUL_SIMPLE_CHUNK >= MAX_LEN_SIMPLE);

    let n = b.len();
    let mut carry_n = 0; // at c[n]
    while a.len() >= 2 * MUL_SIMPLE_CHUNK {
        // Propagate carry_n
        carry_n = add::add_signed_word_in_place(&mut c[n..MUL_SIMPLE_CHUNK + n], carry_n);
        carry_n += add_signed_mul_simple_chunk(
            &mut c[..MUL_SIMPLE_CHUNK + n],
            sign,
            &a[..MUL_SIMPLE_CHUNK],
            b,
        );
        a = &a[MUL_SIMPLE_CHUNK..];
        c = &mut c[MUL_SIMPLE_CHUNK..];
    }
    debug_assert!(a.len() >= b.len() && a.len() < 2 * MUL_SIMPLE_CHUNK);
    // Propagate carry_n
    let mut carry = add::add_signed_word_in_place(&mut c[n..], carry_n);
    carry += add_signed_mul_simple_chunk(c, sign, a, b);
    carry
}

/// c += sign * a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_signed_mul_simple_chunk(c: &mut [Word], sign: Sign, a: &[Word], b: &[Word]) -> SignedWord {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_LEN_SIMPLE);
    debug_assert!(a.len() < 2 * MUL_SIMPLE_CHUNK);

    match sign {
        Positive => SignedWord::from(add_mul_simple_chunk(c, a, b)),
        Negative => -SignedWord::from(sub_mul_simple_chunk(c, a, b)),
    }
}

/// c += a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_mul_simple_chunk(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_LEN_SIMPLE);
    debug_assert!(a.len() < 2 * MUL_SIMPLE_CHUNK);
    let mut carry: Word = 0;
    for (i, m) in b.iter().enumerate() {
        carry += add_mul_word_in_place(&mut c[i..], *m, a);
    }
    debug_assert!(carry <= 1);
    carry != 0
}

/// c -= a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns borrow.
fn sub_mul_simple_chunk(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_LEN_SIMPLE);
    debug_assert!(a.len() < 2 * MUL_SIMPLE_CHUNK);
    let mut borrow: Word = 0;
    for (i, m) in b.iter().enumerate() {
        borrow += sub_mul_word_in_place(&mut c[i..], *m, a);
    }
    debug_assert!(borrow <= 1);
    borrow != 0
}

/// c += sign * a * b
///
/// Splits into multiplies of equal length, and one final short multiply.
///
/// Returns carry.
fn add_signed_mul_split_into_same_len<'a, F1, F2>(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &'a [Word],
    mut b: &'a [Word],
    temp: &mut [Word],
    f_add_signed_mul_same_len: F1,
    f_add_signed_mul_short: F2,
    max_len_short: usize,
) -> SignedWord
where
    F1: Fn(&mut [Word], Sign, &[Word], &[Word], &mut [Word]) -> SignedWord,
    F2: FnOnce(&mut [Word], Sign, &[Word], &[Word], &mut [Word]) -> SignedWord,
{
    let mut carry: SignedWord = 0;
    while b.len() > max_len_short {
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
        mem::swap(&mut a, &mut b);
    }
    carry += f_add_signed_mul_short(c, sign, a, b, temp);
    debug_assert!(carry.abs() <= 1);
    carry
}

/// c += sign * a * b
/// Karatsuba method: O(a.len() * b.len()^0.59).
///
/// Returns carry.
fn add_signed_mul_karatsuba(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    temp: &mut [Word],
) -> SignedWord {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= MAX_LEN_KARATSUBA);

    add_signed_mul_split_into_same_len(
        c,
        sign,
        a,
        b,
        temp,
        add_signed_mul_karatsuba_same_len,
        |c, sign, a, b, _| add_signed_mul_simple(c, sign, a, b),
        MAX_LEN_SIMPLE,
    )
}

/// c = a * b
/// Karatsuba method: O(n^1.59).
fn mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) {
    c.fill(0);
    let overflow = add_signed_mul_karatsuba(c, Positive, a, b, temp);
    assert!(overflow == 0);
}

/// c += sign * a * b
/// Karatsuba method: O(n^1.59).
///
/// Returns carry.
fn add_signed_mul_karatsuba_same_len(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    temp: &mut [Word],
) -> SignedWord {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(n <= MAX_LEN_KARATSUBA);

    if n <= MAX_LEN_SIMPLE {
        return add_signed_mul_simple(c, sign, a, b);
    }

    let mid = (n + 1) / 2;

    // We must have 3mid <= 2n.
    //
    // If MAX_LEN_SIMPLE >= 2 then n >= 3 and:
    // 6mid <= 3(n+1) = 3n + 3 <= 4n
    const_assert!(MAX_LEN_SIMPLE >= 2);

    let (a_lo, a_hi) = a.split_at(mid);
    let (b_lo, b_hi) = b.split_at(mid);
    let (my_temp, temp) = temp.split_at_mut(2 * mid);
    // Result = a_lo * b_lo + a_hi * b_hi * Word^(2mid)
    //        + (a_lo * b_lo + a_hi * b_hi - (a_lo-a_hi)*(b_lo-b_hi)) * Word^mid
    let mut carry: SignedWord = 0;
    let mut carry_c0: SignedWord = 0; // 2*mid
    let mut carry_c1: SignedWord = 0; // 3*mid

    {
        // c_0 += a_lo * b_lo
        // c_1 += a_lo * b_lo
        let c_lo = &mut my_temp[..];
        mul_karatsuba(c_lo, a_lo, b_lo, temp);
        carry_c0 += add::add_signed_same_len_in_place(&mut c[..2 * mid], sign, c_lo);
        carry_c1 += add::add_signed_same_len_in_place(&mut c[mid..3 * mid], sign, c_lo);
    }
    {
        // c_2 += a_hi * b_hi
        // c_1 += a_hi * b_hi
        let c_hi = &mut my_temp[..2 * (n - mid)];
        mul_karatsuba(c_hi, a_hi, b_hi, temp);
        carry += add::add_signed_same_len_in_place(&mut c[2 * mid..], sign, c_hi);
        carry_c1 += add::add_signed_in_place(&mut c[mid..3 * mid], sign, c_hi);
    }
    {
        // c1 -= (a_lo - a_hi) * (b_lo - b_hi)
        let (a_diff, b_diff) = my_temp.split_at_mut(mid);
        a_diff.copy_from_slice(a_lo);
        let mut diff_sign = add::sub_in_place_with_sign(a_diff, a_hi);
        b_diff.copy_from_slice(b_lo);
        diff_sign *= add::sub_in_place_with_sign(b_diff, b_hi);

        carry_c1 += add_signed_mul_karatsuba(
            &mut c[mid..3 * mid],
            -sign * diff_sign,
            a_diff,
            b_diff,
            temp,
        );
    }

    // Propagate carries.
    carry_c1 += add::add_signed_word_in_place(&mut c[2 * mid..3 * mid], carry_c0);
    carry += add::add_signed_word_in_place(&mut c[3 * mid..], carry_c1);

    assert!(carry.abs() <= 1);
    carry
}

/// c += sign * a * b
/// Toom-Cook-3 method. O(a.len() * b.len()^0.47).
///
/// Returns carry.
fn add_signed_mul_toom_3(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    temp: &mut [Word],
) -> SignedWord {
    assert!(a.len() >= b.len() && c.len() == a.len() + b.len());

    add_signed_mul_split_into_same_len(
        c,
        sign,
        a,
        b,
        temp,
        add_signed_mul_toom_3_same_len,
        add_signed_mul_karatsuba,
        MAX_LEN_KARATSUBA,
    )
}

/// c = a * b
/// Toom-Cook-3 method: O(n^1.47).
fn mul_toom_3_same_len(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) {
    c.fill(0);
    let overflow = add_signed_mul_toom_3_same_len(c, Positive, a, b, temp);
    assert!(overflow == 0);
}

/// c += sign * a * b
/// Toom-Cook-3 method: O(n^1.47).
///
/// Returns carry.
fn add_signed_mul_toom_3_same_len(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    temp: &mut [Word],
) -> SignedWord {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);

    // Brent, Zimmermann, Modern Computer Arithmetic 0.5.9, Algorithm 1.4.
    //
    // We evaluate the polynomials A(x) = a0 + a1*x + a2*x^2, B(x) = b0 + b1*x + b2*x^2
    // at points 0, 1, -1, 2, infinity.
    // Multiplying, this gives us values of V(x) = A(x)*B(x) = c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4
    // at the same points (using 5 recursive multiplications).
    //
    // Then we interpolate the polynomial coefficients, which gives the following formulas:
    // c_0 = V(0)
    // c_1 = V(1) - t1
    // c_2 = t2 - V(0) - V(inf)
    // c_3 = t1 - t2
    // c_4 = V(inf)
    // where:
    // t1 = (3V(0) + 2V(-1) + V(2))/6 - 2V(inf)
    // t2 = (V(1) + V(-1))/2

    if n <= MAX_LEN_KARATSUBA {
        return add_signed_mul_karatsuba(c, sign, a, b, temp);
    }

    // Split into 3 parts. Note: a2, b2 may be shorter.
    let n3 = (n + 2) / 3;

    // We must have:
    // 2 * n3 <= n
    // i * n3 + 2 <= (i+1) * n3
    // 5 * n3 + 2 <= 2n
    //
    // Verify:
    // 2 * n3 <= 2/3 (n+2) = 1/3 (2n + 2) <= n if n >= 2
    // i * n3 + 2 <= (i+1) * n3 if n3 >= 2
    // 5 * n3 + 2 <= 5/3 (n+2) + 2 = 1/3 (5n + 16) <= 2n if n >= 16
    // If n >= 16, then n3 >= (16+2)/3 = 6 >= 2
    const_assert!(MAX_LEN_KARATSUBA >= 15);

    let n3_short = n - 2 * n3;

    let (a0, a12) = a.split_at(n3);
    let (a1, a2) = a12.split_at(n3);
    let (b0, b12) = b.split_at(n3);
    let (b1, b2) = b12.split_at(n3);

    let (a_eval, temp) = temp.split_at_mut(n3 + 1);
    let (b_eval, temp) = temp.split_at_mut(n3 + 1);
    let (c_eval, temp) = temp.split_at_mut(2 * n3 + 2);
    let (t1, temp) = temp.split_at_mut(2 * n3 + 2);
    let (t2, temp) = temp.split_at_mut(2 * n3 + 2);

    let mut carry: SignedWord = 0;
    // Accumulate intermediate carries, we will add them at the end.
    let mut carry_c0: SignedWord = 0; // at 2*n3
    let mut carry_c1: SignedWord = 0; // at 3*n3+2
    let mut carry_c2: SignedWord = 0; // at 4*n3+2
    let mut carry_c3: SignedWord = 0; // at 5*n3+2

    // Evaluate at 0.
    // V(0) = a0 * b0
    // c_0 += V(0)
    // c_2 -= V(0)
    // t1 = 3*V(0)
    {
        let t1_short = &mut t1[..2 * n3];
        mul_toom_3_same_len(t1_short, a0, b0, temp);
        carry_c0 += add::add_signed_same_len_in_place(&mut c[..2 * n3], sign, t1_short);
        carry_c2 += add::add_signed_in_place(&mut c[2 * n3..4 * n3 + 2], -sign, t1_short);
        t1[2 * n3] = mul_word_in_place(t1_short, 3);
        t1[2 * n3 + 1] = 0;
    }

    // Evaluate at 2.
    // a_eval = a0 + 2a1 + 4a2
    // b_eval = b0 + 2b1 + 4b2
    // V(2) = a_eval * b_eval
    // t1 += V(2)
    a_eval[..n3].copy_from_slice(a0);
    a_eval[n3] = add_mul_word_same_len_in_place(&mut a_eval[..n3], 2, a1);
    a_eval[n3] += add_mul_word_in_place(&mut a_eval[..n3], 4, a2);
    b_eval[..n3].copy_from_slice(b0);
    b_eval[n3] = add_mul_word_same_len_in_place(&mut b_eval[..n3], 2, b1);
    b_eval[n3] += add_mul_word_in_place(&mut b_eval[..n3], 4, b2);
    let overflow = add_signed_mul_toom_3_same_len(t1, Positive, a_eval, b_eval, temp);
    assert!(overflow == 0);

    // Evaluate at inf.
    // V(inf) = a4 * b4
    // c_2 -= V(inf)
    // c_4 += V(inf)
    // t1 -= 12V(inf)
    // Now t1 = 3V(0) + V(2) - 12V(inf)
    {
        let c_eval_short = &mut c_eval[..2 * n3_short];
        mul_toom_3_same_len(c_eval_short, a2, b2, temp);
        carry_c2 += add::add_signed_in_place(&mut c[2 * n3..4 * n3 + 2], -sign, c_eval_short);
        carry += add::add_signed_same_len_in_place(&mut c[4 * n3..], sign, c_eval_short);
        c_eval[2 * n3_short] = mul_word_in_place(c_eval_short, 12);
    }
    let overflow = add::sub_in_place(t1, &c_eval[..2 * n3_short + 1]);
    // 3V(0) + V(2) - 12V(inf) is never negative
    assert!(!overflow);

    // Sign of V(-1).
    let mut value_neg1_sign;
    {
        // Evaluate at 1.
        // a_eval = a0 + a1 + a2
        // b_eval = b0 + b1 + b2
        // V(1) = a_eval * b_eval
        // c_1 += V(1)
        // t2 = V(1)
        // Temporarily repurpose c_eval space for a0+a2, b0+b2
        let (a02, b02) = c_eval.split_at_mut(n3 + 1);

        a02[..n3].copy_from_slice(a0);
        a02[n3] = Word::from(add::add_in_place(&mut a02[..n3], a2));
        a_eval.copy_from_slice(a02);
        a_eval[n3] += Word::from(add::add_same_len_in_place(&mut a_eval[..n3], a1));

        b02[..n3].copy_from_slice(b0);
        b02[n3] = Word::from(add::add_in_place(&mut b02[..n3], b2));
        b_eval.copy_from_slice(b02);
        b_eval[n3] += Word::from(add::add_same_len_in_place(&mut b_eval[..n3], b1));

        mul_toom_3_same_len(t2, a_eval, b_eval, temp);
        carry_c1 += add::add_signed_in_place(&mut c[n3..3 * n3 + 2], sign, t2);

        // Evaluate at -1.
        // a_eval = a02 - a1
        // b_eval = b02 - b1
        // V(-1) = a_eval * b_eval
        // t2 += V(-1)
        // t1 += 2*V(-1)
        // Now t1 = 3V(0) + 2V(-1) + V(2) - 12V(inf),
        //     t2 = V(1) + V(-1).
        a_eval.copy_from_slice(a02);
        value_neg1_sign = add::sub_in_place_with_sign(a_eval, a1);
        b_eval.copy_from_slice(b02);
        value_neg1_sign *= add::sub_in_place_with_sign(b_eval, b1);
        // We don't need a02, b02 any more, exit the block so that we can use c_eval again.
    }
    mul_toom_3_same_len(c_eval, a_eval, b_eval, temp);
    let overflow = add::add_signed_same_len_in_place(t2, value_neg1_sign, c_eval);
    assert!(overflow == 0);
    match value_neg1_sign {
        Positive => {
            let overflow = add_mul_word_same_len_in_place(t1, 2, c_eval);
            assert!(overflow == 0);
        }
        Negative => {
            let overflow = sub_mul_word_same_len_in_place(t1, 2, c_eval);
            assert!(overflow == 0);
        }
    }

    // t1 /= 6
    // t2 /= 2
    // Now t1 = (3V(0) + 2V(-1) + V(2))/6 - 2V(inf)
    //     t2 = (V(1) + V(-1))/2
    let t1_rem = div::div_by_word_in_place(t1, 6);
    assert_eq!(t1_rem, 0);
    assert_eq!(t2[0] & 1, 0);
    shift::shr_in_place(t2, 1);

    // c1 -= t1
    // c3 += t1
    // c2 += t2
    // c3 -= t2
    carry_c1 += add::add_signed_same_len_in_place(&mut c[n3..3 * n3 + 2], -sign, t1);
    carry_c3 += add::add_signed_same_len_in_place(&mut c[3 * n3..5 * n3 + 2], sign, t1);
    carry_c2 += add::add_signed_same_len_in_place(&mut c[2 * n3..4 * n3 + 2], sign, t2);
    carry_c3 += add::add_signed_same_len_in_place(&mut c[3 * n3..5 * n3 + 2], -sign, t2);

    // Apply carries.
    carry_c1 += add::add_signed_word_in_place(&mut c[2 * n3..3 * n3 + 2], carry_c0);
    carry_c2 += add::add_signed_word_in_place(&mut c[3 * n3 + 2..4 * n3 + 2], carry_c1);
    carry_c3 += add::add_signed_word_in_place(&mut c[4 * n3 + 2..5 * n3 + 2], carry_c2);
    carry += add::add_signed_word_in_place(&mut c[5 * n3 + 2..], carry_c3);

    assert!(carry.abs() <= 1);
    carry
}
