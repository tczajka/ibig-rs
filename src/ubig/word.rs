//! Machine word operations.

use core::mem::size_of;

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("Machine architecture must be 16-bit, 32-bit or 64-bit.");

/// Machine word.
pub type Word = usize;

#[cfg(target_pointer_width = "16")]
/// Double machine word.
pub type DoubleWord = u32;
#[cfg(target_pointer_width = "32")]
/// Double machine word.
pub type DoubleWord = u64;
#[cfg(target_pointer_width = "64")]
/// Double machine word.
pub type DoubleWord = u128;

pub const WORD_BYTES: usize = size_of::<Word>();
pub const WORD_BITS: usize = WORD_BYTES * 8;

pub fn word_from_le_bytes_partial(bytes: &[u8]) -> Word {
    let mut word_bytes = [0; WORD_BYTES];
    word_bytes[..bytes.len()].copy_from_slice(bytes);
    Word::from_le_bytes(word_bytes)
}

pub fn word_from_be_bytes_partial(bytes: &[u8]) -> Word {
    let mut word_bytes = [0; WORD_BYTES];
    word_bytes[WORD_BYTES - bytes.len()..].copy_from_slice(bytes);
    Word::from_be_bytes(word_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_from_le_bytes_partial() {
        assert_eq!(word_from_le_bytes_partial(&[1, 2, 3]), 0x030201);
    }

    #[test]
    fn test_word_from_be_bytes_partial() {
        assert_eq!(word_from_be_bytes_partial(&[1, 2, 3]), 0x010203);
    }
}
