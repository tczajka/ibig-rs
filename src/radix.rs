//! Printing and parsing in any radix.

use crate::{ibig::Sign::{self, *}, ubig::UBig};

/// Representation of `UBig` or `IBig` in some radix between 2 and 36.
// TODO: Document with an example (lower-case and upper-case).
// TODO: Make it support both UBig and IBig. Change num to IBig.
#[derive(Debug)]
pub struct InRadix<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: u8,
}

/*
TODO:
impl<'a> Display for InRadix<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    }
}
*/

/*
   TODO:
   impl Display for UBig
   impl Debug for UBig
   impl Binary for UBig
   impl Octal for UBig
   impl LowerHex for UBig
   impl UpperHex for UBig
*/

impl UBig {
    /// Representation in a given radix.
    ///
    /// `radix` must be between 2 and 36 inclusive.
    #[inline]
    pub fn in_radix(&self, radix: u8) -> InRadix {
        assert!(
            (2..37).contains(&radix),
            "radix must be between 2 and 36 inclusive"
        );
        InRadix { sign: Positive, magnitude: self, radix }
    }

    /*
    /// Equivalent to `self.to_radix(radix).to_string()` but more efficient.
    // TODO: show example
    pub fn to_radix_str(&self, radix: u8) {
    }
    */

    /*
    /// Equivalent to `self.to_radix(radix).to_string()` but more efficient.
    // TODO: show example
    pub fn to_radix_str_uppercase(&self, radix: u8) {
    }
    */
}
