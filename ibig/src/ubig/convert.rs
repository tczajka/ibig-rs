//! Conversions to and from [`UBig`].

use crate::{Digits, UBig};
use alloc::vec::Vec;
use ibig_core::Digit;

impl UBig {
    /// Returns the little-endian (least-significant-first) byte representation, with no
    /// most-significant zero bytes. Zero produces an empty sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[5, 1, 0]).to_le_bytes(), [5, 1]);
    /// assert!(UBig::from_le_bytes(&[]).to_le_bytes().is_empty());
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = self
            .as_digits()
            .iter()
            .map(|digit| digit.to_le_bytes())
            .collect::<Vec<_>>()
            .into_flattened();
        while let Some(&0) = bytes.last() {
            bytes.pop();
        }
        bytes
    }

    /// Returns the big-endian (most-significant-first) byte representation, with no leading
    /// zero bytes. Zero produces an empty sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_be_bytes(&[0, 1, 5]).to_be_bytes(), [1, 5]);
    /// assert!(UBig::from_be_bytes(&[]).to_be_bytes().is_empty());
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }

    /// Constructs a number from its little-endian (least-significant-first) byte
    /// representation. Any length is accepted, and most-significant zero bytes are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[5, 1]), UBig::from_be_bytes(&[1, 5]));
    /// ```
    pub fn from_le_bytes(bytes: &[u8]) -> UBig {
        let mut digits = Digits::with_capacity(bytes.len().div_ceil(Digit::BYTES));
        let (chunks, remainder) = bytes.as_chunks::<{ Digit::BYTES }>();
        for &chunk in chunks {
            digits.push(Digit::from_le_bytes(chunk));
        }
        if !remainder.is_empty() {
            digits.push(digit_from_le(remainder));
        }
        UBig::from_digits(digits)
    }

    /// Constructs a number from its big-endian (most-significant-first) byte representation.
    /// Any length is accepted, and leading zero bytes are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_be_bytes(&[1, 5]), UBig::from_le_bytes(&[5, 1]));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        let mut digits = Digits::with_capacity(bytes.len().div_ceil(Digit::BYTES));
        // `as_rchunks` splits off the  most-significant chunk at the front;
        // the full chunks run most- to least-significant, so push them in reverse to get
        // little-endian digit order, then the most-significant remainder last.
        let (remainder, chunks) = bytes.as_rchunks::<{ Digit::BYTES }>();
        for &chunk in chunks.iter().rev() {
            digits.push(Digit::from_be_bytes(chunk));
        }
        if !remainder.is_empty() {
            digits.push(digit_from_be(remainder));
        }
        UBig::from_digits(digits)
    }
}

/// Builds a digit from up to [`Digit::BYTES`] little-endian bytes.
#[inline]
fn digit_from_le(chunk: &[u8]) -> Digit {
    let mut arr = [0u8; Digit::BYTES];
    arr[..chunk.len()].copy_from_slice(chunk);
    Digit::from_le_bytes(arr)
}

/// Builds a digit from up to [`Digit::BYTES`] big-endian bytes.
#[inline]
fn digit_from_be(chunk: &[u8]) -> Digit {
    let mut arr = [0u8; Digit::BYTES];
    arr[Digit::BYTES - chunk.len()..].copy_from_slice(chunk);
    Digit::from_be_bytes(arr)
}
