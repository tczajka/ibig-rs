//! Unsigned big integers.

use crate::word::Word;
use alloc::vec::Vec;

mod allocate;
mod convert;

/// An unsigned big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct UBig(UBigRepr);

/// Internal representation of UBig.
#[derive(Debug, Eq, PartialEq)]
enum UBigRepr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    Large(Vec<Word>),
}
