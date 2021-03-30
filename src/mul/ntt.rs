//! Number-theoretic multiplication algorithm.

use crate::arch::word::Word;

/// A prime to be used for the number-theoretic transform.
pub(crate) struct Prime {
    /// A prime of the form k * 2^MAX_ORDER + 1.
    #[allow(dead_code)]
    pub(crate) prime: Word,
    /// max_order_root has order 2^MAX_ORDER.
    #[allow(dead_code)]
    pub(crate) max_order_root: Word,
}
