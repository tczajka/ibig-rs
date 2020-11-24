//! Machine word.

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

pub(crate) const WORD_BITS: u32 = (0 as Word).trailing_zeros();
pub(crate) const WORD_BYTES: usize = (WORD_BITS / 8) as usize;
