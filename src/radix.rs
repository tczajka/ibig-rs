//! Information about radixes.

use crate::{
    fast_divide::{FastDivide, FastDivideSmall},
    primitive::{Word, WORD_BITS},
};
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

/// Maximum number of digits that a `Word` can ever have for any non-power-of-2 radix.
pub(crate) const MAX_WORD_DIGITS_NON_POW_2: usize = RadixInfo::for_radix(3).digits_per_word + 1;

/// Properties of a given radix.
#[derive(Clone, Copy)]
pub(crate) struct RadixInfo {
    /// The number of digits that can always fit in a `Word`.
    pub(crate) digits_per_word: usize,

    /// Radix to the power of `max_digits`.
    /// Only for non-power-of-2 radixes.
    pub(crate) range_per_word: Word,

    /// Faster division by `radix`.
    pub(crate) fast_div_radix: FastDivideSmall,

    /// Faster division by range_per_word.
    /// Only for non-power-of-2 radixes.
    pub(crate) fast_div_range_per_word: FastDivide,
}

/// RadixInfo for a given radix.
pub(crate) fn radix_info(radix: Digit) -> &'static RadixInfo {
    debug_assert!(is_radix_valid(radix));
    &RADIX_INFO_TABLE[radix as usize]
}

impl RadixInfo {
    const fn for_radix(radix: Digit) -> RadixInfo {
        let fast_div_radix = FastDivideSmall::new(radix as Word);
        if radix.is_power_of_two() {
            RadixInfo {
                digits_per_word: (WORD_BITS / radix.trailing_zeros()) as usize,
                range_per_word: 0,
                fast_div_radix,
                fast_div_range_per_word: FastDivide::dummy(),
            }
        } else {
            let mut info = RadixInfo::for_radix_recursive(
                radix,
                RadixInfo {
                    digits_per_word: 0,
                    range_per_word: 1,
                    fast_div_radix,
                    fast_div_range_per_word: FastDivide::dummy(),
                },
            );
            info.fast_div_range_per_word = FastDivide::new(info.range_per_word);
            info
        }
    }

    const fn for_radix_recursive(radix: Digit, info: RadixInfo) -> RadixInfo {
        match info.range_per_word.checked_mul(radix as Word) {
            None => info,
            Some(range_per_word) => RadixInfo::for_radix_recursive(
                radix,
                RadixInfo {
                    digits_per_word: info.digits_per_word + 1,
                    range_per_word,
                    ..info
                },
            ),
        }
    }
}

type RadixInfoTable = [RadixInfo; MAX_RADIX as usize + 1];

static RADIX_INFO_TABLE: RadixInfoTable = fill_radix_info_table(
    [RadixInfo {
        digits_per_word: 0,
        range_per_word: 0,
        fast_div_radix: FastDivideSmall::dummy(),
        fast_div_range_per_word: FastDivide::dummy(),
    }; MAX_RADIX as usize + 1],
    2,
);

const fn fill_radix_info_table(mut table: RadixInfoTable, next_radix: Digit) -> RadixInfoTable {
    if next_radix > MAX_RADIX {
        table
    } else {
        table[next_radix as usize] = RadixInfo::for_radix(next_radix);
        fill_radix_info_table(table, next_radix + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_info_table() {
        for radix in 2..=MAX_RADIX {
            let info = radix_info(radix);
            // Check vs an approximation that happens to work for all bases.
            assert_eq!(
                info.digits_per_word,
                ((WORD_BITS as f64 + 0.01) / (radix as f64).log2()) as usize
            );
            if !radix.is_power_of_two() {
                assert_eq!(
                    info.range_per_word,
                    (radix as Word).pow(info.digits_per_word as u32)
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
