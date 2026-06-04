//! Internal macros.

/// Implements the commutative binary operator `&T op T` by delegating to the existing
/// `T op &T` implementation, relying on commutativity (`&a op b == b op &a`). `$trait` is the
/// operator trait, which must be in scope at the call site.
macro_rules! forward_commutative_ref_val {
    ($t:ty, $trait:ident::$method:ident) => {
        impl $trait<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> $t {
                $trait::$method(rhs, self)
            }
        }
    };
}

pub(crate) use forward_commutative_ref_val;
