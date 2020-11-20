//! Machine word operations.

#[cfg(not(any(
    target_pointer_width="16",
    target_pointer_width="32",
    target_pointer_width="64")))]
compile_error!("Machine architecture must be 16-bit, 32-bit or 64-bit.");

/// Machine word.
pub type Word = usize;

#[cfg(target_pointer_width="16")]
/// Double machine word.
pub type DoubleWord = u32;
#[cfg(target_pointer_width="32")]
/// Double machine word.
pub type DoubleWord = u64;
#[cfg(target_pointer_width="64")]
/// Double machine word.
pub type DoubleWord = u128;

pub const WORD_BITS: u32 = (0 as Word).trailing_zeros();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_word_size() {
        assert!(WORD_BITS == 64);
        assert_eq!((0 as DoubleWord).trailing_zeros(), 2 * WORD_BITS);
    }
}
