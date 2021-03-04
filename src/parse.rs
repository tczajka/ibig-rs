use crate::{
    buffer::Buffer,
    ibig::IBig,
    mul::mul_word_in_place_with_carry,
    primitive::{Word, WORD_BITS},
    radix::{check_radix_valid, digit_from_utf8_byte, is_radix_valid, Digit, RADIX_IN_WORD_TABLE},
    sign::Sign::*,
    ubig::UBig,
};
use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

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
        check_radix_valid(radix);
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

    fn from_str_radix_no_sign(mut src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(is_radix_valid(radix));
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

    fn from_str_radix_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(is_radix_valid(radix) && radix.is_power_of_two());

        if src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits {
            let word = word_from_str_radix_pow2(src, radix)?;
            Ok(UBig::from_word(word))
        } else {
            UBig::slow_from_str_radix_pow2(src, radix)
        }
    }

    fn slow_from_str_radix_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(is_radix_valid(radix) && radix.is_power_of_two());

        let log_radix = radix.trailing_zeros();
        let num_bits = src
            .len()
            .checked_mul(log_radix as usize)
            .expect("number too large");
        let mut buffer = Buffer::allocate((num_bits - 1) / WORD_BITS as usize + 1);
        let mut bits = 0;
        let mut word = 0;
        for byte in src.as_bytes().iter().rev() {
            let digit = digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
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

    fn from_str_radix_non_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(is_radix_valid(radix) && !radix.is_power_of_two());

        if src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits {
            let word = word_from_str_radix_non_pow2(src.as_bytes(), radix)?;
            Ok(UBig::from_word(word))
        } else {
            // TODO: Recursive.
            UBig::slow_from_str_radix_non_pow2(src, radix)
        }
    }

    fn slow_from_str_radix_non_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(is_radix_valid(radix) && !radix.is_power_of_two());

        let radix_in_word = RADIX_IN_WORD_TABLE[radix as usize];
        let chunks = src.as_bytes().rchunks(radix_in_word.max_digits);
        let mut buffer = Buffer::allocate(chunks.len());
        for chunk in chunks.rev() {
            let next = word_from_str_radix_non_pow2(chunk, radix)?;
            let carry =
                mul_word_in_place_with_carry(&mut buffer, radix_in_word.max_digits_range, next);
            if carry != 0 {
                buffer.push(carry);
            }
        }
        Ok(buffer.into())
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
        check_radix_valid(radix);
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

fn word_from_str_radix_pow2(src: &str, radix: Digit) -> Result<Word, ParseError> {
    debug_assert!(is_radix_valid(radix) && radix.is_power_of_two());
    debug_assert!(src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits);

    let log_radix = radix.trailing_zeros();
    let mut word = 0;
    let mut bits = 0;
    for byte in src.as_bytes().iter().rev() {
        let digit = digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
        word |= (digit as Word) << bits;
        bits += log_radix;
    }
    Ok(word)
}

fn word_from_str_radix_non_pow2(src: &[u8], radix: Digit) -> Result<Word, ParseError> {
    debug_assert!(is_radix_valid(radix) && !radix.is_power_of_two());
    debug_assert!(src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits);

    let mut word: Word = 0;
    for byte in src.iter() {
        let digit = digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
        word = word * (radix as Word) + (digit as Word);
    }
    Ok(word)
}
