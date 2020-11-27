//! Printing and parsing in any radix.

use crate::{
    buffer::Buffer,
    ibig::Sign::{self, *},
    radix::{digit_case_to_ascii10, digit_to_ascii, Digit, DigitCase, MAX_RADIX},
    ubig::{Repr::*, UBig},
    word::{Word, WORD_BITS},
};
use alloc::string::String;
use core::{
    cmp::max,
    fmt::{self, Alignment, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex, Write},
    str::from_utf8_unchecked,
};

/// Writes an ASCII character into a `Write`.
///
/// Must be valid ASCII, otherwise results in unsafe behavior.
fn write_ascii_char(writer: &mut dyn Write, ascii: u8) -> fmt::Result {
    let utf8 = [ascii];
    let s = unsafe { from_utf8_unchecked(&utf8) };
    writer.write_str(s)
}

/// Representation of a `UBig` or `IBig` in any radix between 2 and 36 inclusive.
///
/// This can be used to format a number in a non-standard radix.
///
/// The default format uses lower-case letters a-z for digits 10-35.
/// The "alternative" format (`{:#}`) uses upper-case letters.
/// ```
/// # use ibig::UBig;
/// assert_eq!(format!("{}", UBig::from(100u32).in_radix(2)), "1100100");
/// assert_eq!(format!("{}", UBig::from(3000u32).in_radix(16)), "bb8");
/// assert_eq!(format!("{:#}", UBig::from(3000u32).in_radix(16)), "BB8");
/// assert_eq!(format!("{:+010}", UBig::from(100u32).in_radix(2)), "+001100100");
/// assert_eq!(format!("{:=^10}", UBig::from(100u32).in_radix(2)), "=1100100==");
/// // For bases 2, 8, 10, 16 we don't have to use `InRadix`:
/// assert_eq!(format!("{:x}", UBig::from(3000u32)), "bb8");
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
                    panic!("Non-power-of-2 radix not implemented")
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
        // (WORD_BITS-1) spare bits before we would hit overflow.
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
                        write_ascii_char(f, b'0')?;
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
        let ascii10 = digit_case_to_ascii10(digit_case);
        let mask: Digit = (1 << self.log_radix) - 1;
        for idx in (0..self.width as u32).rev() {
            let digit = (self.word >> (idx * self.log_radix)) as Digit & mask;
            write_ascii_char(writer, digit_to_ascii(digit, ascii10))?;
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
        debug_assert!(radix >= 2 && radix.is_power_of_two());
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
        let ascii10 = digit_case_to_ascii10(digit_case);
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
            write_ascii_char(writer, digit_to_ascii(digit, ascii10))?;
        }
        debug_assert!(bits == 0);
        Ok(())
    }
}

/*
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
*/

impl Debug for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: Show in decimal.
        // Display::fmt(self, f)
        LowerHex::fmt(self, f)
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
    /// `radix` must be between 2 and 36 inclusive.
    #[inline]
    pub fn in_radix(&self, radix: u32) -> InRadix {
        assert!(
            radix >= 2 && radix <= MAX_RADIX,
            "radix must be between 2 and 36 inclusive"
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
    /// Equivalent to `self.to_radix(radix).to_string()` but more efficient.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0x123fu32).to_radix_str(16), "123f");
    /// ```
    pub fn to_radix_str(&self, radix: u32) -> String {
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
    /// Equivalent to `format!("{:#}", self.in_radix(radix))` but more efficient.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from(0x123fu32).to_radix_str_uppercase(16), "123F");
    /// ```
    pub fn to_radix_str_uppercase(&self, radix: u32) -> String {
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
