macro_rules! impl_unary_operator {
    (impl $trait:ident for $t:ty, $f:ident, $f_cow:ident) => {
        impl $trait for $t {
            type Output = $t;

            #[inline]
            fn $f(self) -> $t {
                $f_cow(::alloc::borrow::Cow::Owned(self))
            }
        }

        impl $trait for &$t {
            type Output = $t;

            #[inline]
            fn $f(self) -> $t {
                $f_cow(::alloc::borrow::Cow::Borrowed(self))
            }
        }
    };
}
