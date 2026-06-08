//! Bit operations.

use crate::{Digit, is_negative, min_len};

/// The width of a [`Digit`] in bits, as a `usize`.
pub const DIGIT_BITS_USIZE: usize = Digit::BITS as usize;

/// A bit position within a digit slice.
///
/// # Examples
///
/// ```
/// # use ibig_core::{BitIndex, DIGIT_BITS_USIZE};
/// let index = BitIndex::from(DIGIT_BITS_USIZE + 3);
/// assert_eq!(index.digit_index(), 1);
/// assert_eq!(index.bit_index(), 3);
/// assert_eq!(usize::try_from(index).unwrap(), DIGIT_BITS_USIZE + 3);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BitIndex {
    digit_index: usize,
    bit_index: u32,
}

impl BitIndex {
    /// Creates a `BitIndex` from a digit index and a bit index within that digit.
    ///
    /// # Panics
    ///
    /// Panics if `bit_index >= Digit::BITS`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig_core::BitIndex;
    /// let index = BitIndex::new(1, 3);
    /// assert_eq!(index.digit_index(), 1);
    /// assert_eq!(index.bit_index(), 3);
    /// ```
    #[inline]
    pub fn new(digit_index: usize, bit_index: u32) -> BitIndex {
        assert!(bit_index < Digit::BITS, "bit index out of range");
        BitIndex {
            digit_index,
            bit_index,
        }
    }

    /// The index of the [`Digit`] that holds the bit.
    #[inline]
    pub fn digit_index(self) -> usize {
        self.digit_index
    }

    /// The index of the bit within its [`Digit`], in the range `0..Digit::BITS`.
    #[inline]
    pub fn bit_index(self) -> u32 {
        self.bit_index
    }
}

impl From<usize> for BitIndex {
    #[inline]
    fn from(index: usize) -> BitIndex {
        BitIndex {
            digit_index: index / DIGIT_BITS_USIZE,
            bit_index: (index % DIGIT_BITS_USIZE).try_into().unwrap(),
        }
    }
}

/// The error returned when converting a [`BitIndex`] to a `usize` that would
/// overflow `usize`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitIndexOutOfRange;

impl core::fmt::Display for BitIndexOutOfRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("bit index does not fit in a usize")
    }
}

impl core::error::Error for BitIndexOutOfRange {}

impl TryFrom<BitIndex> for usize {
    type Error = BitIndexOutOfRange;

    #[inline]
    fn try_from(index: BitIndex) -> Result<usize, BitIndexOutOfRange> {
        const {
            assert!(Digit::BITS.is_power_of_two());
        }
        let high = index
            .digit_index
            .checked_mul(DIGIT_BITS_USIZE)
            .ok_or(BitIndexOutOfRange)?;
        Ok(high | usize::try_from(index.bit_index).unwrap())
    }
}

/// Returns the minimum number of bits needed to represent the unsigned value held in the
/// little-endian `digits`: the position of the most-significant set bit plus one, or 0 for
/// the value zero.
///
/// # Overflow
///
/// The result is up to `digits.len() * Digit::BITS`, which overflows `usize` for a value of
/// more than `usize::MAX` bits. On overflow this panics in debug builds and wraps in release
/// builds.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bit_width};
/// assert_eq!(bit_width(&[]), 0);
/// assert_eq!(bit_width(&[Digit::from(1u8)]), 1);
/// assert_eq!(bit_width(&[Digit::from(1u8), Digit::ZERO]), 1);
/// assert_eq!(bit_width(&[Digit::from(5u8)]), 3); // 0b101
/// assert_eq!(bit_width(&[Digit::ZERO, Digit::from(1u8)]), Digit::BITS as usize + 1);
/// ```
pub fn bit_width(digits: &[Digit]) -> usize {
    let len = min_len(digits);
    if len == 0 {
        0
    } else {
        let top_width =
            DIGIT_BITS_USIZE - usize::try_from(digits[len - 1].leading_zeros()).unwrap();
        (len - 1) * DIGIT_BITS_USIZE + top_width
    }
}

/// Returns the bit at `index`. An `index` whose digit is at or above the value's length reads as
/// `false`, since the value is zero-extended.
///
/// # Examples
///
/// ```
/// # use ibig_core::{BitIndex, Digit, bit};
/// assert!(bit(&[Digit::from(0b101u8)], BitIndex::from(0)));
/// assert!(!bit(&[Digit::from(0b101u8)], BitIndex::from(1)));
/// assert!(bit(&[Digit::from(0b101u8)], BitIndex::from(2)));
/// // Above the value's bits.
/// assert!(!bit(&[Digit::from(5u8)], BitIndex::from(100)));
/// // The low bit of the second digit.
/// assert!(bit(&[Digit::ZERO, Digit::from(1u8)], BitIndex::new(1, 0)));
/// ```
pub fn bit(digits: &[Digit], index: BitIndex) -> bool {
    index.digit_index() < digits.len() && digit_bit(digits[index.digit_index()], index.bit_index())
}

/// Returns the bit at `index` of the two's complement signed value held in the non-empty
/// little-endian `digits`. An `index` whose digit is at or above `digits.len()` reads as the
/// sign bit, since the value is sign-extended.
///
/// # Panics
///
/// Panics if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{BitIndex, Digit, bit_signed};
/// // -1 is all ones, including the sign-extended positions.
/// assert!(bit_signed(&[Digit::MAX], BitIndex::from(0)));
/// assert!(bit_signed(&[Digit::MAX], BitIndex::from(100)));
/// // 0b101 is non-negative, so high positions read as zero.
/// assert!(bit_signed(&[Digit::from(0b101u8)], BitIndex::from(0)));
/// assert!(!bit_signed(&[Digit::from(0b101u8)], BitIndex::from(100)));
/// ```
pub fn bit_signed(digits: &[Digit], index: BitIndex) -> bool {
    if index.digit_index() < digits.len() {
        digit_bit(digits[index.digit_index()], index.bit_index())
    } else {
        is_negative(digits)
    }
}

/// Sets the bit at `index` to `value`.
///
/// # Panics
///
/// Panics if `index.digit_index() >= digits.len()`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{BitIndex, Digit, set_bit};
/// let mut digits = [Digit::from(0b100u8)];
/// set_bit(&mut digits, BitIndex::from(0), true);
/// assert_eq!(digits, [Digit::from(0b101u8)]);
/// set_bit(&mut digits, BitIndex::from(2), false);
/// assert_eq!(digits, [Digit::from(0b001u8)]);
/// ```
pub fn set_bit(digits: &mut [Digit], index: BitIndex, value: bool) {
    let mask = Digit::from_u8(1) << index.bit_index();
    if value {
        digits[index.digit_index()] |= mask;
    } else {
        digits[index.digit_index()] &= !mask;
    }
}

/// Returns the number of trailing zero bits of the unsigned value held in the little-endian
/// `digits`.
///
/// # Overflow
///
/// For extremely long slices, the result may overflow `usize`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, trailing_zeros};
/// assert_eq!(trailing_zeros(&[Digit::from(0b1100u8)]), 2);
/// assert_eq!(trailing_zeros(&[Digit::from(1u8)]), 0);
/// assert_eq!(
///     trailing_zeros(&[Digit::ZERO, Digit::from(1u8)]),
///     usize::try_from(Digit::BITS).unwrap());
/// assert_eq!(
///     trailing_zeros(&[Digit::ZERO, Digit::ZERO]),
///     2 * usize::try_from(Digit::BITS).unwrap());
/// ```
pub fn trailing_zeros(digits: &[Digit]) -> usize {
    match digits
        .iter()
        .enumerate()
        .find(|&(_, &digit)| digit != Digit::ZERO)
    {
        Some((i, &digit)) => {
            i * DIGIT_BITS_USIZE + usize::try_from(digit.trailing_zeros()).unwrap()
        }
        None => digits.len() * DIGIT_BITS_USIZE,
    }
}

/// Returns the number of trailing one bits of the unsigned value held in the little-endian
/// `digits`.
///
/// # Overflow
///
/// For extremely long slices, the result may overflow `usize`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, trailing_ones};
/// assert_eq!(trailing_ones(&[Digit::from(0b1011u8)]), 2);
/// assert_eq!(trailing_ones(&[Digit::ZERO]), 0);
/// assert_eq!(
///     trailing_ones(&[Digit::MAX, Digit::from(0b10u8)]),
///     usize::try_from(Digit::BITS).unwrap());
/// assert_eq!(
///     trailing_ones(&[Digit::MAX, Digit::MAX]),
///     2 * usize::try_from(Digit::BITS).unwrap());
/// ```
pub fn trailing_ones(digits: &[Digit]) -> usize {
    match digits
        .iter()
        .enumerate()
        .find(|&(_, &digit)| digit != Digit::MAX)
    {
        Some((i, &digit)) => i * DIGIT_BITS_USIZE + usize::try_from(digit.trailing_ones()).unwrap(),
        None => digits.len() * DIGIT_BITS_USIZE,
    }
}

/// Returns the number of one bits (the population count) in `digits`.
///
/// # Overflow
///
/// For extremely long slices, the result may overflow `usize`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, count_ones};
/// assert_eq!(count_ones(&[]), 0);
/// assert_eq!(count_ones(&[Digit::from(0b101u8)]), 2);
/// assert_eq!(count_ones(&[Digit::MAX]), Digit::BITS as usize);
/// assert_eq!(count_ones(&[Digit::from(0b11u8), Digit::from(0b1u8)]), 3);
/// ```
pub fn count_ones(digits: &[Digit]) -> usize {
    digits
        .iter()
        .map(|&digit| usize::try_from(digit.count_ones()).unwrap())
        .sum()
}

/// Returns `true` if the unsigned value held in the little-endian `digits` is a power of two,
/// i.e. exactly one bit is set. Returns `false` for zero.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, is_power_of_two};
/// assert!(is_power_of_two(&[Digit::from(8u8)]));
/// assert!(!is_power_of_two(&[Digit::from(6u8)]));
/// assert!(!is_power_of_two(&[Digit::ZERO]));
/// assert!(is_power_of_two(&[Digit::ZERO, Digit::from(4u8)]));
/// assert!(!is_power_of_two(&[Digit::from(1u8), Digit::from(1u8)]));
/// ```
pub fn is_power_of_two(digits: &[Digit]) -> bool {
    let mut found = false;
    for &digit in digits {
        if digit != Digit::ZERO {
            if found || !digit.is_power_of_two() {
                return false;
            }
            found = true;
        }
    }
    found
}

/// Replaces the unsigned value with the smallest power of two greater than or equal to it.
///
/// Returns `true` on overflow — when that power of two does not fit in
/// `digits.len() * Digit::BITS` bits — in which case `digits` is set to zero.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, next_power_of_two};
/// let mut digits = [Digit::from(5u8), Digit::from(5u8)];
/// assert!(!next_power_of_two(&mut digits));
/// assert_eq!(digits, [Digit::ZERO, Digit::from(8u8)]);
/// let mut digits = [];
/// assert!(next_power_of_two(&mut digits));
/// let mut digits = [Digit::ZERO];
/// assert!(!next_power_of_two(&mut digits));
/// assert_eq!(digits, [Digit::from(1u8)]);
/// let mut digits = [Digit::MAX];
/// assert!(next_power_of_two(&mut digits));
/// assert_eq!(digits, [Digit::ZERO]);
/// ```
pub fn next_power_of_two(digits: &mut [Digit]) -> bool {
    // Find the top non-zero digit.
    let Some(top) = digits.iter().rposition(|&digit| digit != Digit::ZERO) else {
        // `digits` is all zeros
        if digits.is_empty() {
            // Overflow.
            return true;
        } else {
            digits[0] = Digit::from_u8(1);
            return false;
        }
    };

    let top_digit = digits[top];

    // Zero out digits below top.
    // If `digits` is already a power of two, return.
    if top_digit.is_power_of_two() {
        // Find the next non-zero digit below top.
        let Some(second) = digits[..top]
            .iter()
            .rposition(|&digit| digit != Digit::ZERO)
        else {
            // The whole number is already a power of two.
            return false;
        };
        digits[..second + 1].fill(Digit::ZERO);
    } else {
        digits[..top].fill(Digit::ZERO);
    }

    // We now know `digits` is not a power of two.
    // The top digit should become strictly next power of two.
    match top_digit
        .checked_add(Digit::from_u8(1))
        .and_then(Digit::checked_next_power_of_two)
    {
        Some(new_top_digit) => {
            digits[top] = new_top_digit;
        }
        None => {
            digits[top] = Digit::ZERO;
            if top + 1 != digits.len() {
                digits[top + 1] = Digit::from_u8(1);
            } else {
                // Overflow.
                return true;
            }
        }
    }
    false
}

/// Returns the bit at `bit_index` (which must be less than `Digit::BITS`) of a single digit.
fn digit_bit(digit: Digit, bit_index: u32) -> bool {
    (digit >> bit_index) & Digit::from_u8(1) != Digit::ZERO
}
