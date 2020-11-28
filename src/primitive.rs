//! Primitive types.

use core::{
    convert::{TryFrom, TryInto},
    mem::size_of,
    num::TryFromIntError,
};

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("Machine architecture must be 16-bit, 32-bit or 64-bit.");

/// Machine word.
pub(crate) type Word = usize;

#[cfg(target_pointer_width = "16")]
/// Double machine word.
pub(crate) type DoubleWord = u32;
#[cfg(target_pointer_width = "32")]
/// Double machine word.
pub(crate) type DoubleWord = u64;
#[cfg(target_pointer_width = "64")]
/// Double machine word.
pub(crate) type DoubleWord = u128;

pub(crate) trait PrimitiveUnsigned
where
    Self: Copy,
    Self: Default,
    Self: TryFrom<Word>,
    Self: TryInto<Word>,
{
    const BYTE_SIZE: usize = size_of::<Self>();
    const BIT_SIZE: u32 = 8 * Self::BYTE_SIZE as u32;
    type ByteRepr: AsRef<[u8]> + AsMut<[u8]>;

    fn to_le_bytes(self) -> Self::ByteRepr;
    fn from_le_bytes(repr: Self::ByteRepr) -> Self;
}

pub(crate) trait PrimitiveSigned
where
    Self: Copy,
    Self: TryFrom<Word, Error = TryFromIntError>,
    Self::Unsigned: PrimitiveUnsigned,
    Self::Unsigned: TryFrom<Self, Error = TryFromIntError>,
    Self::Unsigned: TryInto<Self, Error = TryFromIntError>,
{
    type Unsigned;
}

macro_rules! impl_primitive_unsigned {
    ($t:ty) => {
        impl PrimitiveUnsigned for $t {
            type ByteRepr = [u8; size_of::<$t>()];

            fn to_le_bytes(self) -> Self::ByteRepr {
                self.to_le_bytes()
            }

            fn from_le_bytes(repr: Self::ByteRepr) -> Self {
                Self::from_le_bytes(repr)
            }
        }
    };
}

impl_primitive_unsigned!(u8);
impl_primitive_unsigned!(u16);
impl_primitive_unsigned!(u32);
impl_primitive_unsigned!(u64);
impl_primitive_unsigned!(u128);
impl_primitive_unsigned!(usize);

macro_rules! impl_primitive_signed {
    ($t:ty, $u:ty) => {
        impl PrimitiveSigned for $t {
            type Unsigned = $u;
        }
    };
}

impl_primitive_signed!(i8, u8);
impl_primitive_signed!(i16, u16);
impl_primitive_signed!(i32, u32);
impl_primitive_signed!(i64, u64);
impl_primitive_signed!(i128, u128);
impl_primitive_signed!(isize, usize);

pub(crate) const WORD_BITS: u32 = Word::BIT_SIZE;
pub(crate) const WORD_BYTES: usize = Word::BYTE_SIZE;

pub(crate) fn word_from_le_bytes_partial(bytes: &[u8]) -> Word {
    let mut word_bytes = [0; WORD_BYTES];
    word_bytes[..bytes.len()].copy_from_slice(bytes);
    Word::from_le_bytes(word_bytes)
}

pub(crate) fn word_from_be_bytes_partial(bytes: &[u8]) -> Word {
    let mut word_bytes = [0; WORD_BYTES];
    word_bytes[Word::BYTE_SIZE - bytes.len()..].copy_from_slice(bytes);
    Word::from_be_bytes(word_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_bytes() {
        assert_eq!(u8::BIT_SIZE, 8);
        assert_eq!(u64::BIT_SIZE, 64);
        assert_eq!(u8::BYTE_SIZE, 1);
        assert_eq!(u64::BYTE_SIZE, 8);
    }

    #[test]
    fn test_word_from_le_bytes_partial() {
        assert_eq!(word_from_le_bytes_partial(&[1, 2]), 0x0201);
    }

    #[test]
    fn test_word_from_be_bytes_partial() {
        assert_eq!(word_from_be_bytes_partial(&[1, 2]), 0x0102);
    }
}
