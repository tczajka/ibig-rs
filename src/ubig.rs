//! Unsigned big integers.

use self::word::Word;

use alloc::vec::Vec;

mod word;

/// An unsigned big integer.
pub struct UBig(UBigRepr);

/// Internal representation of UBig.
enum UBigRepr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    Large(Vec<Word>),
}
