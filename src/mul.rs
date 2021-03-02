use crate::{
    add::{
        add_in_place, add_one_in_place, add_same_len_in_place, add_word_in_place, sub_in_place,
        sub_in_place_with_sign, sub_one_in_place, sub_same_len_in_place, sub_word_in_place,
    },
    buffer::Buffer,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, Word},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Mul, MulAssign},
};

/// If both lengths >= this, use Karatsuba algorithm.
const THRESHOLD_KARATSUBA: usize = 24;

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
        debug_assert!(!overflow);
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
    debug_assert!(words.len() == rhs.len());
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

/// words -= mult * rhs
///
/// Returns borrow.
pub(crate) fn sub_mul_word_same_len_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    debug_assert!(words.len() == rhs.len());
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

/// c += a * b
///
/// Returns carry.
fn add_mul(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    if b.len() < THRESHOLD_KARATSUBA {
        add_mul_simple(c, a, b)
    } else {
        add_mul_karatsuba_different_len(c, a, b)
    }
}

/// c += a * b
///
/// Returns carry.
fn add_mul_simple(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(
        a.len() >= b.len() && b.len() < THRESHOLD_KARATSUBA && c.len() == a.len() + b.len()
    );
    let mut carry: u32 = 0;
    for (i, m) in b.iter().enumerate() {
        let carry1 = add_mul_word_same_len_in_place(&mut c[i..i + a.len()], *m, a);
        carry += u32::from(add_word_in_place(&mut c[i + a.len()..], carry1));
    }
    debug_assert!(carry <= 1);
    carry != 0
}

/// c -= a * b
///
/// Returns borrow.
fn sub_mul_simple(c: &mut [Word], a: &[Word], b: &[Word]) -> bool {
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    let mut borrow: u32 = 0;
    for (i, m) in b.iter().enumerate() {
        let borrow1 = sub_mul_word_same_len_in_place(&mut c[i..i + a.len()], *m, a);
        borrow += u32::from(sub_word_in_place(&mut c[i + a.len()..], borrow1));
    }
    debug_assert!(borrow <= 1);
    borrow != 0
}

/// c += a * b
///
/// Returns carry.
fn add_mul_karatsuba_different_len<'a>(
    mut c: &mut [Word],
    mut a: &'a [Word],
    mut b: &'a [Word],
) -> bool {
    debug_assert!(
        a.len() >= b.len() && b.len() >= THRESHOLD_KARATSUBA && c.len() == a.len() + b.len()
    );
    let temp_len = karatsuba_temp_buffer_len(b.len());
    let mut temp = Buffer::allocate_no_extra(temp_len);
    temp.push_zeros(temp_len);

    let mut carry: u32 = 0;
    while b.len() >= THRESHOLD_KARATSUBA {
        let n = b.len();
        while a.len() >= b.len() {
            let (a_lo, a_hi) = a.split_at(n);
            let carry1 = add_mul_karatsuba(&mut c[..2 * n], a_lo, b, &mut temp);
            if carry1 {
                carry += u32::from(add_one_in_place(&mut c[2 * n..]));
            }
            a = a_hi;
            c = &mut c[n..];
        }
        mem::swap(&mut a, &mut b);
    }
    if !b.is_empty() {
        carry += u32::from(add_mul_simple(c, a, b));
    }
    debug_assert!(carry <= 1);
    carry != 0
}

/// Minimum temporary buffer required for Karatsuba multiplication.
fn karatsuba_temp_buffer_len(n: usize) -> usize {
    // We prove by induction that f(n) <= 2n + 2ceil(log_2 n).
    //
    // f(n) = 2ceil(n/2) + f(ceil(n/2)) <= n+1 + n+1 + 2ceil(log ceil n/2)
    //      <= 2n+2 + 2(ceil(log n)-1) = 2n + 2ceil(log n)
    2 * n + 2 * (n.next_power_of_two().trailing_zeros() as usize)
}

/// c += a * b
///
/// Returns carry.
fn add_mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) -> bool {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(temp.len() >= karatsuba_temp_buffer_len(n));

    if n < THRESHOLD_KARATSUBA {
        return add_mul_simple(c, a, b);
    }

    let mid = (n + 1) / 2;
    let (a_lo, a_hi) = a.split_at(mid);
    let (b_lo, b_hi) = b.split_at(mid);
    let (my_temp, recursive_temp) = temp.split_at_mut(2 * mid);
    // Result = a_lo * b_lo + a_hi * b_hi * Word^(2mid)
    //        + (a_lo * b_lo + a_hi * b_hi - (a_lo-a_hi)*(b_lo-b_hi)) * Word^mid
    let mut carry: i32 = 0;
    {
        // a_lo * b_lo
        let c_lo = &mut my_temp[..];
        c_lo.fill(0);
        let overflow = add_mul_karatsuba(c_lo, a_lo, b_lo, recursive_temp);
        debug_assert!(!overflow);
        carry += i32::from(add_in_place(c, c_lo));
        carry += i32::from(add_in_place(&mut c[mid..], c_lo));
    }
    {
        let c_hi = &mut my_temp[..2 * (n - mid)];
        c_hi.fill(0);
        let overflow = add_mul_karatsuba(c_hi, a_hi, b_hi, recursive_temp);
        debug_assert!(!overflow);
        carry += i32::from(add_same_len_in_place(&mut c[2 * mid..], c_hi));
        carry += i32::from(add_in_place(&mut c[mid..], c_hi));
    }
    {
        let (a_diff, b_diff) = my_temp.split_at_mut(mid);
        a_diff.copy_from_slice(a_lo);
        let mut sign = sub_in_place_with_sign(a_diff, a_hi);
        b_diff.copy_from_slice(b_lo);
        sign *= sub_in_place_with_sign(b_diff, b_hi);
        match sign {
            Positive => {
                let borrow1 =
                    sub_mul_karatsuba(&mut c[mid..3 * mid], a_diff, b_diff, recursive_temp);
                if borrow1 {
                    carry -= i32::from(sub_one_in_place(&mut c[3 * mid..]));
                }
            }
            Negative => {
                let carry1 =
                    add_mul_karatsuba(&mut c[mid..3 * mid], a_diff, b_diff, recursive_temp);
                if carry1 {
                    carry += i32::from(add_one_in_place(&mut c[3 * mid..]));
                }
            }
        }
    }
    assert!(carry >= 0 && carry <= 1);
    carry != 0
}

/// c -= a * b
///
/// Returns borrow.
fn sub_mul_karatsuba(c: &mut [Word], a: &[Word], b: &[Word], temp: &mut [Word]) -> bool {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(temp.len() >= karatsuba_temp_buffer_len(n));

    if n < THRESHOLD_KARATSUBA {
        return sub_mul_simple(c, a, b);
    }

    let mid = (n + 1) / 2;
    let (a_lo, a_hi) = a.split_at(mid);
    let (b_lo, b_hi) = b.split_at(mid);
    let (my_temp, recursive_temp) = temp.split_at_mut(2 * mid);
    // Result = a_lo * b_lo + a_hi * b_hi * Word^(2mid)
    //        + (a_lo * b_lo + a_hi * b_hi - (a_lo-a_hi)*(b_lo-b_hi)) * Word^mid
    let mut borrow: i32 = 0;
    {
        // a_lo * b_lo
        let c_lo = &mut my_temp[..];
        c_lo.fill(0);
        let overflow = add_mul_karatsuba(c_lo, a_lo, b_lo, recursive_temp);
        debug_assert!(!overflow);
        borrow += i32::from(sub_in_place(c, c_lo));
        borrow += i32::from(sub_in_place(&mut c[mid..], c_lo));
    }
    {
        let c_hi = &mut my_temp[..2 * (n - mid)];
        c_hi.fill(0);
        let overflow = add_mul_karatsuba(c_hi, a_hi, b_hi, recursive_temp);
        debug_assert!(!overflow);
        borrow += i32::from(sub_same_len_in_place(&mut c[2 * mid..], c_hi));
        borrow += i32::from(sub_in_place(&mut c[mid..], c_hi));
    }
    {
        let (a_diff, b_diff) = my_temp.split_at_mut(mid);
        a_diff.copy_from_slice(a_lo);
        let mut sign = sub_in_place_with_sign(a_diff, a_hi);
        b_diff.copy_from_slice(b_lo);
        sign *= sub_in_place_with_sign(b_diff, b_hi);
        match sign {
            Positive => {
                let carry1 =
                    add_mul_karatsuba(&mut c[mid..3 * mid], a_diff, b_diff, recursive_temp);
                if carry1 {
                    borrow -= i32::from(add_one_in_place(&mut c[3 * mid..]));
                }
            }
            Negative => {
                let borrow1 =
                    sub_mul_karatsuba(&mut c[mid..3 * mid], a_diff, b_diff, recursive_temp);
                if borrow1 {
                    borrow += i32::from(sub_one_in_place(&mut c[3 * mid..]));
                }
            }
        }
    }
    assert!(borrow >= 0 && borrow <= 1);
    borrow != 0
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
