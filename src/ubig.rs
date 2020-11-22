//! Unsigned big integers.

use self::{normalize::NormalizedBuffer, word::Word};

mod buffer;
mod convert;
mod normalize;
mod word;

/// Internal representation of UBig.
#[derive(Debug, Eq, PartialEq)]
enum Repr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    Large(NormalizedBuffer),
}

use Repr::*;

/// An unsigned big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct UBig(Repr);

impl UBig {
    fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }
}

impl Clone for UBig {
    fn clone(&self) -> UBig {
        match self.0 {
            Small(x) => UBig(Small(x)),
            Large(ref large) => UBig(Large(large.clone())),
        }
    }

    fn clone_from(&mut self, other: &UBig) {
        if let Large(ref mut large) = self.0 {
            if let Large(ref other_large) = other.0 {
                large.clone_from(other_large);
                return;
            }
        }
        *self = other.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::buffer::Buffer;
    use super::*;

    #[test]
    fn test_from_word() {
        assert_eq!(UBig::from_word(5), UBig(Small(5)));
    }

    #[test]
    fn test_clone() {
        let a = UBig::from_word(5);
        assert_eq!(a.clone(), a);

        let mut buf = Buffer::allocate(5);
        buf.push(1);
        buf.push(2);
        buf.push(3);
        let a: UBig = buf.into();
        assert_eq!(a.clone(), a);
    }

    fn buffer_capacity(x: &UBig) -> usize {
        match x.0 {
            Small(_) => 1,
            Large(ref large) => large.capacity(),
        }
    }

    fn gen_large(num_words: usize) -> UBig {
        let mut buf = Buffer::allocate(num_words);
        for i in 0..num_words {
            buf.push(i);
        }
        buf.into()
    }

    #[test]
    fn test_clone_from() {
        let num: UBig = gen_large(10);

        let mut a = UBig::from_word(3);
        a.clone_from(&num);
        assert_eq!(a, num);
        let b = UBig::from_word(7);
        a.clone_from(&b);
        assert_eq!(a, b);
        a.clone_from(&b);
        assert_eq!(a, b);

        let mut a = gen_large(9);
        let prev_cap = buffer_capacity(&a);
        a.clone_from(&num);
        // The buffer should be reused, 9 is close enough to 10.
        assert!(buffer_capacity(&a) == prev_cap);
        assert!(buffer_capacity(&a) != buffer_capacity(&num));

        let mut a = gen_large(2);
        let prev_cap = buffer_capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too small.
        assert!(buffer_capacity(&a) != prev_cap);
        assert!(buffer_capacity(&a) == buffer_capacity(&num));

        let mut a = gen_large(100);
        let prev_cap = buffer_capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too large.
        assert!(buffer_capacity(&a) != prev_cap);
        assert!(buffer_capacity(&a) == buffer_capacity(&num));
    }
}
