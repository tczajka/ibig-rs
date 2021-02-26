use crate::{
    buffer::Buffer,
    ibig::IBig,
    primitive::{extend_word, split_double_word, Word},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    cmp::{min, Ordering},
    convert::TryFrom,
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

impl Add<UBig> for UBig {
    type Output = UBig;

    fn add(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0, &buffer1)
                } else {
                    UBig::add_large(buffer1, &buffer0)
                }
            }
        }
    }
}

impl Add<&UBig> for UBig {
    type Output = UBig;

    fn add(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::add_word(word0, *word1),
                Large(buffer1) => UBig::add_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::add_large_word(buffer0, *word1),
                Large(buffer1) => UBig::add_large(buffer0, buffer1),
            },
        }
    }
}

impl Add<UBig> for &UBig {
    type Output = UBig;

    fn add(self, rhs: UBig) -> UBig {
        rhs.add(self)
    }
}

impl Add<&UBig> for &UBig {
    type Output = UBig;

    fn add(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0.clone(), buffer1)
                } else {
                    UBig::add_large(buffer1.clone(), buffer0)
                }
            }
        }
    }
}

impl AddAssign<UBig> for UBig {
    fn add_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&UBig> for UBig {
    fn add_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl UBig {
    /// Add two `Word`s.
    fn add_word(a: Word, b: Word) -> UBig {
        let (res, overflow) = a.overflowing_add(b);
        if overflow {
            let mut buffer = Buffer::allocate(2);
            buffer.push(res);
            buffer.push(1);
            buffer.into()
        } else {
            UBig::from_word(res)
        }
    }

    /// Add a large number to a `Word`.
    fn add_large_word(mut buffer: Buffer, rhs: Word) -> UBig {
        debug_assert!(buffer.len() >= 2);
        if add_word_in_place(&mut buffer, rhs) {
            buffer.push_may_reallocate(1);
        }
        buffer.into()
    }

    /// Add two large numbers.
    fn add_large(mut buffer: Buffer, rhs: &[Word]) -> UBig {
        let n = min(buffer.len(), rhs.len());
        let overflow = add_same_len_in_place(&mut buffer[..n], &rhs[..n]);
        if rhs.len() > n {
            buffer.ensure_capacity(rhs.len());
            buffer.extend(&rhs[n..]);
        }
        if overflow && add_one_in_place(&mut buffer[n..]) {
            buffer.push_may_reallocate(1);
        }
        buffer.into()
    }
}

/// Add one to a word sequence.
///
/// Returns overflow.
fn add_one_in_place(words: &mut [Word]) -> bool {
    for word in words {
        let (a, overflow) = word.overflowing_add(1);
        *word = a;
        if !overflow {
            return false;
        }
    }
    true
}

/// Add a word to a non-empty word sequence.
///
/// Returns overflow.
fn add_word_in_place(words: &mut [Word], rhs: Word) -> bool {
    debug_assert!(!words.is_empty());
    let (a, overflow) = words[0].overflowing_add(rhs);
    words[0] = a;
    overflow && add_one_in_place(&mut words[1..])
}

/// Add a word to a non-empty word sequence.
///
/// Returns overflow.
pub(crate) fn add_same_len_in_place(words: &mut [Word], rhs: &[Word]) -> bool {
    debug_assert!(words.len() == rhs.len());

    let mut carry = 0;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        let (sum, c) = split_double_word(extend_word(*a) + extend_word(*b) + extend_word(carry));
        *a = sum;
        carry = c;
    }
    carry != 0
}

impl Sub<UBig> for UBig {
    type Output = UBig;

    fn sub(self, rhs: UBig) -> UBig {
        UBig::from_ibig_after_sub(IBig::sub_ubig_val_val(self, rhs))
    }
}

impl Sub<&UBig> for UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_after_sub(IBig::sub_ubig_val_ref(self, rhs))
    }
}

impl Sub<UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: UBig) -> UBig {
        UBig::from_ibig_after_sub(-IBig::sub_ubig_val_ref(rhs, self))
    }
}

impl Sub<&UBig> for &UBig {
    type Output = UBig;

    fn sub(self, rhs: &UBig) -> UBig {
        UBig::from_ibig_after_sub(IBig::sub_ubig_ref_ref(self, rhs))
    }
}

impl SubAssign<UBig> for UBig {
    fn sub_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&UBig> for UBig {
    fn sub_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl UBig {
    fn from_ibig_after_sub(x: IBig) -> UBig {
        match UBig::try_from(x) {
            Ok(v) => v,
            Err(_) => panic!("UBig subtraction overflow"),
        }
    }

    fn sub_large_word(mut lhs: Buffer, rhs: Word) -> UBig {
        let borrow = sub_word_in_place(&mut lhs, rhs);
        assert!(!borrow);
        lhs.into()
    }

    /// Subtract lhs - rhs.
    ///
    /// Panics if lhs < rhs.
    fn sub_large_val_ref_no_overflow(mut lhs: Buffer, rhs: &[Word]) -> UBig {
        let n = rhs.len();
        let borrow = sub_words_same_len_in_place(&mut lhs[..n], rhs);
        if borrow {
            let borrow2 = sub_one_in_place(&mut lhs[n..]);
            assert!(!borrow2);
        }
        lhs.into()
    }

    /// Subtract lhs - rhs.
    ///
    /// Panics if lhs < rhs.
    fn sub_large_ref_val_no_overflow(lhs: &[Word], mut rhs: Buffer) -> UBig {
        let n = rhs.len();
        let borrow = sub_words_same_len_in_place_swap(&lhs[..n], &mut rhs);
        rhs.extend(&lhs[n..]);
        if borrow {
            let borrow2 = sub_one_in_place(&mut rhs[n..]);
            assert!(!borrow2);
        }
        rhs.into()
    }
}

/// Subtract one from a word sequence.
///
/// Returns borrow.
fn sub_one_in_place(words: &mut [Word]) -> bool {
    for word in words {
        let (a, borrow) = word.overflowing_sub(1);
        *word = a;
        if !borrow {
            return false;
        }
    }
    true
}

/// Subtract a word from a non-empty word sequence.
///
/// Returns borrow.
fn sub_word_in_place(words: &mut [Word], rhs: Word) -> bool {
    debug_assert!(!words.is_empty());
    let (a, borrow) = words[0].overflowing_sub(rhs);
    words[0] = a;
    borrow && sub_one_in_place(&mut words[1..])
}

/// lhs -= rhs
///
/// Returns borrow.
fn sub_words_same_len_in_place(lhs: &mut [Word], rhs: &[Word]) -> bool {
    // carry_plus_1 is 0 or 1
    let mut carry_plus_1: Word = 1;
    for (a, b) in lhs.iter_mut().zip(rhs.iter()) {
        // (diff, c) = a - b + carry_plus_1 + (1 << WORD_BITS - 1)
        let (diff, c) = split_double_word(
            extend_word(*a) + extend_word(Word::MAX) + extend_word(carry_plus_1) - extend_word(*b),
        );
        *a = diff;
        carry_plus_1 = c;
    }
    carry_plus_1 == 0
}

/// rhs = lhs - rhs
///
/// Returns borrow.
fn sub_words_same_len_in_place_swap(lhs: &[Word], rhs: &mut [Word]) -> bool {
    // carry_plus_1 is 0 or 1
    let mut carry_plus_1: Word = 1;
    for (a, b) in lhs.iter().zip(rhs.iter_mut()) {
        // (diff, c) = a - b + carry_plus_1 + (1 << WORD_BITS - 1)
        let (diff, c) = split_double_word(
            extend_word(*a) + extend_word(Word::MAX) + extend_word(carry_plus_1) - extend_word(*b),
        );
        *b = diff;
        carry_plus_1 = c;
    }
    carry_plus_1 == 0
}

impl IBig {
    fn sub_ubig_val_val(lhs: UBig, rhs: UBig) -> IBig {
        match (lhs.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => IBig::sub_word_word(word0, word1),
            (Small(word0), Large(buffer1)) => -IBig::sub_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => IBig::sub_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    IBig::sub_large(buffer0, &buffer1)
                } else {
                    -IBig::sub_large(buffer1, &buffer0)
                }
            }
        }
    }

    fn sub_ubig_val_ref(lhs: UBig, rhs: &UBig) -> IBig {
        match lhs.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => IBig::sub_word_word(word0, *word1),
                Large(buffer1) => -IBig::sub_large_word(buffer1.clone(), word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => IBig::sub_large_word(buffer0, *word1),
                Large(buffer1) => IBig::sub_large(buffer0, buffer1),
            },
        }
    }

    fn sub_ubig_ref_ref(lhs: &UBig, rhs: &UBig) -> IBig {
        match (lhs.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => IBig::sub_word_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => -IBig::sub_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => IBig::sub_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    IBig::sub_large(buffer0.clone(), buffer1)
                } else {
                    -IBig::sub_large(buffer1.clone(), buffer0)
                }
            }
        }
    }

    fn sub_word_word(lhs: Word, rhs: Word) -> IBig {
        if lhs >= rhs {
            IBig::from(lhs - rhs)
        } else {
            -IBig::from(rhs - lhs)
        }
    }

    fn sub_large_word(lhs: Buffer, rhs: Word) -> IBig {
        UBig::sub_large_word(lhs, rhs).into()
    }

    fn sub_large(mut lhs: Buffer, rhs: &[Word]) -> IBig {
        match lhs.len().cmp(&rhs.len()) {
            Ordering::Greater => IBig::from(UBig::sub_large_val_ref_no_overflow(lhs, rhs)),
            Ordering::Less => -IBig::from(UBig::sub_large_ref_val_no_overflow(rhs, lhs)),
            Ordering::Equal => {
                let mut n = lhs.len();
                while n > 0 {
                    match lhs[n - 1].cmp(&rhs[n - 1]) {
                        Ordering::Greater => {
                            lhs.truncate(n);
                            return IBig::from(UBig::sub_large_val_ref_no_overflow(lhs, &rhs[..n]));
                        }
                        Ordering::Less => {
                            lhs.truncate(n);
                            return -IBig::from(UBig::sub_large_ref_val_no_overflow(
                                &rhs[..n],
                                lhs,
                            ));
                        }
                        Ordering::Equal => {}
                    }
                    n -= 1;
                }
                IBig::from(0u8)
            }
        }
    }
}

impl Add<IBig> for IBig {
    type Output = IBig;

    fn add(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_val(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_val_val(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<&IBig> for IBig {
    type Output = IBig;

    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Positive) => -IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<IBig> for &IBig {
    type Output = IBig;

    fn add(self, rhs: IBig) -> IBig {
        rhs.add(self)
    }
}

impl Add<&IBig> for &IBig {
    type Output = IBig;

    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_ref_ref(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl AddAssign<IBig> for IBig {
    fn add_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&IBig> for IBig {
    fn add_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl Sub<IBig> for IBig {
    type Output = IBig;

    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for IBig {
    type Output = IBig;

    fn sub(self, rhs: &IBig) -> IBig {
        -(-self + rhs)
    }
}

impl Sub<IBig> for &IBig {
    type Output = IBig;

    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for &IBig {
    type Output = IBig;

    fn sub(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Positive, Negative) => IBig::from(mag0 + mag1),
            (Negative, Positive) => -IBig::from(mag0 + mag1),
            (Negative, Negative) => IBig::sub_ubig_ref_ref(mag1, mag0),
        }
    }
}

impl SubAssign<IBig> for IBig {
    fn sub_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&IBig> for IBig {
    fn sub_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) - rhs;
    }
}
