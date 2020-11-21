//! Unsigned big integers.

use self::{large::LargeUBig, word::Word};

mod convert;
mod large;
mod word;

/// Internal representation of UBig.
#[derive(Debug, Eq, PartialEq)]
enum UBigRepr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    Large(LargeUBig),
}

use UBigRepr::*;

impl Clone for UBigRepr {
    fn clone(&self) -> UBigRepr {
        match *self {
            Small(x) => Small(x),
            Large(ref large) => Large(large.clone()),
        }
    }

    fn clone_from(&mut self, other: &UBigRepr) {
        if let Large(ref mut large) = *self {
            if let Large(ref other_large) = *other {
                large.clone_from(other_large);
                return;
            }
        }
        *self = other.clone();
    }
}

/// An unsigned big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct UBig(UBigRepr);

impl UBig {
    /// Create UBig from one Word.
    fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Get the capacity in words.
    #[cfg(test)]
    fn capacity(&self) -> usize {
        match self.0 {
            Small(_) => 1,
            Large(ref large) => large.capacity(),
        }
    }
}

impl Clone for UBig {
    fn clone(&self) -> UBig {
        UBig(self.0.clone())
    }

    fn clone_from(&mut self, other: &UBig) {
        self.0.clone_from(&other.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_word() {
        let num = UBig::from_word(5);
        assert_eq!(num, UBig(Small(5)));
    }
}
