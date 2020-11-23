//! Big integer library.

#![no_std]

extern crate alloc;

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
    Small(crate::word::Word),
    /// A number that does not fit in a single Word.
    Large(crate::normalize::NormalizedBuffer),
}

/// Unsigned big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct UBig(Repr);

/// Sign of IBig.
#[derive(Debug, Eq, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IBig {
    sign: Sign,
    magnitude: UBig,
}
