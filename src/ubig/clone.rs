use crate::ubig::{Repr::*, UBig};

impl Clone for UBig {
    #[inline]
    fn clone(&self) -> UBig {
        match self.0 {
            Small(x) => UBig(Small(x)),
            Large(ref large) => UBig(Large(large.clone())),
        }
    }

    #[inline]
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
    use super::*;
    use crate::ubig::buffer::Buffer;

    #[test]
    fn test_clone() {
        let a = UBig::from_word(5);
        assert_eq!(a.clone(), a);

        let mut buf = Buffer::allocate(10);
        for i in 0..9 {
            buf.push(i);
        }
        let a: UBig = buf.into();
        let b = a.clone();
        assert_eq!(a, b);
        assert_ne!(buffer_capacity(&a), buffer_capacity(&b));
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
        assert_eq!(buffer_capacity(&a), prev_cap);
        assert_ne!(buffer_capacity(&a), buffer_capacity(&num));

        let mut a = gen_large(2);
        let prev_cap = buffer_capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too small.
        assert_ne!(buffer_capacity(&a), prev_cap);
        assert_eq!(buffer_capacity(&a), buffer_capacity(&num));

        let mut a = gen_large(100);
        let prev_cap = buffer_capacity(&a);
        a.clone_from(&num);
        // The buffer should now be reallocated, it's too large.
        assert_ne!(buffer_capacity(&a), prev_cap);
        assert_eq!(buffer_capacity(&a), buffer_capacity(&num));
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
}
