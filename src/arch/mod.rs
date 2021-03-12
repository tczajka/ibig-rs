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

#[cfg(all(feature = "force-16-bit", feature = "force-32-bit"))]
compile_error!("features force-16-bit and force-32-bit are mutually exclusive");

#[cfg(all(feature = "force-16-bit", feature = "force-64-bit"))]
compile_error!("features force-16-bit and force-64-bit are mutually exclusive");

#[cfg(all(feature = "force-32-bit", feature = "force-64-bit"))]
compile_error!("features force-32-bit and force-64-bit are mutually exclusive");

#[cfg_attr(
    all(
        target_arch = "x86",
        not(any(
            feature = "force-16-bit",
            feature = "force-32-bit",
            feature = "force-64-bit"
        ))
    ),
    path = "x86/mod.rs"
)]
#[cfg_attr(
    all(
        target_arch = "x86_64",
        not(any(
            feature = "force-16-bit",
            feature = "force-32-bit",
            feature = "force-64-bit"
        ))
    ),
    path = "x86_64/mod.rs"
)]
#[cfg_attr(
    any(
        all(
            target_pointer_width = "16",
            not(any(
                target_arch = "x86",
                target_arch = "x86_64",
                feature = "force-16-bit",
                feature = "force-32-bit",
                feature = "force-64-bit"
            )),
        ),
        feature = "force-16-bit",
    ),
    path = "generic_16_bit/mod.rs"
)]
#[cfg_attr(
    any(
        all(
            target_pointer_width = "32",
            not(any(
                target_arch = "x86",
                target_arch = "x86_64",
                feature = "force-16-bit",
                feature = "force-32-bit",
                feature = "force-64-bit"
            )),
        ),
        feature = "force-32-bit"
    ),
    path = "generic_32_bit/mod.rs"
)]
#[cfg_attr(
    any(
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            feature = "force-16-bit",
            feature = "force-32-bit",
            feature = "force-64-bit",
            target_pointer_width = "16",
            target_pointer_width = "32"
        )),
        feature = "force-64-bit"
    ),
    path = "generic_64_bit/mod.rs"
)]
mod arch_impl;
