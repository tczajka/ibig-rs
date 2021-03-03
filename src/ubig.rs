//! Unsigned big integer.

use self::Repr::*;
use crate::{buffer::Buffer, primitive::Word};

/// Internal representation of UBig.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Repr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    ///
    /// The buffer has:
    /// * length at least 2
    /// * no leading zero
    /// * compact capacity
    Large(Buffer),
}

/// Unsigned big integer.
///
/// Arbitrarily large unsigned integer.
///
/// # Examples
///
/// ```
/// # use ibig::{prelude::*, ParseError};
/// let a = ubig!(a2a123bbb127779cccc123123ccc base 32);
/// let b = ubig!(0x1231abcd4134);
/// let c = UBig::from_str_radix("a2a123bbb127779cccc123123ccc", 32)?;
/// let d = UBig::from_str_radix("1231abcd4134", 16)?;
/// assert_eq!(a, c);
/// assert_eq!(b, d);
/// # Ok::<(), ParseError>(())
/// ```
#[derive(Eq, PartialEq)]
pub struct UBig(Repr);

impl UBig {
    /// Construct from one word.
    pub(crate) fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Get the representation of UBig.
    pub(crate) fn repr(&self) -> &Repr {
        &self.0
    }

    /// Convert into representation.
    pub(crate) fn into_repr(self) -> Repr {
        self.0
    }
}

impl Clone for UBig {
    fn clone(&self) -> UBig {
        match self.repr() {
            Small(x) => UBig(Small(*x)),
            Large(buffer) => UBig(Large(buffer.clone())),
        }
    }

    fn clone_from(&mut self, source: &UBig) {
        if let Large(buffer) = &mut self.0 {
            if let Large(source_buffer) = source.repr() {
                buffer.resizing_clone_from(source_buffer);
                return;
            }
        }
        *self = source.clone();
    }
}

impl From<Buffer> for UBig {
    /// If the Buffer was allocated with `Buffer::allocate(n)`
    /// and the normalized length is between `n - 2` and `n + 2`
    /// (or even approximately between `0.9 * n` and `1.125 * n`),
    /// there will be no reallocation here.
    fn from(mut buffer: Buffer) -> UBig {
        buffer.pop_leading_zeros();
        match buffer.len() {
            0 => UBig::from_word(0),
            1 => UBig::from_word(buffer[0]),
            _ => {
                buffer.shrink();
                UBig(Large(buffer))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Current capacity in Words.
    fn capacity(x: &UBig) -> usize {
        match x.repr() {
            Small(_) => 1,
            Large(large) => large.capacity(),
        }
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
        assert_eq!(capacity(&num), 7);

        let mut buf = Buffer::allocate(100);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        let num: UBig = buf.into();
        assert_eq!(capacity(&num), 6);
    }

    #[test]
    fn test_clone() {
        let a = UBig::from_word(5);
        assert_eq!(a.clone(), a);

        let a = gen_ubig(10);
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(capacity(&a), capacity(&b));
    }

    #[test]
    fn test_clone_from() {
        let num: UBig = gen_ubig(10);

        let mut a = UBig::from_word(3);
        a.clone_from(&num);
        assert_eq!(a, num);
        let b = UBig::from_word(7);
        a.clone_from(&b);
        assert_eq!(a, b);
        a.clone_from(&b);
        assert_eq!(a, b);

        let mut a = gen_ubig(9);
        let prev_cap = capacity(&a);
        a.clone_from(&num);
        // The buffer should be reused, 9 is close enough to 10.
        assert_eq!(capacity(&a), prev_cap);
        assert_ne!(capacity(&a), capacity(&num));

        let mut a = gen_ubig(2);
        let prev_cap = capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too small.
        assert_ne!(capacity(&a), prev_cap);
        assert_eq!(capacity(&a), capacity(&num));

        let mut a = gen_ubig(100);
        let prev_cap = capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too large.
        assert_ne!(capacity(&a), prev_cap);
        assert_eq!(capacity(&a), capacity(&num));
    }

    fn gen_ubig(num_words: usize) -> UBig {
        let mut buf = Buffer::allocate(num_words);
        for i in 0..num_words {
            buf.push(i);
        }
        buf.into()
    }
}
