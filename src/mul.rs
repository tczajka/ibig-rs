use crate::{
    add::{
        add_in_place, add_one_in_place, add_same_len_in_place, add_signed_word_in_place,
        add_word_in_place, sub_in_place,
        sub_in_place_with_sign, sub_same_len_in_place, sub_word_in_place,
    },
    buffer::Buffer,
    div::div_rem_by_word_in_place,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, SignedWord, Word},
    shift::shr_in_place,
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Mul, MulAssign},
};

/// If both lengths >= this, use Karatsuba algorithm.
const THRESHOLD_KARATSUBA: usize = 24;
/// If both lengths >= this, use Toom-Cook-3 algorithm.
const THRESHOLD_TOOM_3: usize = 192;

impl Mul<UBig> for UBig {
    type Output = UBig;

    fn mul(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(&buffer0, &buffer1),
        }
    }
}

impl Mul<&UBig> for UBig {
    type Output = UBig;

    fn mul(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::mul_word(word0, *word1),
                Large(buffer1) => UBig::mul_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::mul_large_word(buffer0, *word1),
                Large(buffer1) => UBig::mul_large(&buffer0, buffer1),
            },
        }
    }
}

impl Mul<UBig> for &UBig {
    type Output = UBig;

    fn mul(self, rhs: UBig) -> UBig {
        rhs.mul(self)
    }
}

impl Mul<&UBig> for &UBig {
    type Output = UBig;

    fn mul(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(buffer0, buffer1),
        }
    }
}

impl MulAssign<UBig> for UBig {
    fn mul_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&UBig> for UBig {
    fn mul_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl UBig {
    /// Multiply two `Word`s.
    fn mul_word(a: Word, b: Word) -> UBig {
        match a.checked_mul(b) {
            Some(c) => UBig::from_word(c),
            None => UBig::from(extend_word(a) * extend_word(b)),
        }
    }

    /// Multiply a large number by a `Word`.
    fn mul_large_word(mut buffer: Buffer, a: Word) -> UBig {
        match a {
            0 => UBig::from_word(0),
            1 => buffer.into(),
            _ => {
                let carry = mul_word_in_place(&mut buffer, a);
                if carry != 0 {
                    buffer.push_may_reallocate(carry);
                }
                buffer.into()
            }
        }
    }

    /// Multiply two large numbers.
    fn mul_large(lhs: &[Word], rhs: &[Word]) -> UBig {
        debug_assert!(lhs.len() >= 2 && rhs.len() >= 2);
        let (lhs, rhs) = if lhs.len() >= rhs.len() {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        let mut buffer = Buffer::allocate(lhs.len() + rhs.len());
        buffer.push_zeros(lhs.len() + rhs.len());
        let overflow = add_mul(&mut buffer, lhs, rhs);
        assert!(!overflow);
        buffer.into()
    }
}

/// Multiply a word sequence by a `Word` in place.
///
/// Returns carry.
fn mul_word_in_place(words: &mut [Word], rhs: Word) -> Word {
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
        carry = Word::from(add_word_in_place(&mut words[n..], carry));
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
        borrow = Word::from(sub_word_in_place(&mut words[n..], borrow));
    }
    borrow
}

/// c += a * b
///
/// Returns carry.
fn add_mul(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    if b.len() < THRESHOLD_KARATSUBA {
        // Simple O(n^2) method.
        add_mul_simple(c, a, b)
    } else if b.len() < THRESHOLD_TOOM_3 {
        // Karatsuba method: O(n^(log_2 3)) = O(n^1.59)
        let temp_len = karatsuba_temp_buffer_len(b.len());
        let mut temp = Buffer::allocate_no_extra(temp_len);
        temp.push_zeros(temp_len);
        add_mul_karatsuba_different_len(c, a, b, &mut temp)
    } else {
        // Toom-Cook-3 method: O(n^(log_3 5)) = O(n^1.47)
        let temp_len = toom_3_temp_buffer_len(b.len());
        let mut temp = Buffer::allocate_no_extra(temp_len);
        temp.push_zeros(temp_len);
        add_mul_toom_3_different_len(c, a, b, &mut temp)
    }
}

/// c += a * b
/// Simple method: O(a.len() * b.len()).
///
/// Returns carry.
fn add_mul_simple(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() < THRESHOLD_KARATSUBA);
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
fn sub_mul_simple(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() < THRESHOLD_KARATSUBA);
    let mut borrow: Word = 0;
    for (i, m) in b.iter().enumerate() {
        borrow += sub_mul_word_in_place(&mut c[i..], *m, a);
    }
    debug_assert!(borrow <= 1);
    borrow != 0
}

/// Minimum temporary buffer required for Karatsuba multiplication.
fn karatsuba_temp_buffer_len(n: usize) -> usize {
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
}

/// c += a * b
/// Karatsuba method: O(a.len() * b.len()^0.59).
///
/// Returns carry.
fn add_mul_karatsuba_different_len<'a>(
    mut c: &mut [Word],
    mut a: &'a [Word],
    mut b: &'a [Word],
    temp: &mut [Word],
) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() < THRESHOLD_TOOM_3);

    let mut carry: Word = 0;
    while b.len() >= THRESHOLD_KARATSUBA {
        let n = b.len();
        while a.len() >= n {
            let (a_lo, a_hi) = a.split_at(n);
            let carry1 = add_mul_karatsuba(&mut c[..2 * n], a_lo, b, temp);
            if carry1 {
                carry += Word::from(add_one_in_place(&mut c[2 * n..]));
            }
            a = a_hi;
            c = &mut c[n..];
        }
        mem::swap(&mut a, &mut b);
    }
    carry += Word::from(add_mul_simple(c, a, b));
    debug_assert!(carry <= 1);
    carry != 0
}

/// c = a * b
/// Karatsuba method: O(n^1.59).
fn mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) {
    c.fill(0);
    let overflow = add_mul_karatsuba(c, a, b, temp);
    assert!(!overflow);
}

/// c += a * b
/// Karatsuba method: O(n^1.59).
///
/// Returns carry.
fn add_mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) -> bool {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(n < THRESHOLD_TOOM_3);

    if n < THRESHOLD_KARATSUBA {
        return add_mul_simple(c, a, b);
    }

    let mid = (n + 1) / 2;
    let (a_lo, a_hi) = a.split_at(mid);
    let (b_lo, b_hi) = b.split_at(mid);
    let (my_temp, temp) = temp.split_at_mut(2 * mid);
    // Result = a_lo * b_lo + a_hi * b_hi * Word^(2mid)
    //        + (a_lo * b_lo + a_hi * b_hi - (a_lo-a_hi)*(b_lo-b_hi)) * Word^mid
    let mut carry: SignedWord = 0;
    let mut carry_c0: SignedWord = 0; // 2*mid
    let mut carry_c1: SignedWord = 0; // 3*mid

    {
        // a_lo * b_lo
        let c_lo = &mut my_temp[..];
        mul_karatsuba(c_lo, a_lo, b_lo, temp);
        carry_c0 += SignedWord::from(add_same_len_in_place(&mut c[..2 * mid], c_lo));
        carry_c1 += SignedWord::from(add_same_len_in_place(&mut c[mid..3 * mid], c_lo));
    }
    {
        let c_hi = &mut my_temp[..2 * (n - mid)];
        mul_karatsuba(c_hi, a_hi, b_hi, temp);
        carry += SignedWord::from(add_same_len_in_place(&mut c[2 * mid..], c_hi));
        carry_c1 += SignedWord::from(add_in_place(&mut c[mid..3 * mid], c_hi));
    }
    {
        let (a_diff, b_diff) = my_temp.split_at_mut(mid);
        a_diff.copy_from_slice(a_lo);
        let mut sign = sub_in_place_with_sign(a_diff, a_hi);
        b_diff.copy_from_slice(b_lo);
        sign *= sub_in_place_with_sign(b_diff, b_hi);
        match sign {
            Positive => {
                carry_c1 -= SignedWord::from(sub_mul_karatsuba(
                    &mut c[mid..3 * mid],
                    a_diff,
                    b_diff,
                    temp,
                ));
            }
            Negative => {
                carry_c1 += SignedWord::from(add_mul_karatsuba(
                    &mut c[mid..3 * mid],
                    a_diff,
                    b_diff,
                    temp,
                ));
            }
        }
    }

    // Apply carries.
    carry_c1 += add_signed_word_in_place(&mut c[2 * mid..3 * mid], carry_c0);
    carry += add_signed_word_in_place(&mut c[3 * mid..], carry_c1);

    assert!(carry >= 0 && carry <= 1);
    carry != 0
}

/// c -= a * b
/// Karatsuba method: O(n^1.59).
///
/// Returns borrow.
fn sub_mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) -> bool {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(n < THRESHOLD_TOOM_3);

    if n < THRESHOLD_KARATSUBA {
        return sub_mul_simple(c, a, b);
    }

    let mid = (n + 1) / 2;
    let (a_lo, a_hi) = a.split_at(mid);
    let (b_lo, b_hi) = b.split_at(mid);
    let (my_temp, temp) = temp.split_at_mut(2 * mid);
    // Result = a_lo * b_lo + a_hi * b_hi * Word^(2mid)
    //        + (a_lo * b_lo + a_hi * b_hi - (a_lo-a_hi)*(b_lo-b_hi)) * Word^mid
    let mut carry: SignedWord = 0;
    let mut carry_c0: SignedWord = 0; // 2*mid
    let mut carry_c1: SignedWord = 0; // 3*mid
    {
        // a_lo * b_lo
        let c_lo = &mut my_temp[..];
        mul_karatsuba(c_lo, a_lo, b_lo, temp);
        carry_c0 -= SignedWord::from(sub_same_len_in_place(&mut c[..2 * mid], c_lo));
        carry_c1 -= SignedWord::from(sub_same_len_in_place(&mut c[mid..3 * mid], c_lo));
    }
    {
        let c_hi = &mut my_temp[..2 * (n - mid)];
        mul_karatsuba(c_hi, a_hi, b_hi, temp);
        carry -= SignedWord::from(sub_same_len_in_place(&mut c[2 * mid..], c_hi));
        carry_c1 -= SignedWord::from(sub_in_place(&mut c[mid..3 * mid], c_hi));
    }
    {
        let (a_diff, b_diff) = my_temp.split_at_mut(mid);
        a_diff.copy_from_slice(a_lo);
        let mut sign = sub_in_place_with_sign(a_diff, a_hi);
        b_diff.copy_from_slice(b_lo);
        sign *= sub_in_place_with_sign(b_diff, b_hi);
        match sign {
            Positive => {
                carry_c1 += SignedWord::from(add_mul_karatsuba(
                    &mut c[mid..3 * mid],
                    a_diff,
                    b_diff,
                    temp,
                ));
            }
            Negative => {
                carry_c1 -= SignedWord::from(sub_mul_karatsuba(
                    &mut c[mid..3 * mid],
                    a_diff,
                    b_diff,
                    temp,
                ));
            }
        }
    }

    // Apply carries.
    carry_c1 += add_signed_word_in_place(&mut c[2 * mid..3 * mid], carry_c0);
    carry += add_signed_word_in_place(&mut c[3 * mid..], carry_c1);

    assert!(carry >= -1 && carry <= 0);
    carry != 0
}

/// Minimum temporary buffer required for Toom-3 multiplication.
/// Note: toom_3_temp_buffer_len(n) >= karatsuba_temp_buffer_len(n).
fn toom_3_temp_buffer_len(n: usize) -> usize {
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
    // This is more than Karatsuba.
    4 * n + 13 * (n.next_power_of_two().trailing_zeros() as usize)
}

/// c += a * b
/// Toom-Cook-3 method. O(a.len() * b.len()^0.47).
///
/// Returns carry.
fn add_mul_toom_3_different_len<'a>(
    mut c: &mut [Word],
    mut a: &'a [Word],
    mut b: &'a [Word],
    temp: &mut [Word],
) -> bool {
    assert!(a.len() >= b.len() && c.len() == a.len() + b.len());

    let mut carry: Word = 0;
    while b.len() >= THRESHOLD_TOOM_3 {
        let n = b.len();
        while a.len() >= n {
            let (a_lo, a_hi) = a.split_at(n);
            let carry1 = add_mul_toom_3(&mut c[..2 * n], a_lo, b, temp);
            if carry1 {
                carry += Word::from(add_one_in_place(&mut c[2 * n..]));
            }
            a = a_hi;
            c = &mut c[n..];
        }
        mem::swap(&mut a, &mut b);
    }
    carry += Word::from(add_mul_karatsuba_different_len(c, a, b, temp));
    assert!(carry <= 1);
    carry != 0
}

/// c = a * b
/// Toom-Cook-3 method: O(n^1.47).
fn mul_toom_3(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) {
    c.fill(0);
    let overflow = add_mul_toom_3(c, a, b, temp);
    assert!(!overflow);
}

/// c += a * b
/// Toom-Cook-3 method: O(n^1.47).
///
/// Returns carry.
fn add_mul_toom_3(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) -> bool {
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

    if n < THRESHOLD_TOOM_3 {
        return add_mul_karatsuba(c, a, b, temp);
    }

    // Split into 3 parts. Note: a2, b2 may be shorter.
    let n3 = (n + 2) / 3;
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
        mul_toom_3(t1_short, a0, b0, temp);
        carry_c0 += SignedWord::from(add_same_len_in_place(&mut c[..2 * n3], t1_short));
        carry_c2 -= SignedWord::from(sub_in_place(&mut c[2 * n3..4 * n3 + 2], t1_short));
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
    let overflow = add_mul_toom_3(t1, a_eval, b_eval, temp);
    assert!(!overflow);

    // Evaluate at inf.
    // V(inf) = a4 * b4
    // c_2 -= V(inf)
    // c_4 += V(inf)
    // t1 -= 12V(inf)
    // Now t1 = 3V(0) + V(2) - 12V(inf)
    {
        let c_eval_short = &mut c_eval[..2 * n3_short];
        mul_toom_3(c_eval_short, a2, b2, temp);
        carry_c2 -= SignedWord::from(sub_in_place(&mut c[2 * n3..4 * n3 + 2], c_eval_short));
        carry += SignedWord::from(add_same_len_in_place(&mut c[4 * n3..], c_eval_short));
        c_eval[2 * n3_short] = mul_word_in_place(c_eval_short, 12);
    }
    let overflow = sub_in_place(t1, &c_eval[..2 * n3_short + 1]);
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
        a02[n3] = Word::from(add_in_place(&mut a02[..n3], a2));
        a_eval.copy_from_slice(a02);
        a_eval[n3] += Word::from(add_same_len_in_place(&mut a_eval[..n3], a1));

        b02[..n3].copy_from_slice(b0);
        b02[n3] = Word::from(add_in_place(&mut b02[..n3], b2));
        b_eval.copy_from_slice(b02);
        b_eval[n3] += Word::from(add_same_len_in_place(&mut b_eval[..n3], b1));

        mul_toom_3(t2, a_eval, b_eval, temp);
        carry_c1 += SignedWord::from(add_in_place(&mut c[n3..3 * n3 + 2], t2));

        // Evaluate at -1.
        // a_eval = a02 - a1
        // b_eval = b02 - b1
        // V(-1) = a_eval * b_eval
        // t2 += V(-1)
        // t1 += 2*V(-1)
        // Now t1 = 3V(0) + 2V(-1) + V(2) - 12V(inf),
        //     t2 = V(1) + V(-1).
        a_eval.copy_from_slice(a02);
        value_neg1_sign = sub_in_place_with_sign(a_eval, a1);
        b_eval.copy_from_slice(b02);
        value_neg1_sign *= sub_in_place_with_sign(b_eval, b1);
        // We don't need a02, b02 any more, exit the block so that we can use c_eval again.
    }
    mul_toom_3(c_eval, a_eval, b_eval, temp);
    match value_neg1_sign {
        Positive => {
            let overflow = add_same_len_in_place(t2, c_eval);
            assert!(!overflow);
            let overflow = add_mul_word_same_len_in_place(t1, 2, c_eval);
            assert!(overflow == 0);
        }
        Negative => {
            let overflow = sub_same_len_in_place(t2, c_eval);
            // t2 is never negative.
            assert!(!overflow);
            let overflow = sub_mul_word_same_len_in_place(t1, 2, c_eval);
            // t1 is never negative.
            assert!(overflow == 0);
        }
    }

    // t1 /= 6
    // t2 /= 2
    // Now t1 = (3V(0) + 2V(-1) + V(2))/6 - 2V(inf)
    //     t2 = (V(1) + V(-1))/2
    let t1_rem = div_rem_by_word_in_place(t1, 6);
    assert_eq!(t1_rem, 0);
    assert_eq!(t2[0] & 1, 0);
    shr_in_place(t2, 1);

    // c1 -= t1
    // c3 += t1
    // c2 += t2
    // c3 -= t2
    carry_c1 -= SignedWord::from(sub_same_len_in_place(&mut c[n3..3 * n3 + 2], t1));
    carry_c3 += SignedWord::from(add_same_len_in_place(&mut c[3 * n3..5 * n3 + 2], t1));
    carry_c2 += SignedWord::from(add_same_len_in_place(&mut c[2 * n3..4 * n3 + 2], t2));
    carry_c3 -= SignedWord::from(sub_same_len_in_place(&mut c[3 * n3..5 * n3 + 2], t2));

    // Apply carries.
    carry_c1 += add_signed_word_in_place(&mut c[2 * n3..3 * n3 + 2], carry_c0);
    carry_c2 += add_signed_word_in_place(&mut c[3 * n3 + 2..4 * n3 + 2], carry_c1);
    carry_c3 += add_signed_word_in_place(&mut c[4 * n3 + 2..5 * n3 + 2], carry_c2);
    carry += add_signed_word_in_place(&mut c[5 * n3 + 2..], carry_c3);

    assert!(carry >= 0 && carry <= 1);
    carry != 0
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    fn mul(self, rhs: Sign) -> Sign {
        match (self, rhs) {
            (Positive, Positive) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
            (Negative, Negative) => Positive,
        }
    }
}

impl MulAssign<Sign> for Sign {
    fn mul_assign(&mut self, rhs: Sign) {
        *self = *self * rhs;
    }
}

impl Mul<IBig> for IBig {
    type Output = IBig;

    fn mul(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<&IBig> for IBig {
    type Output = IBig;

    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<IBig> for &IBig {
    type Output = IBig;

    fn mul(self, rhs: IBig) -> IBig {
        rhs.mul(self)
    }
}

impl Mul<&IBig> for &IBig {
    type Output = IBig;

    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl MulAssign<IBig> for IBig {
    fn mul_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&IBig> for IBig {
    fn mul_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) * rhs;
    }
}
