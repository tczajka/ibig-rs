use crate::ubig::word::{Word, WORD_BITS};

use alloc::vec::Vec;
use core::{cmp::min, ops::Deref};

/// Buffer for Words.
///
/// UBig operations are usually performed by creating a Buffer with appropriate capacity, filling it
/// in with Words, and then converting to UBig.
///
/// If its capacity is exceeded, the `Buffer` will panic.
#[derive(Debug, Eq, PartialEq)]
pub struct Buffer(Vec<Word>);

impl Buffer {
    /// Creates a `Buffer` with at least specified capacity.
    ///
    /// It leaves some extra space for future growth.
    pub fn allocate(num_words: usize) -> Buffer {
        if num_words > Buffer::MAX_CAPACITY {
            panic!("Buffer too large");
        }
        Buffer(Vec::with_capacity(Buffer::default_capacity(num_words)))
    }

    /// Ensure there is enough capacity in the buffer for `num_words`. Will reallocate if there is
    /// not enough.
    pub fn ensure_capacity(&mut self, num_words: usize) {
        if num_words > self.capacity() {
            self.reallocate(num_words);
        }
    }

    /// Change capacity to store `num_words` plus some extra space for future growth.
    ///
    /// Panics if `num_words < len()`.
    pub fn reallocate(&mut self, num_words: usize) {
        if num_words < self.len() {
            panic!("New capacity too small to fit data in Buffer");
        }
        let mut new_buffer = Buffer::allocate(num_words);
        new_buffer.clone_from(self);
        *self = new_buffer
    }

    /// Append a Word to the buffer.
    ///
    /// Panics if there is not enough capacity.
    pub fn push(&mut self, word: Word) {
        if self.len() >= self.capacity() {
            panic!("Buffer capacity exceeded");
        }
        self.0.push(word)
    }

    /// Pop the most significant `Word`.
    pub fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Maximum number of `Word`s.
    ///
    /// It ensures that the number of bits fits in `usize`, which is useful for bit count
    /// operations, and for radix conversions (even base 2 can be represented).
    ///
    /// It also ensures that we can add two lengths without overflow.
    pub const MAX_CAPACITY: usize = usize::MAX / WORD_BITS;

    /// Default capacity for a given number of `Word`s.
    /// It should be between `num_words` and `max_capacity(num_words).
    ///
    /// Requires that `num_words <= MAX_CAPACITY`.
    ///
    /// Provides `2 + 0.125 * num_words` extra space.
    fn default_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= Buffer::MAX_CAPACITY);
        min(num_words + num_words / 8 + 2, Buffer::MAX_CAPACITY)
    }
}

impl Clone for Buffer {
    /// New buffer will be sized as `Buffer::allocate(self.len())`.
    fn clone(&self) -> Buffer {
        let mut new_buffer = Buffer::allocate(self.len());
        new_buffer.clone_from(self);
        new_buffer
    }

    /// If capacity is exceeded, panic.
    fn clone_from(&mut self, other: &Buffer) {
        if self.capacity() < other.len() {
            panic!("Buffer capacity exceeded");
        }
        self.0.clone_from(&other.0);
    }
}

impl Deref for Buffer {
    type Target = Vec<Word>;

    fn deref(&self) -> &Vec<Word> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capacity() {
        assert_eq!(Buffer::default_capacity(2), 4);
        assert_eq!(Buffer::default_capacity(1000), 1127);
    }

    #[test]
    fn test_allocate() {
        let buffer = Buffer::allocate(1000);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), Buffer::default_capacity(1000));
    }

    #[test]
    #[should_panic]
    fn test_allocate_too_large() {
        let _ = Buffer::allocate(Buffer::MAX_CAPACITY + 1);
    }

    #[test]
    fn test_ensure_capacity() {
        let mut buffer = Buffer::allocate(2);
        buffer.push(7);
        assert_eq!(buffer.capacity(), 4);
        buffer.ensure_capacity(4);
        assert_eq!(buffer.capacity(), 4);
        buffer.ensure_capacity(5);
        assert_eq!(buffer.capacity(), 7);
        assert_eq!(&buffer[..], &[7]);
    }

    #[test]
    fn test_shrink_capacity() {
        let mut buffer = Buffer::allocate(100);
        buffer.push(7);
        buffer.reallocate(2);
        assert_eq!(buffer.capacity(), Buffer::default_capacity(2));
        assert_eq!(&buffer[..], [7]);
    }

    #[test]
    fn test_operations() {
        let mut buffer = Buffer::allocate(5);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(&buffer[..], [1, 2]);
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    #[should_panic]
    fn test_push_failed() {
        let mut buffer = Buffer::allocate(2);
        for _ in 0..10 {
            buffer.push(7);
        }
    }

    #[test]
    fn test_clone() {
        let mut buffer = Buffer::allocate(100);
        buffer.push(7);
        buffer.push(8);
        let buffer2 = buffer.clone();
        assert_eq!(buffer, buffer2);
        assert_eq!(buffer2.capacity(), Buffer::default_capacity(2));
    }

    #[test]
    fn test_clone_from() {
        let mut buffer = Buffer::allocate(100);
        buffer.push(7);
        buffer.push(8);
        let mut buffer2 = Buffer::allocate(50);
        buffer2.clone_from(&buffer);
        assert_eq!(buffer, buffer2);
        assert_eq!(buffer2.capacity(), Buffer::default_capacity(50));
    }
}
