//! Conversions of UBig to and from primitive integer types.

use crate::ubig::{
    buffer::Buffer,
    word::{word_from_be_bytes_partial, word_from_le_bytes_partial, Word, WORD_BITS, WORD_BYTES},
    Repr::*,
    UBig,
};
use core::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};

impl UBig {
    /// Construct from one word.
    pub(in crate::ubig) fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Construct from little-endian bytes.
    ///
    /// ```
    /// # use ibig::UBig;
    /// assert_eq!(UBig::from_le_bytes(&[1, 2, 3]), UBig::from(0x030201u32));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_word() {
        assert_eq!(UBig::from_word(5), UBig(Small(5)));
    }

    // TODO: test for all word sizes by not using Buffer
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_from_le_bytes() {
        assert_eq!(UBig::from_le_bytes(&[]), UBig::from_word(0));
        assert_eq!(UBig::from_le_bytes(&[0; 100]), UBig::from_word(0));
        assert_eq!(UBig::from_le_bytes(&[1, 2, 3]), UBig::from_word(0x030201));
        let mut buf = Buffer::allocate(3);
        buf.push(0x0706050403020100);
        buf.push(0x0f0e0d0c0b0a0908);
        buf.push(0x10);
        let num = buf.into();
        assert_eq!(
            UBig::from_le_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            num
        );
    }

    // TODO: test for all word sizes by not using Buffer
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_from_be_bytes() {
        assert_eq!(UBig::from_be_bytes(&[]), UBig::from_word(0));
        assert_eq!(UBig::from_be_bytes(&[0; 100]), UBig::from_word(0));
        assert_eq!(UBig::from_be_bytes(&[1, 2, 3]), UBig::from_word(0x010203));
        let mut buf = Buffer::allocate(3);
        buf.push(0x0706050403020100);
        buf.push(0x0f0e0d0c0b0a0908);
        buf.push(0x10);
        let num = buf.into();
        assert_eq!(
            UBig::from_be_bytes(&[16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            num
        );
    }

    #[test]
    fn test_from_unsigned() {
        assert_eq!(UBig::from(0xf1u8), UBig::from_be_bytes(&[0xf1]));
        assert_eq!(UBig::from(0xf123u16), UBig::from_be_bytes(&[0xf1, 0x23]));
        assert_eq!(
            UBig::from(0xf1234567u32),
            UBig::from_be_bytes(&[0xf1, 0x23, 0x45, 0x67])
        );
        assert_eq!(
            UBig::from(0xf123456701234567u64),
            UBig::from_be_bytes(&[0xf1, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67])
        );
        assert_eq!(
            UBig::from(0xf1234567012345670123456701234567u128),
            UBig::from_be_bytes(&[
                0xf1, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, 0x01, 0x23,
                0x45, 0x67
            ])
        );

        assert_eq!(UBig::from(5u128), UBig::from_word(5));
        assert_eq!(UBig::from(5usize), UBig::from_word(5));
    }

    #[test]
    fn test_from_bool() {
        assert_eq!(UBig::from(false), UBig::from(0u8));
        assert_eq!(UBig::from(true), UBig::from(1u8));
    }

    #[test]
    fn test_from_char() {
        assert_eq!(UBig::from('a'), UBig::from(0x61u8));
        assert_eq!(UBig::from('Ł'), UBig::from(0x141u16));
    }

    #[test]
    fn test_from_signed() {
        assert!(UBig::try_from(-5i32).is_err());
        assert_eq!(UBig::try_from(5i32), Ok(UBig::from(5u32)));
        assert_eq!(UBig::try_from(5i128 << 120), Ok(UBig::from(5u128 << 120)));
    }
}
