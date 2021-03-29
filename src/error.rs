use core::fmt::{self, Display, Formatter};

/// Number out of bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("number out of bounds")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OutOfBoundsError {}

/// Parse error when parsing `UBig` or `IBig`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// There were no digits in the string.
    NoDigits,
    /// Invalid digit for a given radix.
    InvalidDigit,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::NoDigits => f.write_str("no digits"),
            ParseError::InvalidDigit => f.write_str("invalid digit"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
