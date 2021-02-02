use crate::{
    buffer::Buffer,
    primitive::{extend_word, split_double_word, Word},
    ubig::{Repr::*, UBig},
};
use core::{
    cmp, mem,
    ops::{Add, AddAssign},
};

impl Add<UBig> for UBig {
    type Output = UBig;

    #[inline]
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

    #[inline]
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

    #[inline]
    fn add(self, rhs: UBig) -> UBig {
        rhs.add(self)
    }
}

impl Add<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
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
    #[inline]
    fn add_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&UBig> for UBig {
    #[inline]
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
        let n = cmp::min(buffer.len(), rhs.len());
        let overflow = add_words_same_len_in_place(&mut buffer[..n], &rhs[..n]);
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

/// Add one to a non-empty word sequence.
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
fn add_words_same_len_in_place(words: &mut [Word], rhs: &[Word]) -> bool {
    debug_assert!(words.len() == rhs.len());

    let mut carry = 0;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        let (sum, c) = split_double_word(extend_word(*a) + extend_word(*b) + extend_word(carry));
        *a = sum;
        carry = c;
    }
    carry != 0
}
