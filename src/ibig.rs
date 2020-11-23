//! Signed big integer.

use crate::ubig::UBig;

/// Sign of IBig.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

/// Signed big integer.
#[derive(Debug, Eq, PartialEq)]
pub struct IBig {
    sign: Sign,
    magnitude: UBig,
}
