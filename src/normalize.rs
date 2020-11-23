use core::{cmp::min, ops::Deref};

use crate::{buffer::Buffer, Repr::*, UBig};

/// Normalized buffer.
///
/// Normalization means:
/// * at least 2 words
/// * most significant word non-zero
/// * capacity no larger than approximately 25% overhead
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct NormalizedBuffer(Buffer);

impl NormalizedBuffer {
    /// Maximum capacity for a given number of `Word`s.
    ///
    /// Requires that `num_words <= Buffer::MAX_CAPACITY`.
    ///
    /// Allows `4 + 0.25 * num_words` overhead.
    fn max_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= Buffer::MAX_CAPACITY);
        min(num_words + num_words / 4 + 4, Buffer::MAX_CAPACITY)
    }
}

/// Checks whether a Buffer is normalized.
fn is_normalized(buffer: &Buffer) -> bool {
    buffer.len() >= 2
        && *buffer.last().unwrap() != 0
        && buffer.capacity() <= NormalizedBuffer::max_capacity(buffer.len())
}

impl Clone for NormalizedBuffer {
    fn clone(&self) -> NormalizedBuffer {
        let buffer = self.0.clone();
        debug_assert!(is_normalized(&buffer));
        NormalizedBuffer(buffer)
    }

    fn clone_from(&mut self, other: &NormalizedBuffer) {
        let cap = self.capacity();
        let n = other.len();
        if cap >= n && cap <= NormalizedBuffer::max_capacity(n) {
            // Reuse buffer.
            self.0.clone_from(&other.0);
            debug_assert!(is_normalized(&self.0));
        } else {
            *self = other.clone();
        }
    }
}

impl Deref for NormalizedBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Buffer {
        &self.0
    }
}

impl From<Buffer> for UBig {
    /// If the Buffer was allocated with `Buffer::allocate(n)` or `Buffer::reallocate(n)`,
    /// and the normalized length is between `n - 2` and `n + 2`
    /// (or even approximately between `0.9 * n` and `1.125 * n`),
    /// there will be no reallocation here.
    fn from(mut buffer: Buffer) -> UBig {
        while let Some(0) = buffer.last() {
            buffer.pop();
        }
        match buffer.len() {
            0 => UBig::from_word(0),
            1 => UBig::from_word(buffer[0]),
            n => {
                if buffer.capacity() > NormalizedBuffer::max_capacity(n) {
                    buffer.reallocate(n);
                }
                debug_assert!(is_normalized(&buffer));
                UBig(Large(NormalizedBuffer(buffer)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_capacity() {
        assert_eq!(NormalizedBuffer::max_capacity(2), 6);
        assert_eq!(NormalizedBuffer::max_capacity(1000), 1254);
    }

    #[test]
    fn test_buffer_to_ubig() {
        let buf = Buffer::allocate(5);
        let num: UBig = buf.into();
        assert_eq!(num, UBig::from_word(0));

        let mut buf = Buffer::allocate(5);
        buf.push(7);
        let num: UBig = buf.into();
        assert_eq!(num, UBig::from_word(7));

        let mut buf = Buffer::allocate(100);
        buf.push(7);
        buf.push(0);
        buf.push(0);
        let num: UBig = buf.into();
        assert_eq!(num, UBig::from_word(7));

        let mut buf = Buffer::allocate(5);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        let num: UBig = buf.into();
        assert_eq!(num.capacity(), 7);

        let mut buf = Buffer::allocate(100);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        let num: UBig = buf.into();
        assert_eq!(num.capacity(), 6);
    }
}
