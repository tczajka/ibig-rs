use crate::primitive::{Word, WORD_BITS};

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

/// Converts an ASCII representation of a digit to its value.
pub(crate) fn digit_from_ascii(ascii: u8) -> Option<Digit> {
    match ascii {
        b'0'..=b'9' => Some((ascii - b'0') as Digit),
        b'a'..=b'z' => Some((ascii - b'a') as Digit + 10),
        b'A'..=b'Z' => Some((ascii - b'A') as Digit + 10),
        _ => None,
    }
}

/// Properties of a given radix in a `Word`.
#[derive(Clone, Copy)]
pub(crate) struct RadixInWord {
    /// Maximum number of digits that fit in a `Word`.
    pub(crate) max_digits: usize,
    /// Radix to the power of `max_digits`.
    /// Only for non-power-of-2 radi.
    pub(crate) max_digits_range: Word,
}

impl RadixInWord {
    const fn for_radix(radix: Digit) -> RadixInWord {
        if radix.is_power_of_two() {
            RadixInWord {
                max_digits: (WORD_BITS / radix.trailing_zeros()) as usize,
                max_digits_range: 0,
            }
        } else {
            RadixInWord::for_radix_recursive(
                radix,
                RadixInWord {
                    max_digits: 0,
                    max_digits_range: 1,
                },
            )
        }
    }

    const fn for_radix_recursive(radix: Digit, entry: RadixInWord) -> RadixInWord {
        match entry.max_digits_range.checked_mul(radix as Word) {
            None => entry,
            Some(max_digits_range) => RadixInWord::for_radix_recursive(
                radix,
                RadixInWord {
                    max_digits: entry.max_digits + 1,
                    max_digits_range,
                },
            ),
        }
    }
}

type RadixInWordTable = [RadixInWord; MAX_RADIX as usize + 1];

pub(crate) static RADIX_IN_WORD_TABLE: RadixInWordTable = fill_radix_in_word_table(
    [RadixInWord {
        max_digits: 0,
        max_digits_range: 0,
    }; MAX_RADIX as usize + 1],
    2,
);

const fn fill_radix_in_word_table(
    mut table: RadixInWordTable,
    next_radix: Digit,
) -> RadixInWordTable {
    if next_radix > MAX_RADIX {
        table
    } else {
        table[next_radix as usize] = RadixInWord::for_radix(next_radix);
        fill_radix_in_word_table(table, next_radix + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_in_word_table() {
        for radix in 2..=MAX_RADIX {
            let entry = &RADIX_IN_WORD_TABLE[radix as usize];
            // Check vs an approximation that happens to work for all bases.
            assert_eq!(
                entry.max_digits,
                ((WORD_BITS as f64 + 0.01) / (radix as f64).log2()) as usize
            );
            if !radix.is_power_of_two() {
                assert_eq!(
                    entry.max_digits_range,
                    (radix as usize).pow(entry.max_digits as u32)
                );
            }
        }
    }

    #[test]
    fn test_digit_to_ascii() {
        let ascii10 = digit_case_to_ascii10(DigitCase::Lower);
        assert_eq!(digit_to_ascii(7, ascii10), b'7');
        assert_eq!(digit_to_ascii(10, ascii10), b'a');
        assert_eq!(digit_to_ascii(35, ascii10), b'z');
        let ascii10 = digit_case_to_ascii10(DigitCase::Upper);
        assert_eq!(digit_to_ascii(7, ascii10), b'7');
        assert_eq!(digit_to_ascii(35, ascii10), b'Z');
    }

    #[test]
    fn test_digit_from_ascii() {
        assert_eq!(digit_from_ascii(b'7'), Some(7));
        assert_eq!(digit_from_ascii(b'a'), Some(10));
        assert_eq!(digit_from_ascii(b'z'), Some(35));
        assert_eq!(digit_from_ascii(b'Z'), Some(35));
        assert_eq!(digit_from_ascii(b'?'), None);
    }
}
