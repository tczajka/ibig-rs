//! Word buffer.

use crate::primitive::{Word, WORD_BITS};

use alloc::vec::Vec;
use core::{
    cmp::min,
    ops::{Deref, DerefMut},
};

/// Buffer for Words.
///
/// UBig operations are usually performed by creating a Buffer with appropriate capacity, filling it
/// in with Words, and then converting to UBig.
///
/// If its capacity is exceeded, the `Buffer` will panic.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct Buffer(Vec<Word>);

impl Buffer {
    /// Creates a `Buffer` with at least specified capacity.
    ///
    /// It leaves some extra space for future growth.
    pub(crate) fn allocate(num_words: usize) -> Buffer {
        if num_words > Buffer::MAX_CAPACITY {
            Buffer::too_large();
        }
        Buffer(Vec::with_capacity(Buffer::default_capacity(num_words)))
    }

    pub(crate) fn too_large() -> ! {
        panic!(
            "number too large, maximum is {} bits",
            Buffer::MAX_CAPACITY * (WORD_BITS as usize)
        )
    }

    /// Ensure there is enough capacity in the buffer for `num_words`. Will reallocate if there is
    /// not enough.
    pub(crate) fn ensure_capacity(&mut self, num_words: usize) {
        if num_words > self.capacity() {
            self.reallocate(num_words);
        }
    }

    /// Makes sure that the capacity is compact.
    pub(crate) fn shrink(&mut self) {
        if self.capacity() > Buffer::max_compact_capacity(self.len()) {
            self.reallocate(self.len());
        }
    }

    /// Change capacity to store `num_words` plus some extra space for future growth.
    ///
    /// # Panics
    ///
    /// Panics if `num_words < len()`.
    fn reallocate(&mut self, num_words: usize) {
        assert!(num_words >= self.len());
        let mut new_buffer = Buffer::allocate(num_words);
        new_buffer.clone_from(self);
        *self = new_buffer
    }

    /// Return buffer capacity.
    pub(crate) fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Append a Word to the buffer.
    ///
    /// # Panics
    ///
    /// Panics if there is not enough capacity.
    pub(crate) fn push(&mut self, word: Word) {
        assert!(self.len() < self.capacity());
        self.0.push(word)
    }

    /// Append `n` zeros.
    ///
    /// # Panics
    ///
    /// Panics if there is not enough capacity.
    pub(crate) fn push_zeros(&mut self, n: usize) {
        assert!(n <= self.capacity() - self.len());
        self.0.extend(core::iter::repeat(0).take(n));
    }

    /// Pop the most significant `Word`.
    pub(crate) fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Truncate length to `len`.
    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(self.len() >= len);

        self.0.truncate(len);
    }

    /// Clone from `other` and resize if necessary.
    ///
    /// Equivalent to, but more efficient than:
    ///
    /// ```ignore
    /// buffer.ensure_capacity(source.len());
    /// buffer.clone_from(source);
    /// buffer.shrink();
    /// ```
    pub(crate) fn resizing_clone_from(&mut self, source: &Buffer) {
        let cap = self.capacity();
        let n = source.len();
        if cap >= n && cap <= Buffer::max_compact_capacity(n) {
            self.clone_from(&source);
        } else {
            *self = source.clone();
        }
    }

    /// Maximum number of `Word`s.
    ///
    /// It ensures that the number of bits fits in `usize`, which is useful for bit count
    /// operations, and for radix conversions (even base 2 can be represented).
    ///
    /// It also ensures that we can add two lengths without overflow.
    pub(crate) const MAX_CAPACITY: usize = usize::MAX / (WORD_BITS as usize);

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

    /// Maximum compact capacity for a given number of `Word`s.
    ///
    /// Requires that `num_words <= Buffer::MAX_CAPACITY`.
    ///
    /// Allows `4 + 0.25 * num_words` overhead.
    fn max_compact_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= Buffer::MAX_CAPACITY);
        min(num_words + num_words / 4 + 4, Buffer::MAX_CAPACITY)
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
    fn clone_from(&mut self, source: &Buffer) {
        assert!(self.capacity() >= source.len());
        self.0.clone_from(&source.0);
    }
}

impl Deref for Buffer {
    type Target = [Word];

    fn deref(&self) -> &[Word] {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut [Word] {
        &mut self.0
    }
}

impl Extend<Word> for Buffer {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Word>,
    {
        for word in iter {
            self.push(word);
        }
    }
}

impl<'a> Extend<&'a Word> for Buffer {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a Word>,
    {
        for word in iter {
            self.push(*word);
        }
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
    fn test_max_compact_capacity() {
        assert_eq!(Buffer::max_compact_capacity(2), 6);
        assert_eq!(Buffer::max_compact_capacity(1000), 1254);
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
        assert_eq!(&buffer[..], [7]);
    }

    #[test]
    fn test_shrink() {
        let mut buffer = Buffer::allocate(100);
        buffer.push(7);
        buffer.push(8);
        buffer.shrink();
        assert_eq!(buffer.capacity(), Buffer::default_capacity(2));
        assert_eq!(&buffer[..], [7, 8]);
    }

    #[test]
    fn test_push_pop() {
        let mut buffer = Buffer::allocate(5);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(&buffer[..], [1, 2]);
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_extend() {
        let mut buffer = Buffer::allocate(5);
        buffer.push(1);
        let list: [Word; 2] = [2, 3];
        buffer.extend(&list);
        assert_eq!(&buffer[..], [1, 2, 3]);
    }

    #[test]
    fn test_push_zeros() {
        let mut buffer = Buffer::allocate(5);
        buffer.push(1);
        buffer.push_zeros(2);
        assert_eq!(&buffer[..], [1, 0, 0]);
    }

    #[test]
    fn test_truncate() {
        let mut buffer = Buffer::allocate(5);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.truncate(1);
        assert_eq!(&buffer[..], [1]);
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

    #[test]
    fn test_resizing_clone_from() {
        let mut buf = Buffer::allocate(5);
        assert_eq!(buf.capacity(), 7);

        let mut buf2 = Buffer::allocate(4);
        assert_eq!(buf2.capacity(), 6);
        for i in 0..4 {
            buf2.push(i);
        }
        buf.resizing_clone_from(&buf2);
        assert_eq!(buf.capacity(), 7);
        assert_eq!(&buf[..], [0, 1, 2, 3]);

        let mut buf3 = Buffer::allocate(100);
        for i in 0..100 {
            buf3.push(i);
        }
        buf.resizing_clone_from(&buf3);
        assert_eq!(buf.capacity(), Buffer::default_capacity(100));
        assert_eq!(buf.len(), 100);

        buf.resizing_clone_from(&buf2);
        assert_eq!(buf.capacity(), 6);
        assert_eq!(&buf[..], [0, 1, 2, 3]);
    }
}
