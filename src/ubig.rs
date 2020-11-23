//! Unsigned big integers.

use self::{normalize::NormalizedBuffer, word::Word};
pub use radix::InRadix;

mod buffer;
mod convert;
mod memory;
mod normalize;
mod radix;
mod word;

/// Internal representation of UBig.
#[derive(Debug, Eq, PartialEq)]
enum Repr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    Large(NormalizedBuffer),
}

/// An unsigned big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct UBig(Repr);
