//! Parsing numbers.

use crate::{
    buffer::Buffer,
    ibig::IBig,
    mul,
    primitive::{Word, WORD_BITS},
    radix::{self, Digit},
    sign::Sign::*,
    ubig::UBig,
};
use alloc::vec::Vec;
use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Parse non-power-of-2 radix in chunks of CHUNK_LEN * digits_per_word(radix).
const CHUNK_LEN: usize = 256;

impl FromStr for UBig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<UBig, ParseError> {
        UBig::from_str_radix(s, 10)
    }
}

impl FromStr for IBig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<IBig, ParseError> {
        IBig::from_str_radix(s, 10)
    }
}

impl UBig {
    /// Convert a string in a given base to `UBig`.
    ///
    /// `src` may contain an optional `+` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{prelude::*, ParseError};
    /// assert_eq!(UBig::from_str_radix("+7ab", 32)?, ubig!(7499));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_radix(src: &str, radix: u32) -> Result<UBig, ParseError> {
        radix::check_radix_valid(radix);
        let src = src.strip_prefix("+").unwrap_or(src);
        UBig::from_str_radix_no_sign(src, radix)
    }

    /// Convert a string with an optional radix prefix to `UBig`.
    ///
    /// `src` may contain an optional `+` prefix.
    ///
    /// Allowed prefixes: `0b` for binary, `0o` for octal, `0x` for hexadecimal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{prelude::*, ParseError};
    /// assert_eq!(UBig::from_str_with_radix_prefix("+0o17")?, ubig!(0o17));
    /// assert_eq!(UBig::from_str_with_radix_prefix("0x1f")?, ubig!(0x1f));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_with_radix_prefix(src: &str) -> Result<UBig, ParseError> {
        let src = src.strip_prefix("+").unwrap_or(src);
        UBig::from_str_with_radix_prefix_no_sign(src)
    }

    /// Convert an unsigned string with an optional radix prefix to `UBig`.
    fn from_str_with_radix_prefix_no_sign(src: &str) -> Result<UBig, ParseError> {
        if let Some(bin) = src.strip_prefix("0b") {
            UBig::from_str_radix_no_sign(bin, 2)
        } else if let Some(oct) = src.strip_prefix("0o") {
            UBig::from_str_radix_no_sign(oct, 8)
        } else if let Some(hex) = src.strip_prefix("0x") {
            UBig::from_str_radix_no_sign(hex, 16)
        } else {
            UBig::from_str_radix_no_sign(src, 10)
        }
    }

    /// Convert an unsigned string to `UBig`.
    fn from_str_radix_no_sign(mut src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix));
        if src.is_empty() {
            return Err(ParseError::NoDigits);
        }

        while let Some(src2) = src.strip_prefix("0") {
            src = src2;
        }

        if radix.is_power_of_two() {
            UBig::from_str_radix_pow2(src, radix)
        } else {
            UBig::from_str_radix_non_pow2(src, radix)
        }
    }

    /// Convert an unsigned string to `UBig` for a power-of-2 radix.
    fn from_str_radix_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix) && radix.is_power_of_two());

        if src.len() <= radix::digits_per_word(radix) {
            let word = from_str_radix_pow2_word(src, radix)?;
            Ok(UBig::from_word(word))
        } else {
            UBig::from_str_radix_pow2_large(src, radix)
        }
    }

    /// Convert an unsigned string to `UBig` for a power-of-2 radix.
    ///
    /// The result will usually not fit in a single word.
    fn from_str_radix_pow2_large(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix) && radix.is_power_of_two());

        let log_radix = radix.trailing_zeros();
        let num_bits = src
            .len()
            .checked_mul(log_radix as usize)
            .expect("number too large");
        let mut buffer = Buffer::allocate((num_bits - 1) / WORD_BITS as usize + 1);
        let mut bits = 0;
        let mut word = 0;
        for byte in src.as_bytes().iter().rev() {
            let digit =
                radix::digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
            word |= (digit as Word) << bits;
            let new_bits = bits + log_radix;
            if new_bits >= WORD_BITS {
                buffer.push(word);
                word = (digit as Word) >> (WORD_BITS - bits);
                bits = new_bits - WORD_BITS;
            } else {
                bits = new_bits;
            }
        }
        if bits > 0 {
            buffer.push(word);
        }
        Ok(buffer.into())
    }

    /// Convert an unsigned string to `UBig` for a non-power-of-2 radix.
    fn from_str_radix_non_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());

        let bytes = src.as_bytes();
        let digits_per_word = radix::digits_per_word(radix);

        if bytes.len() <= digits_per_word {
            let word = from_str_radix_non_pow2_word(bytes, radix)?;
            Ok(UBig::from_word(word))
        } else if bytes.len() <= CHUNK_LEN * digits_per_word {
            UBig::from_str_radix_non_pow2_chunk(bytes, radix)
        } else {
            UBig::from_str_radix_non_pow2_large(bytes, radix)
        }
    }

    /// Convert an unsigned string to `UBig` for a non-power-of-2 radix.
    ///
    /// The length of input is limited to `CHUNK_LEN * digits_per_word(radix)`.
    fn from_str_radix_non_pow2_chunk(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());

        let digits_per_word = radix::digits_per_word(radix);
        debug_assert!(bytes.len() <= CHUNK_LEN * digits_per_word);

        let range_per_word = radix::range_per_word(radix);
        let groups = bytes.rchunks(digits_per_word);
        let mut buffer = Buffer::allocate(groups.len());
        for group in groups.rev() {
            let next = from_str_radix_non_pow2_word(group, radix)?;
            let carry = mul::mul_word_in_place_with_carry(&mut buffer, range_per_word, next);
            if carry != 0 {
                buffer.push(carry);
            }
        }
        Ok(buffer.into())
    }

    /// Convert an unsigned string to `UBig` for a non-power-of-2 radix.
    ///
    /// This result will usually not fit in CHUNK_LEN words.
    fn from_str_radix_non_pow2_large(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let chunk_bytes = CHUNK_LEN * radix::digits_per_word(radix);
        assert!(bytes.len() > chunk_bytes);

        // Calculate radix^n for n = (chunk_bytes << i) < bytes.len().
        let mut radix_powers: Vec<UBig> = Vec::new();
        radix_powers.push(UBig::from_word(radix::range_per_word(radix)).pow(CHUNK_LEN));

        // while (chunk_bytes << radix_powers.len()) < bytes.len()
        // To avoid overflow:
        while chunk_bytes <= (bytes.len() - 1) >> radix_powers.len() {
            let prev = radix_powers.last().unwrap();
            let new = prev * prev;
            radix_powers.push(new);
        }

        UBig::from_str_radix_non_pow2_divide_conquer(bytes, radix, chunk_bytes, &radix_powers)
    }

    /// Convert an unsigned string to `UBig` for a non-power-of-2 radix.
    ///
    /// `radix_powers` contains radix^n for n = chunk digits << i
    fn from_str_radix_non_pow2_divide_conquer(
        bytes: &[u8],
        radix: Digit,
        chunk_bytes: usize,
        radix_powers: &[UBig],
    ) -> Result<UBig, ParseError> {
        debug_assert!(bytes.len() <= chunk_bytes << radix_powers.len());

        match radix_powers.split_last() {
            None => UBig::from_str_radix_non_pow2_chunk(bytes, radix),
            Some((radix_power, radix_powers)) => {
                let bytes_lo_len = chunk_bytes << radix_powers.len();
                if bytes.len() <= bytes_lo_len {
                    UBig::from_str_radix_non_pow2_divide_conquer(
                        bytes,
                        radix,
                        chunk_bytes,
                        radix_powers,
                    )
                } else {
                    let (bytes_hi, bytes_lo) = bytes.split_at(bytes.len() - bytes_lo_len);
                    let res_hi = UBig::from_str_radix_non_pow2_divide_conquer(
                        bytes_hi,
                        radix,
                        chunk_bytes,
                        radix_powers,
                    )?;
                    let res_lo = UBig::from_str_radix_non_pow2_divide_conquer(
                        bytes_lo,
                        radix,
                        chunk_bytes,
                        radix_powers,
                    )?;
                    Ok(res_hi * radix_power + res_lo)
                }
            }
        }
    }
}

impl IBig {
    /// Convert a string in a given base to `IBig`.
    ///
    /// The string may contain a `+` or `-` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{prelude::*, ParseError};
    /// assert_eq!(IBig::from_str_radix("-7ab", 32)?, ibig!(-7499));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_radix(mut src: &str, radix: u32) -> Result<IBig, ParseError> {
        radix::check_radix_valid(radix);
        let sign;
        match src.strip_prefix("-") {
            Some(s) => {
                sign = Negative;
                src = s;
            }
            None => {
                sign = Positive;
                src = src.strip_prefix("+").unwrap_or(src);
            }
        }
        let mag = UBig::from_str_radix_no_sign(src, radix)?;
        Ok(IBig::from_sign_magnitude(sign, mag))
    }

    /// Convert a string with an optional radix prefix to `IBig`.
    ///
    /// `src` may contain an '+' or `-` prefix..
    ///
    /// Allowed prefixes: `0b` for binary, `0o` for octal, `0x` for hexadecimal.
    ///
    /// # Examples
    /// ```
    /// # use ibig::{prelude::*, ParseError};
    /// assert_eq!(IBig::from_str_with_radix_prefix("+0o17")?, ibig!(0o17));
    /// assert_eq!(IBig::from_str_with_radix_prefix("-0x1f")?, ibig!(-0x1f));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_str_with_radix_prefix(mut src: &str) -> Result<IBig, ParseError> {
        let sign;
        match src.strip_prefix("-") {
            Some(s) => {
                sign = Negative;
                src = s;
            }
            None => {
                sign = Positive;
                src = src.strip_prefix("+").unwrap_or(src);
            }
        }
        let mag = UBig::from_str_with_radix_prefix_no_sign(src)?;
        Ok(IBig::from_sign_magnitude(sign, mag))
    }
}

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

/// Convert an unsigned string to `Word` for a power-of-2 radix.
///
/// The length of the string must be at most digits_per_word(radix).
fn from_str_radix_pow2_word(src: &str, radix: Digit) -> Result<Word, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && radix.is_power_of_two());
    debug_assert!(src.len() <= radix::digits_per_word(radix));

    let log_radix = radix.trailing_zeros();
    let mut word = 0;
    let mut bits = 0;
    for byte in src.as_bytes().iter().rev() {
        let digit = radix::digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
        word |= (digit as Word) << bits;
        bits += log_radix;
    }
    Ok(word)
}

/// Convert an unsigned string to `Word` for a non-power-of-2 radix.
///
/// The length of the string must be at most digits_per_word(radix).
fn from_str_radix_non_pow2_word(src: &[u8], radix: Digit) -> Result<Word, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    debug_assert!(src.len() <= radix::digits_per_word(radix));

    let mut word: Word = 0;
    for byte in src.iter() {
        let digit = radix::digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
        word = word * (radix as Word) + (digit as Word);
    }
    Ok(word)
}
