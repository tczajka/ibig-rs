//! Error types.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// The error returned when a big integer ([`UBig`](crate::UBig) or [`IBig`](crate::IBig))
/// is out of range for the target type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TryFromBigError;

impl Display for TryFromBigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("number out of range for the target type")
    }
}

impl Error for TryFromBigError {}
