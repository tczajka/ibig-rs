//! Bit operations.

use crate::{Digit, is_negative, min_len};

/// The width of a [`Digit`] in bits, as a `usize`.
pub const DIGIT_BITS_USIZE: usize = Digit::BITS as usize;

/// Splits a bit `position` into the index of the [`Digit`] that holds it and the index of the
/// bit within that digit.
///
/// Returns `(digit_index, bit_index)` such that
/// `position == digit_index * DIGIT_BITS_USIZE + bit_index`, with `bit_index` in
/// `0..Digit::BITS`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{split_bit_index, DIGIT_BITS_USIZE};
/// assert_eq!(split_bit_index(0), (0, 0));
/// assert_eq!(split_bit_index(5), (0, 5));
/// assert_eq!(split_bit_index(DIGIT_BITS_USIZE), (1, 0));
/// assert_eq!(split_bit_index(DIGIT_BITS_USIZE + 3), (1, 3));
/// ```
pub fn split_bit_index(position: usize) -> (usize, u32) {
    (
        position / DIGIT_BITS_USIZE,
        (position % DIGIT_BITS_USIZE).try_into().unwrap(),
    )
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

/// Returns the bit at `position`. Positions at or above the value's bit width read as `false`,
/// since the value is zero-extended.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bit};
/// assert!(bit(&[Digit::from(0b101u8)], 0));
/// assert!(!bit(&[Digit::from(0b101u8)], 1));
/// assert!(bit(&[Digit::from(0b101u8)], 2));
/// // Above the value's bits.
/// assert!(!bit(&[Digit::from(5u8)], 100));
/// // The low bit of the second digit.
/// assert!(bit(&[Digit::ZERO, Digit::from(1u8)], Digit::BITS.try_into().unwrap()));
/// ```
pub fn bit(digits: &[Digit], position: usize) -> bool {
    let (digit_index, bit_index) = split_bit_index(position);
    digit_index < digits.len() && digit_bit(digits[digit_index], bit_index)
}

/// Returns the bit at `position` of the two's complement signed value held in the non-empty
/// little-endian `digits`. Positions at or above `digits.len() * Digit::BITS` read as the sign
/// bit, since the value is sign-extended.
///
/// # Panics
///
/// Panics if `digits` is empty.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, bit_signed};
/// // -1 is all ones, including the sign-extended positions.
/// assert!(bit_signed(&[Digit::MAX], 0));
/// assert!(bit_signed(&[Digit::MAX], 100));
/// // 0b101 is non-negative, so high positions read as zero.
/// assert!(bit_signed(&[Digit::from(0b101u8)], 0));
/// assert!(!bit_signed(&[Digit::from(0b101u8)], 100));
/// ```
pub fn bit_signed(digits: &[Digit], position: usize) -> bool {
    let (digit_index, bit_index) = split_bit_index(position);
    if digit_index < digits.len() {
        digit_bit(digits[digit_index], bit_index)
    } else {
        is_negative(digits)
    }
}

/// Sets the bit at `position` to `value`.
///
/// # Panics
///
/// Panics if `position` is not within `digits`, that is if
/// `position >= digits.len() * Digit::BITS`.
///
/// # Examples
///
/// ```
/// # use ibig_core::{Digit, set_bit};
/// let mut digits = [Digit::from(0b100u8)];
/// set_bit(&mut digits, 0, true);
/// assert_eq!(digits, [Digit::from(0b101u8)]);
/// set_bit(&mut digits, 2, false);
/// assert_eq!(digits, [Digit::from(0b001u8)]);
/// ```
pub fn set_bit(digits: &mut [Digit], position: usize, value: bool) {
    let (digit_index, bit_index) = split_bit_index(position);
    let mask = Digit::from_u8(1) << bit_index;
    if value {
        digits[digit_index] |= mask;
    } else {
        digits[digit_index] &= !mask;
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
