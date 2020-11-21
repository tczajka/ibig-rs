//! Large (at least 2 words) unsigned integers allocated on the heap.

use super::{
    word::{Word, WORD_BITS},
    UBig,
    UBigRepr::*,
};
use alloc::vec::Vec;
use core::cmp::min;

#[derive(Debug, Eq, PartialEq)]
pub struct LargeUBig {
    words: Vec<Word>,
}

impl LargeUBig {
    /// Maximum number of Words.
    ///
    /// It ensures that the number of bits fits in usize, which is useful with bit count operations,
    /// and for radix conversions.
    ///
    /// It also ensures that we can add two lengths without overflow.
    const MAX_CAPACITY: usize = usize::MAX / WORD_BITS;

    /// Maximum capacity for a given number of words.
    ///
    /// Requires that num_words <= MAX_UBIG_CAPACITY.
    ///
    /// Allows approximately 25% overhead.
    fn max_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= LargeUBig::MAX_CAPACITY);
        min(num_words + num_words / 4 + 4, LargeUBig::MAX_CAPACITY)
    }

    /// Default capacity for a given number of words. Should be between num_words and
    /// max_capacity(num_words).
    ///
    /// Requires that num_words <= MAX_CAPACITY.
    ///
    /// Approximately 12.5% overhead.
    fn default_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= LargeUBig::MAX_CAPACITY);
        min(num_words + num_words / 8 + 2, LargeUBig::MAX_CAPACITY)
    }

    /// Create LargeUBig from a Vec of at least 2 Words with no leading zeros and with valid
    /// capacity.
    fn from_words_normalized_correct_capacity(words: Vec<Word>) -> LargeUBig {
        debug_assert!(words.len() >= 2);
        debug_assert!(*words.last().unwrap() != 0);
        debug_assert!(words.capacity() <= LargeUBig::max_capacity(words.len()));
        LargeUBig { words }
    }

    /// Get the capacity in words.
    #[cfg(test)]
    pub fn capacity(&self) -> usize {
        self.words.capacity()
    }
}

impl Clone for LargeUBig {
    fn clone(&self) -> LargeUBig {
        let mut words_clone = UBig::allocate_words(self.words.len());
        words_clone.clone_from(&self.words);
        LargeUBig::from_words_normalized_correct_capacity(words_clone)
    }

    fn clone_from(&mut self, other: &LargeUBig) {
        let cap = self.words.capacity();
        let n = other.words.len();
        if cap >= n && cap <= LargeUBig::max_capacity(n) {
            // Reuse buffer.
            self.words.clone_from(&other.words);
        } else {
            *self = other.clone();
        }
    }
}

impl UBig {
    /// Allocate words on the heap to hold at least num_words.
    ///
    /// If the final number of words passed to from_words is between num_words-2 and num_words
    /// inclusive, there will not be a reallocation.
    pub(super) fn allocate_words(num_words: usize) -> Vec<Word> {
        if num_words > LargeUBig::MAX_CAPACITY {
            panic!("UBig too large");
        }
        Vec::with_capacity(LargeUBig::default_capacity(num_words))
    }

    /// Create UBig from a Vec of Words.
    pub(super) fn from_words(mut words: Vec<Word>) -> UBig {
        while let Some(&0) = words.last() {
            words.pop();
        }
        match words.len() {
            0 => UBig::from_word(0),
            1 => UBig::from_word(words[0]),
            n => {
                let new_words = if words.capacity() <= LargeUBig::max_capacity(n) {
                    words
                } else {
                    let mut shorter_words = UBig::allocate_words(n);
                    shorter_words.clone_from(&words);
                    shorter_words
                };
                UBig(Large(LargeUBig::from_words_normalized_correct_capacity(
                    new_words,
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity() {
        assert_eq!(LargeUBig::max_capacity(2), 6);
        assert_eq!(LargeUBig::default_capacity(2), 4);
        assert_eq!(LargeUBig::max_capacity(1000), 1254);
        assert_eq!(LargeUBig::default_capacity(1000), 1127);
    }

    #[test]
    fn test_allocate_words() {
        let words = UBig::allocate_words(1000);
        assert_eq!(words.len(), 0);
        assert_eq!(words.capacity(), 1127);
    }

    #[test]
    #[should_panic]
    fn test_allocate_words_too_large() {
        let _ = UBig::allocate_words(LargeUBig::MAX_CAPACITY + 1);
    }

    #[test]
    fn test_from_words() {
        let num = UBig::from_words(vec![5, 6]);
        assert_eq!(num, UBig(Large(LargeUBig { words: vec![5, 6] })));
        let num = UBig::from_words(vec![5, 6, 0, 0, 0]);
        assert_eq!(num, UBig(Large(LargeUBig { words: vec![5, 6] })));
        let num = UBig::from_words(vec![5, 0, 0, 0, 0]);
        assert_eq!(num, UBig(Small(5)));
        let num = UBig::from_words(vec![]);
        assert_eq!(num, UBig(Small(0)));

        let mut v = Vec::with_capacity(10);
        for _ in 0..9 {
            v.push(5);
        }
        let num = UBig::from_words(v);
        assert_eq!(num.capacity(), 10);

        let mut v = Vec::with_capacity(10);
        for _ in 0..2 {
            v.push(5);
        }
        for _ in 2..9 {
            v.push(0);
        }
        let num = UBig::from_words(v);
        assert_eq!(num.capacity(), 4);
    }

    #[test]
    fn test_clone() {
        let num = UBig::from_word(5);
        assert_eq!(num.clone(), num);

        let num = UBig::from_words(vec![5, 6]);
        let num2 = num.clone();
        assert_eq!(num2, num);
        assert_eq!(num.capacity(), 2);
        assert_eq!(num2.capacity(), 4);
    }

    #[test]
    fn test_clone_from() {
        let num = UBig::from_word(7);
        let mut num2 = UBig::from_word(8);
        num2.clone_from(&num);
        assert_eq!(num2, num);

        let num = UBig::from_words(vec![5, 5, 5, 5, 5, 5, 5, 5, 5]);
        let mut num2 = UBig::from_words(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
        num2.clone_from(&num);
        assert_eq!(num2.capacity(), 10);

        let num = UBig::from_words(vec![5, 5]);
        num2.clone_from(&num);
        assert_eq!(num2.capacity(), 4);
    }
}
