/// Digit and radix type.
pub(crate) type Digit = u32;

/// Maximum supported radix.
pub(crate) const MAX_RADIX: Digit = 36;

/// Lower-case or upper-case digits: a-z or A-Z.
#[derive(Clone, Copy)]
pub(crate) enum DigitCase {
    Lower,
    Upper,
}

/// Convert DigitCase to ASCII representation of 10.
pub(crate) fn digit_case_to_ascii10(digit_case: DigitCase) -> u8 {
    match digit_case {
        DigitCase::Lower => b'a',
        DigitCase::Upper => b'A',
    }
}

/// Converts a `Digit` to its ASCII representation.
///
/// `ascii10` is the ASCII representation of 10. Should be `b'a'` or `b'A'`.
pub(crate) fn digit_to_ascii(digit: Digit, ascii10: u8) -> u8 {
    debug_assert!(digit < MAX_RADIX);
    let digit = digit as u8;
    if digit < 10 {
        digit + b'0'
    } else {
        digit - 10 + ascii10
    }
}
