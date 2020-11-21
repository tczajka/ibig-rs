//! Machine word operations.

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("Machine architecture must be 16-bit, 32-bit or 64-bit.");

/// Number of bits in a type.
pub const fn bit_size<T>() -> usize {
    core::mem::size_of::<T>() * 8
}

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

pub const WORD_BITS: usize = bit_size::<Word>();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_size() {
        assert_eq!(bit_size::<u32>(), 32);
        assert_eq!(bit_size::<DoubleWord>(), 2 * WORD_BITS);
    }
}
