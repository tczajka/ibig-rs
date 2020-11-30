use crate::{
    buffer::Buffer,
    ibig::IBig,
    primitive::{Sign::*, Word, WORD_BITS},
    radix::{digit_from_ascii, Digit, MAX_RADIX, RADIX_IN_WORD_TABLE},
    ubig::UBig,
};
use core::fmt::{self, Display, Formatter};

/// Parse error when parsing `UBig` or `IBig`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// There were no digits in the string.
    NoDigits,
    /// Invalid digit for a given radix.
    InvalidDigit,
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::NoDigits => f.write_str("no digits"),
            ParseError::InvalidDigit => f.write_str("invalid digit"),
        }
    }
}

impl UBig {
    /// Convert a string in a given base to `UBig`.
    ///
    /// `src` may contain an optional `+` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// Panics if `radix` not in range 2-36.
    ///
    /// ```
    /// # use ibig::{ubig, UBig};
    /// assert_eq!(UBig::from_str_radix("+7ab", 32).unwrap(), ubig!(7499));
    /// ```
    #[inline]
    pub fn from_str_radix(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX
        );
        let src = src.strip_prefix("+").unwrap_or(src);
        UBig::from_str_radix_no_sign(src, radix)
    }

    /// Convert a string with an optional radix prefix to `UBig`.
    ///
    /// `src` may contain an optional `+` prefix.
    ///
    /// Allowed prefixes: `0b` for binary, `0o` for octal, `0x` for hexadecimal.
    ///
    /// ```
    /// # use ibig::{ubig, UBig};
    /// assert_eq!(UBig::from_str_with_radix_prefix("+0o17").unwrap(), ubig!(0o17));
    /// assert_eq!(UBig::from_str_with_radix_prefix("0x1f").unwrap(), ubig!(0x1f));
    /// ```
    #[inline]
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

    fn from_str_radix_no_sign(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(radix >= 2 && radix <= MAX_RADIX);
        if src.is_empty() {
            Err(ParseError::NoDigits)
        } else if radix.is_power_of_two() {
            UBig::from_str_radix_pow2(src, radix)
        } else {
            panic!("Non-power-of-2 radix parsing not implemented!")
        }
    }

    fn from_str_radix_pow2(src: &str, radix: Digit) -> Result<UBig, ParseError> {
        debug_assert!(
            !src.is_empty() && radix >= 2 && radix <= MAX_RADIX && radix.is_power_of_two()
        );

        if src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits {
            let word = word_from_str_radix_pow2(src, radix)?;
            Ok(UBig::from_word(word))
        } else {
            UBig::slow_from_str_radix_pow2(src, radix)
        }
    }

    fn slow_from_str_radix_pow2(src: &str, radix: u32) -> Result<UBig, ParseError> {
        debug_assert!(
            !src.is_empty() && radix >= 2 && radix <= MAX_RADIX && radix.is_power_of_two()
        );

        let log_radix = radix.trailing_zeros();
        let num_bits = src
            .len()
            .checked_mul(log_radix as usize)
            .expect("number too large");
        let mut buffer = Buffer::allocate((num_bits - 1) / WORD_BITS as usize + 1);
        let mut bits = 0;
        let mut word = 0;
        for ascii in src.as_bytes().iter().rev() {
            let digit = digit_from_ascii(*ascii).ok_or(ParseError::InvalidDigit)?;
            if digit >= radix {
                return Err(ParseError::InvalidDigit);
            }
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
}

fn word_from_str_radix_pow2(src: &str, radix: u32) -> Result<Word, ParseError> {
    debug_assert!(
        !src.is_empty()
            && radix >= 2
            && radix <= MAX_RADIX
            && radix.is_power_of_two()
            && src.len() <= RADIX_IN_WORD_TABLE[radix as usize].max_digits
    );

    let log_radix = radix.trailing_zeros();
    let mut word = 0;
    let mut bits = 0;
    for ascii in src.as_bytes().iter().rev() {
        let digit = digit_from_ascii(*ascii).ok_or(ParseError::InvalidDigit)?;
        if digit >= radix {
            return Err(ParseError::InvalidDigit);
        }
        word |= (digit as Word) << bits;
        bits += log_radix;
    }
    Ok(word)
}

impl IBig {
    /// Convert a string in a given base to `IBig`.
    ///
    /// The string may contain a `+` or `-` prefix.
    /// Digits 10-35 are represented by `a-z` or `A-Z`.
    ///
    /// Panics if `radix` not in range 2-36.
    ///
    /// ```
    /// # use ibig::{ibig, IBig};
    /// assert_eq!(IBig::from_str_radix("-7ab", 32).unwrap(), ibig!(-7499));
    /// ```
    #[inline]
    pub fn from_str_radix(mut src: &str, radix: Digit) -> Result<IBig, ParseError> {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX
        );
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
    /// ```
    /// # use ibig::{ibig, IBig};
    /// assert_eq!(IBig::from_str_with_radix_prefix("+0o17").unwrap(), ibig!(0o17));
    /// assert_eq!(IBig::from_str_with_radix_prefix("-0x1f").unwrap(), ibig!(-0x1f));
    /// ```
    #[inline]
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
