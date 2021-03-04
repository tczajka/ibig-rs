use crate::primitive::{Word, WORD_BITS};
use ascii::AsciiChar;

/// Digit and radix type.
pub(crate) type Digit = u32;

/// Maximum supported radix.
pub(crate) const MAX_RADIX: Digit = 36;

/// Is a radix in valid range?
pub(crate) fn is_radix_valid(radix: Digit) -> bool {
    (2..=MAX_RADIX).contains(&radix)
}

/// Panics if `radix` is not in valid range.
pub(crate) fn check_radix_valid(radix: Digit) {
    if !is_radix_valid(radix) {
        panic!("Invalid radix: {}", radix);
    }
}

/// Lower-case or upper-case digits: a-z or A-Z.
#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum DigitCase {
    Lower = b'a',
    Upper = b'A',
}

/// Converts a `Digit` to its ASCII representation.
///
/// Panics if `digit` is out of range.
pub(crate) fn digit_to_ascii(digit: Digit, digit_case: DigitCase) -> AsciiChar {
    let ascii = match digit {
        0..=9 => b'0' + (digit as u8),
        10..=35 => (digit_case as u8) + (digit - 10) as u8,
        _ => panic!("Invalid digit"),
    };
    AsciiChar::from_ascii(ascii).unwrap()
}

/// Converts a byte (ASCII) representation of a digit to its value.
pub(crate) fn digit_from_utf8_byte(byte: u8, radix: Digit) -> Option<Digit> {
    let res = match byte {
        b'0'..=b'9' => (byte - b'0') as Digit,
        b'a'..=b'z' => (byte - b'a') as Digit + 10,
        b'A'..=b'Z' => (byte - b'A') as Digit + 10,
        _ => return None,
    };
    if res < radix {
        Some(res)
    } else {
        None
    }
}

/// Properties of a given radix in a `Word`.
#[derive(Clone, Copy)]
pub(crate) struct RadixInWord {
    /// Maximum number of digits that fit in a `Word`.
    pub(crate) max_digits: usize,
    /// Radix to the power of `max_digits`.
    /// Only for non-power-of-2 radixes.
    pub(crate) max_digits_range: Word,
}

impl RadixInWord {
    pub(crate) const fn for_radix(radix: Digit) -> RadixInWord {
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
        assert_eq!(digit_to_ascii(7, DigitCase::Lower), AsciiChar::_7);
        assert_eq!(digit_to_ascii(10, DigitCase::Lower), AsciiChar::a);
        assert_eq!(digit_to_ascii(35, DigitCase::Lower), AsciiChar::z);
        assert_eq!(digit_to_ascii(7, DigitCase::Upper), AsciiChar::_7);
        assert_eq!(digit_to_ascii(35, DigitCase::Upper), AsciiChar::Z);
    }

    #[test]
    fn test_digit_from_utf8_byte() {
        assert_eq!(digit_from_utf8_byte(b'7', 10), Some(7));
        assert_eq!(digit_from_utf8_byte(b'a', 16), Some(10));
        assert_eq!(digit_from_utf8_byte(b'z', 36), Some(35));
        assert_eq!(digit_from_utf8_byte(b'Z', 36), Some(35));
        assert_eq!(digit_from_utf8_byte(b'?', 10), None);
        assert_eq!(digit_from_utf8_byte(b'a', 10), None);
        assert_eq!(digit_from_utf8_byte(b'z', 35), None);
        assert_eq!(digit_from_utf8_byte(255, 35), None);
    }
}
