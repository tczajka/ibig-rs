//! Conversions of UBig to and from primitive integer types.

use super::{
    buffer::Buffer,
    word::{word_from_be_bytes_partial, word_from_le_bytes_partial, Word, WORD_BITS, WORD_BYTES},
    Repr::*,
    UBig,
};
use alloc::vec::Vec;
use core::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};

impl UBig {
    /// Construct from one word.
    pub(super) fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Construct from little-endian bytes.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[3, 2, 1]), UBig::from(0x010203u32));
    /// ```
    #[inline]
    pub fn from_le_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(word_from_le_bytes_partial(bytes))
        } else {
            UBig::from_le_bytes_large(bytes)
        }
    }

    fn from_le_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        // TODO: Switch to bytes.array_chunks when the API is stable.
        let mut chunks = bytes.chunks_exact(WORD_BYTES);
        let mut buffer = Buffer::allocate(chunks.len() + 1);
        for chunk in &mut chunks {
            buffer.push(Word::from_le_bytes(chunk.try_into().unwrap()));
        }
        buffer.push(word_from_le_bytes_partial(chunks.remainder()));
        buffer.into()
    }

    /// Construct from big-endian bytes.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_be_bytes(&[1, 2, 3]), UBig::from(0x010203u32));
    /// ```
    #[inline]
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(word_from_be_bytes_partial(bytes))
        } else {
            UBig::from_be_bytes_large(bytes)
        }
    }

    fn from_be_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        // TODO: Switch to bytes.array_chunks when the API is stable.
        let mut chunks = bytes.rchunks_exact(WORD_BYTES);
        let mut buffer = Buffer::allocate(chunks.len() + 1);
        for chunk in &mut chunks {
            buffer.push(Word::from_be_bytes(chunk.try_into().unwrap()));
        }
        buffer.push(word_from_be_bytes_partial(chunks.remainder()));
        buffer.into()
    }

    /// Return little-endian bytes.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert!(UBig::from(0u32).to_le_bytes().is_empty());
    /// assert_eq!(UBig::from(0x010203u32).to_le_bytes(), [3, 2, 1]);
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match self.0 {
            Small(x) => {
                let bytes = x.to_le_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[..WORD_BYTES - skip_bytes].to_vec()
            }
            Large(ref buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                for word in &buffer[..n - 1] {
                    bytes.extend_from_slice(&word.to_le_bytes());
                }
                let last_bytes = last.to_le_bytes();
                bytes.extend_from_slice(&last_bytes[..WORD_BYTES - skip_last_bytes]);
                bytes
            }
        }
    }

    /// Return big-endian bytes.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert!(UBig::from(0u32).to_be_bytes().is_empty());
    /// assert_eq!(UBig::from(0x010203u32).to_be_bytes(), [1, 2, 3]);
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self.0 {
            Small(x) => {
                let bytes = x.to_be_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[skip_bytes..].to_vec()
            }
            Large(ref buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                let last_bytes = last.to_be_bytes();
                bytes.extend_from_slice(&last_bytes[skip_last_bytes..]);
                for word in buffer[..n - 1].iter().rev() {
                    bytes.extend_from_slice(&word.to_be_bytes());
                }
                bytes
            }
        }
    }
}

macro_rules! impl_from_unsigned {
    ($t:ty) => {
        impl From<$t> for UBig {
            fn from(x: $t) -> UBig {
                match Word::try_from(x) {
                    Ok(w) => UBig::from_word(w),
                    Err(_) => {
                        let n = (size_of::<$t>() * 8 - x.leading_zeros() as usize
                            + (WORD_BITS - 1))
                            / WORD_BITS;
                        let mut buffer = Buffer::allocate(n);
                        let mut remaining_bits = x;
                        // Makes the shift non-constant to silence error for smaller bit sizes where
                        // we never reach this loop.
                        let shift = WORD_BITS;
                        for _ in 0..n {
                            buffer.push(remaining_bits as Word);
                            remaining_bits >>= shift;
                        }
                        debug_assert!(*buffer.last().unwrap() != 0);
                        debug_assert!(remaining_bits == 0);
                        buffer.into()
                    }
                }
            }
        }
    };
}

impl_from_unsigned!(u8);
impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(u128);
impl_from_unsigned!(usize);

impl From<bool> for UBig {
    fn from(b: bool) -> UBig {
        u8::from(b).into()
    }
}

impl From<char> for UBig {
    fn from(c: char) -> UBig {
        u32::from(c).into()
    }
}

macro_rules! impl_from_signed {
    ($t:ty as $u:ty) => {
        impl TryFrom<$t> for UBig {
            type Error = <$u as TryFrom<$t>>::Error;

            fn try_from(x: $t) -> Result<UBig, Self::Error> {
                let y = <$u as TryFrom<$t>>::try_from(x)?;
                Ok(y.into())
            }
        }
    };
}

impl_from_signed!(i8 as u8);
impl_from_signed!(i16 as u16);
impl_from_signed!(i32 as u32);
impl_from_signed!(i64 as u64);
impl_from_signed!(i128 as u128);
impl_from_signed!(isize as usize);
