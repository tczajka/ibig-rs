//! Printing and parsing in any radix.

use crate::{
    ibig::IBig,
    primitive::{Word, WORD_BITS},
    radix::{digit_to_ascii, Digit, DigitCase, RadixInWord, MAX_RADIX},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use alloc::string::String;
use core::{
    cmp::max,
    fmt::{self, Alignment, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex, Write},
};

/// Representation of a `UBig` or `IBig` in any radix between 2 and 36 inclusive.
///
/// This can be used to format a number in a non-standard radix.
///
/// The default format uses lower-case letters a-z for digits 10-35.
/// The "alternative" format (`{:#}`) uses upper-case letters.
///
/// # Examples
///
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(format!("{}", ubig!(100).in_radix(2)), "1100100");
/// assert_eq!(format!("{}", ibig!(-3000).in_radix(16)), "-bb8");
/// assert_eq!(format!("{:#}", ubig!(3000).in_radix(16)), "BB8");
/// assert_eq!(format!("{:+010}", ubig!(100).in_radix(2)), "+001100100");
/// assert_eq!(format!("{:=^10}", ubig!(100).in_radix(2)), "=1100100==");
/// // For bases 2, 8, 10, 16 we don't have to use `InRadix`:
/// assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
/// assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
/// ```
pub struct InRadix<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: Digit,
    prefix: &'static str,
    digit_case: Option<DigitCase>,
}

impl InRadix<'_> {
    /// Format `InRadix`.
    ///
    /// Takes a continuation that completes formatting given a `PreparedForFormatting`.
    fn format<F, R>(&self, continuation: F) -> R
    where
        F: FnOnce(&mut dyn PreparedForFormatting) -> R,
    {
        match self.magnitude.repr() {
            Small(word) => {
                if self.radix.is_power_of_two() {
                    let mut prepared = PreparedWordInPow2::new(*word, self.radix);
                    continuation(&mut prepared)
                } else {
                    let mut prepared = PreparedWordInNonPow2::new(*word, self.radix);
                    continuation(&mut prepared)
                }
            }
            Large(buffer) => {
                if self.radix.is_power_of_two() {
                    let mut prepared = PreparedLargeInPow2::new(buffer, self.radix);
                    continuation(&mut prepared)
                } else {
                    panic!("Non-power-of-2 radix not implemented")
                }
            }
        }
    }

    /// Complete formatting in a `Formatter`.
    fn format_continuation_formatter(
        &self,
        f: &mut Formatter,
        prepared: &mut dyn PreparedForFormatting,
    ) -> fmt::Result {
        let mut width = prepared.width();

        // Adding sign and prefix to width will not overflow, because Buffer::MAX_CAPACITY leaves
        // (WORD_BITS - 1) spare bits before we would hit overflow.
        let sign = if self.sign == Negative {
            "-"
        } else if f.sign_plus() {
            "+"
        } else {
            ""
        };
        // In bytes, but it's OK because everything is ASCII.
        width += sign.len() + self.prefix.len();

        let digit_case = self.digit_case.unwrap_or_else(|| {
            if f.alternate() {
                DigitCase::Upper
            } else {
                DigitCase::Lower
            }
        });

        match f.width() {
            None => {
                f.write_str(sign)?;
                f.write_str(self.prefix)?;
                prepared.write(f, digit_case)?;
            }
            Some(min_width) => {
                if width >= min_width {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    prepared.write(f, digit_case)?;
                } else if f.sign_aware_zero_pad() {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    for _ in 0..min_width - width {
                        f.write_char('0')?;
                    }
                    prepared.write(f, digit_case)?;
                } else {
                    let left = match f.align() {
                        Some(Alignment::Left) => 0,
                        Some(Alignment::Right) | None => min_width - width,
                        Some(Alignment::Center) => (min_width - width) / 2,
                    };
                    let fill = f.fill();
                    for _ in 0..left {
                        f.write_char(fill)?;
                    }
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    prepared.write(f, digit_case)?;
                    for _ in left..min_width - width {
                        f.write_char(fill)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Complete formatting as a `String`.
    fn format_continuation_to_string(&self, prepared: &mut dyn PreparedForFormatting) -> String {
        let mut width = prepared.width();

        let sign = match self.sign {
            Positive => "",
            Negative => "-",
        };

        width += sign.len();

        let digit_case = self.digit_case.unwrap();

        let mut s = String::with_capacity(width);
        s += sign;
        prepared.write(&mut s, digit_case).unwrap();

        s
    }
}

impl Display for InRadix<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.format(|prepared| self.format_continuation_formatter(f, prepared))
    }
}

/// Trait for state of a partially-formatted `UBig`.
///
/// The state must be such the width (number of digits) is already known.
trait PreparedForFormatting {
    /// Returns the number of characters that will be written.
    fn width(&self) -> usize;

    /// Write to a stream.
    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result;
}

/// A `Word` prepared for formatting in a power-of-2 radix.
struct PreparedWordInPow2 {
    word: Word,
    log_radix: u32,
    width: usize,
}

impl PreparedWordInPow2 {
    /// Prepare a `Word` for formatting in a power-of-2 radix.
    fn new(word: Word, radix: Digit) -> PreparedWordInPow2 {
        debug_assert!(radix >= 2 && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);
        let width = max(
            (WORD_BITS - word.leading_zeros() + log_radix - 1) / log_radix,
            1,
        ) as usize;

        PreparedWordInPow2 {
            word,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedWordInPow2 {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        let mask: Digit = (1 << self.log_radix) - 1;
        for idx in (0..self.width as u32).rev() {
            let digit = (self.word >> (idx * self.log_radix)) as Digit & mask;
            writer.write_char(digit_to_ascii(digit, digit_case).as_char())?;
        }
        Ok(())
    }
}

/// A large number prepared for formatting in a power-of-2 radix.
struct PreparedLargeInPow2<'a> {
    words: &'a [Word],
    log_radix: u32,
    width: usize,
}

impl PreparedLargeInPow2<'_> {
    /// Prepare a large number for formatting in a power-of-2 radix.
    fn new(words: &[Word], radix: Digit) -> PreparedLargeInPow2 {
        debug_assert!(radix >= 2 && radix <= MAX_RADIX && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);
        // No overflow because words.len() * WORD_BITS + (log_radix-1) <= usize::MAX for
        // words.len() <= Buffer::MAX_CAPACITY.
        let width = max(
            (words.len() * WORD_BITS as usize - words.last().unwrap().leading_zeros() as usize
                + (log_radix - 1) as usize)
                / log_radix as usize,
            1,
        );
        PreparedLargeInPow2 {
            words,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedLargeInPow2<'_> {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        let mask: Digit = (1 << self.log_radix) - 1;

        let mut it = self.words.iter().rev();
        let mut word = it.next().unwrap();
        let mut bits = (self.width * self.log_radix as usize
            - (self.words.len() - 1) * WORD_BITS as usize) as u32;

        loop {
            let digit;
            if bits < self.log_radix {
                match it.next() {
                    Some(w) => {
                        let extra_bits = self.log_radix - bits;
                        bits = WORD_BITS - extra_bits;
                        digit = (word << extra_bits | w >> bits) as Digit & mask;
                        word = w;
                    }
                    None => break,
                }
            } else {
                bits -= self.log_radix;
                digit = (word >> bits) as Digit & mask;
            }
            writer.write_char(digit_to_ascii(digit, digit_case).as_char())?;
        }
        debug_assert!(bits == 0);
        Ok(())
    }
}

const MAX_DIGITS_IN_WORD_NON_POW_2: usize = RadixInWord::for_radix(3).max_digits;

/// A `Word` prepared for formatting in a non-power-of-2 radix.
struct PreparedWordInNonPow2 {
    // Little-endian digits.
    digits: [u8; MAX_DIGITS_IN_WORD_NON_POW_2],
    width: usize,
}

impl PreparedWordInNonPow2 {
    /// Prepare a `Word` for formatting in a non-power-of-2 radix.
    fn new(mut word: Word, radix: Digit) -> PreparedWordInNonPow2 {
        debug_assert!(radix >= 2 && radix <= MAX_RADIX && !radix.is_power_of_two());

        let mut prepared = PreparedWordInNonPow2 {
            digits: [0; MAX_DIGITS_IN_WORD_NON_POW_2],
            width: 0,
        };

        while word != 0 || prepared.width == 0 {
            prepared.digits[prepared.width] = (word % (radix as Word)) as u8;
            prepared.width += 1;
            word /= radix as Word;
        }

        prepared
    }
}

impl PreparedForFormatting for PreparedWordInNonPow2 {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, writer: &mut dyn Write, digit_case: DigitCase) -> fmt::Result {
        for digit in self.digits[..self.width].iter().rev() {
            writer.write_char(digit_to_ascii(*digit as Digit, digit_case).as_char())?;
        }
        Ok(())
    }
}

impl Display for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Debug for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Octal for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl LowerHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl UBig {
    /// Representation in a given radix.
    ///
    /// Using `in_radix` is a bit more efficient for formatting than converting to a string and then
    /// formatting.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ubig!(100).in_radix(2)), "1100100");
    /// assert_eq!(format!("{}", ubig!(3000).in_radix(16)), "bb8");
    /// assert_eq!(format!("{:#}", ubig!(3000).in_radix(16)), "BB8");
    /// assert_eq!(format!("{:+010}", ubig!(100).in_radix(2)), "+001100100");
    /// assert_eq!(format!("{:=^10}", ubig!(100).in_radix(2)), "=1100100==");
    /// // For bases 2, 8, 10, 16 we don't have to use `in_radix`:
    /// assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
    ///
    /// // This performs an extra String allocation and is less efficient:
    /// assert_eq!(format!("{:=^10}", ubig!(100).to_str_radix(2)), "=1100100==");
    /// ```
    #[inline]
    pub fn in_radix(&self, radix: u32) -> InRadix {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX
        );
        InRadix {
            sign: Positive,
            magnitude: self,
            radix,
            prefix: "",
            digit_case: None,
        }
    }

    /// String representation in an arbitrary radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(0x123f).to_str_radix(16), "123f");
    ///
    /// // Equivalent but slightly less efficient:
    /// assert_eq!(ubig!(0x123f).in_radix(16).to_string(), "123f");
    /// ```
    pub fn to_str_radix(&self, radix: u32) -> String {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX
        );
        let in_radix = InRadix {
            sign: Positive,
            magnitude: self,
            radix,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        };

        in_radix.format(|prepared| in_radix.format_continuation_to_string(prepared))
    }

    /// Upper-case string representation in an arbitrary radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(0x123f).to_str_radix_uppercase(16), "123F");
    ///
    /// // Equivalent but slightly less efficient:
    /// assert_eq!(format!("{:#}", ubig!(0x123f).in_radix(16)), "123F");
    /// ```
    pub fn to_str_radix_uppercase(&self, radix: u32) -> String {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX
        );
        let in_radix = InRadix {
            sign: Positive,
            magnitude: self,
            radix,
            prefix: "",
            digit_case: Some(DigitCase::Upper),
        };

        in_radix.format(|prepared| in_radix.format_continuation_to_string(prepared))
    }
}

impl Display for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Debug for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl Octal for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl LowerHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl IBig {
    /// Representation in a given radix.
    ///
    /// Using `in_radix` is a bit more efficient for formatting than converting to a string and then
    /// formatting.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ibig!(100).in_radix(2)), "1100100");
    /// assert_eq!(format!("{}", ibig!(-3000).in_radix(16)), "-bb8");
    /// assert_eq!(format!("{:#}", ibig!(3000).in_radix(16)), "BB8");
    /// assert_eq!(format!("{:+010}", ibig!(100).in_radix(2)), "+001100100");
    /// assert_eq!(format!("{:=^10}", ibig!(-100).in_radix(2)), "=-1100100=");
    /// // For bases 2, 8, 10, 16 we don't have to use `in_radix`:
    /// assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
    ///
    /// // This performs an extra String allocation and is less efficient:
    /// assert_eq!(format!("{:=^10}", ibig!(-100).to_str_radix(2)), "=-1100100=");
    /// ```
    #[inline]
    pub fn in_radix(&self, radix: u32) -> InRadix {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX,
        );
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
            prefix: "",
            digit_case: None,
        }
    }

    /// String representation in an arbitrary radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-0x123f).to_str_radix(16), "-123f");
    ///
    /// // Equivalent but slightly less efficient:
    /// assert_eq!(ibig!(-0x123f).in_radix(16).to_string(), "-123f");
    /// ```
    pub fn to_str_radix(&self, radix: u32) -> String {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX,
        );
        let in_radix = InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
            prefix: "",
            digit_case: Some(DigitCase::Lower),
        };

        in_radix.format(|prepared| in_radix.format_continuation_to_string(prepared))
    }

    /// Upper-case string representation in an arbitrary radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-0x123f).to_str_radix_uppercase(16), "-123F");
    ///
    /// // Equivalent but slightly less efficient:
    /// assert_eq!(format!("{:#}", ibig!(-0x123f).in_radix(16)), "-123F");
    /// ```
    pub fn to_str_radix_uppercase(&self, radix: u32) -> String {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and {} inclusive",
            MAX_RADIX,
        );
        let in_radix = InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
            prefix: "",
            digit_case: Some(DigitCase::Upper),
        };

        in_radix.format(|prepared| in_radix.format_continuation_to_string(prepared))
    }
}
