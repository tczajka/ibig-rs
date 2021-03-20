//! Architecture dependent functionality.

pub(crate) use arch_impl::add;
pub(crate) use arch_impl::digits;
pub(crate) use arch_impl::word;

/// Architecture choice. The logic works like this:
/// 1. If the configuration option force_bits is set to 16, 32 or 64, use generic_<n>_bit.
/// 2. Otherwise if target_arch is known, select that architecture.
/// 3. Otherwise target_pointer_width is 16, 32, use generic_<n>_bit.
/// 4. Otherwise, use generic_64_bit.
// Step 1. Check force_bits.
#[cfg_attr(force_bits = "16", path = "generic_16_bit/mod.rs")]
#[cfg_attr(force_bits = "32", path = "generic_32_bit/mod.rs")]
#[cfg_attr(force_bits = "64", path = "generic_64_bit/mod.rs")]
// Step 2. Specific architectures.
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
#[cfg_attr(
    all(
        any(
            target_arch = "arm",
            target_arch = "mips",
            target_arch = "powerpc",
            target_arch = "sparc",
            target_arch = "wasm32",
        ),
        not(any(force_bits = "16", force_bits = "32", force_bits = "64"))
    ),
    path = "generic_32_bit/mod.rs"
)]
#[cfg_attr(
    all(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        not(any(force_bits = "16", force_bits = "32", force_bits = "64"))
    ),
    path = "generic_64_bit/mod.rs"
)]
// Step 3. target_pointer_width 16 or 32.
#[cfg_attr(
    all(
        target_pointer_width = "16",
        not(any(
            force_bits = "16",
            force_bits = "32",
            force_bits = "64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "mips",
            target_arch = "powerpc",
            target_arch = "sparc",
            target_arch = "wasm32",
            target_arch = "aarch64",
            target_arch = "mips64",
            target_arch = "powerpc64",
        )),
    ),
    path = "generic_16_bit/mod.rs"
)]
#[cfg_attr(
    all(
        target_pointer_width = "32",
        not(any(
            force_bits = "16",
            force_bits = "32",
            force_bits = "64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "mips",
            target_arch = "powerpc",
            target_arch = "sparc",
            target_arch = "wasm32",
            target_arch = "aarch64",
            target_arch = "mips64",
            target_arch = "powerpc64",
        )),
    ),
    path = "generic_32_bit/mod.rs"
)]
// Step 4. Fall back on generic_64_bit.
#[cfg_attr(
    not(any(
        force_bits = "16",
        force_bits = "32",
        force_bits = "64",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "powerpc",
        target_arch = "sparc",
        target_arch = "wasm32",
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_pointer_width = "16",
        target_pointer_width = "32"
    )),
    path = "generic_64_bit/mod.rs"
)]
mod arch_impl;
