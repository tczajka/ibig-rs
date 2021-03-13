//! Architecture dependent things.

/// Machine word.
pub(crate) use arch_impl::word::Word;

/// Signed machine word.
pub(crate) use arch_impl::word::SignedWord;

/// Double machine word.
pub(crate) use arch_impl::word::DoubleWord;

/// fn add_with_carry(a: Word, b: Word, carry: bool) -> (Word, bool)
///
/// Add a + b + carry.
///
/// Returns (result, overflow).
pub(crate) use arch_impl::add::add_with_carry;

/// fn sub_with_borrow(a: Word, b: Word, borrow: bool) -> (Word, bool) {
///
/// Subtract a - b - borrow.
///
/// Returns (result, overflow).
pub(crate) use arch_impl::add::sub_with_borrow;

/// Architecture choice. The logic works like this:
/// 1. If the configuration option force_bits is set to 16, 32 or 64, use generic_<n>_bit.
/// 2. Otherwise if target_arch is "x86" or "x86_64", select that architecture.
/// 3. Otherwise target_pointer_width is 16, 32 or 64, use generic_<n>_bit.
/// 4. Otherwise, use generic_64_bit.
#[cfg_attr(
    any(
        force_bits = "16",
        all(
            target_pointer_width = "16",
            not(any(
                force_bits = "16",
                force_bits = "32",
                force_bits = "64",
                target_arch = "x86",
                target_arch = "x86_64",
            )),
        ),
    ),
    path = "generic_16_bit/mod.rs"
)]
#[cfg_attr(
    any(
        force_bits = "32",
        all(
            target_pointer_width = "32",
            not(any(
                force_bits = "16",
                force_bits = "32",
                force_bits = "64",
                target_arch = "x86",
                target_arch = "x86_64",
            )),
        ),
    ),
    path = "generic_32_bit/mod.rs"
)]
#[cfg_attr(
    any(
        force_bits = "64",
        not(any(
            force_bits = "16",
            force_bits = "32",
            force_bits = "64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_pointer_width = "16",
            target_pointer_width = "32"
        )),
    ),
    path = "generic_64_bit/mod.rs"
)]
#[cfg_attr(
    all(
        target_arch = "x86",
        not(any(force_bits = "16", force_bits = "32", force_bits = "64"))
    ),
    path = "x86/mod.rs"
)]
#[cfg_attr(
    all(
        target_arch = "x86_64",
        not(any(force_bits = "16", force_bits = "32", force_bits = "64"))
    ),
    path = "x86_64/mod.rs"
)]
mod arch_impl;
