//! Allocation of UBig storage on the heap.

use super::{
    UBig,
    UBigRepr::{Large, Small},
    word::{Word, WORD_BITS},
};
use alloc::vec::Vec;
use core::cmp::min;

/// Maximum number of Words in a UBig.
///
/// It ensures that the number of bits fits in usize, which is useful with bit count operations,
/// and for radix conversions.
///
/// It also ensures that we can add two lengths without overflow.
const MAX_UBIG_CAPACITY: usize = usize::MAX / WORD_BITS;

/// Maximum capacity for a given number of words.
///
/// Requires that num_words <= MAX_UBIG_CAPACITY.
///
/// Allows approximately 25% overhead.
fn max_ubig_capacity(num_words: usize) -> usize {
    debug_assert!(num_words <= MAX_UBIG_CAPACITY);
    min(num_words + num_words / 4 + 4, MAX_UBIG_CAPACITY)
}

/// Default capacity for a given number of words. Should be between num_words and
/// max_ubig_capacity(num_words).
///
/// Requires that num_words <= MAX_UBIG_CAPACITY.
///
/// Approximately 12.5% overhead.
fn default_ubig_capacity(num_words: usize) -> usize {
    debug_assert!(num_words <= MAX_UBIG_CAPACITY);
    min(num_words + num_words / 8 + 2, MAX_UBIG_CAPACITY)
}

/// Allocate words on the heap with a bit of spare space.
pub fn allocate_words(num_words: usize) -> Vec<Word> {
    if num_words > MAX_UBIG_CAPACITY {
        panic!("UBig too large");
    }
    Vec::with_capacity(default_ubig_capacity(num_words))
}

impl UBig {
    /// Create UBig from one Word.
    pub(in crate::ubig) fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Create UBig from a Vec of Words.
    pub(in crate::ubig) fn from_words(mut words: Vec<Word>) -> UBig {
        while let Some(&0) = words.last() {
            words.pop();
        }
        match words.len() {
            0 => UBig::from_word(0),
            1 => UBig::from_word(words[0]),
            n => {
                if words.capacity() <= max_ubig_capacity(n) {
                    UBig::from_words_normalized_correct_capacity(words)
                } else {
                    let mut shorter_words = allocate_words(n);
                    shorter_words.clone_from(&words);
                    UBig::from_words_normalized_correct_capacity(shorter_words)
                }
            }
        }
    }

    /// Create UBig from a Vec of at least 2 Words with no leading zeros and with valid capacity.
    pub(in crate::ubig) fn from_words_normalized_correct_capacity(words: Vec<Word>) -> UBig {
        debug_assert!(words.len() >= 2);
        debug_assert!(*words.last().unwrap() != 0);
        debug_assert!(words.capacity() <= max_ubig_capacity(words.len()));
        UBig(Large(words))
    }

    /// Get the capacity in words.
    #[cfg(test)]
    fn capacity(&self) -> usize {
        match self.0 {
            Small(_) => 1,
            Large(ref words) => words.capacity(),
        }
    }
}

impl Clone for UBig {
    fn clone(&self) -> UBig {
        match self.0 {
            Small(x) => UBig::from_word(x),
            Large(ref words) => {
                let mut words_clone = allocate_words(words.len());
                words_clone.clone_from(&words);
                UBig::from_words_normalized_correct_capacity(words_clone)
            }
        }
    }

    fn clone_from(&mut self, other: &UBig) {
        if let Large(ref mut words) = self.0 {
            if let Large(ref other_words) = other.0 {
                let cap = words.capacity();
                let n = other_words.len();
                if cap >= n && cap <= max_ubig_capacity(n) {
                    // Reuse buffer.
                    words.clone_from(other_words);
                    return;
                }
            }
        }
        *self = other.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity() {
        assert_eq!(max_ubig_capacity(2), 6);
        assert_eq!(default_ubig_capacity(2), 4);
        assert_eq!(max_ubig_capacity(1000), 1254);
        assert_eq!(default_ubig_capacity(1000), 1127);
    }

    #[test]
    fn test_allocate_words() {
        let words = allocate_words(1000);
        assert_eq!(words.len(), 0);
        assert_eq!(words.capacity(), 1127);
    }

    #[test]
    #[should_panic]
    fn test_allocate_words_too_large() {
        let _ = allocate_words(MAX_UBIG_CAPACITY + 1);
    }

    #[test]
    fn test_from_words() {
        let num = UBig::from_word(5);
        assert_eq!(num, UBig(Small(5)));
        let num = UBig::from_words(vec![5, 6]);
        assert_eq!(num, UBig(Large(vec![5, 6])));
        let num = UBig::from_words(vec![5, 6, 0, 0, 0]);
        assert_eq!(num, UBig(Large(vec![5, 6])));
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
