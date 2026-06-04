//! Tests of the `const` byte constructors.

use crate::{IBig, UBig};

#[test]
fn ubig_const_from_le_bytes() {
    // A runtime call exercises the instrumented body (the `from_uN` constructors only reach
    // it through their multi-digit fallback, which is dead on word sizes where the value
    // always fits a single digit). Inputs stay within `INLINE_DIGITS * Digit::BYTES`, which
    // is 8 at the 16-bit word size.
    assert_eq!(
        UBig::const_from_le_bytes(&[1, 2, 3, 4, 5, 6, 7, 8]),
        UBig::from_le_bytes(&[1, 2, 3, 4, 5, 6, 7, 8])
    );
    // Most-significant zero bytes still normalize.
    assert_eq!(
        UBig::const_from_le_bytes(&[0xff, 0x00, 0x10]),
        UBig::from_le_bytes(&[0xff, 0x00, 0x10])
    );
}

#[test]
fn ibig_const_from_le_bytes() {
    // Positive: the top byte's high bit is clear.
    assert_eq!(
        IBig::const_from_le_bytes(&[1, 2, 3, 4, 5, 6, 7, 8]),
        IBig::from_le_bytes(&[1, 2, 3, 4, 5, 6, 7, 8])
    );
    // Negative: the top byte's high bit is set.
    assert_eq!(
        IBig::const_from_le_bytes(&[0x00, 0x80]),
        IBig::from_le_bytes(&[0x00, 0x80])
    );
}
