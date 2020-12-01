/// Create a `UBig` value.
///
/// For typical numbers: `ubig!(100)`, `ubig!(0b101)`, `ubig!(0o202)`, `ubig!(0x2ff)`.
///
/// For an arbitrary base, add `base N`, e.g.: `ubig!(100 base 13)`, `ubig!(3fp1 base 32)`.
///
/// For very large numbers the above may not compile. Then put an underscore before the digits:
/// `ubig!(_10000000000000000000000000000000000000)`,
/// `ubig!(_3ffffffffffffffffffffffffffffffffffffppp base 36)`,
/// ```
/// # use ibig::{ubig, UBig};
/// assert_eq!(ubig!(17).to_str_radix(16), "11");
/// assert_eq!(ubig!(0b101).to_str_radix(2), "101");
/// assert_eq!(ubig!(0o177).to_str_radix(8), "177");
/// assert_eq!(ubig!(_0x1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa).to_str_radix(16),
///            "1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
/// assert_eq!(ubig!(100 base 32).to_str_radix(16), "400");
/// assert_eq!(ubig!(ppppppppppppppppppp base 32).to_str_radix(32),
///            "ppppppppppppppppppp");
/// ```
#[macro_export]
macro_rules! ubig {
    ($val:ident) => {{
        let s = ::core::stringify!($val);
        let s = ::core::option::Option::unwrap_or(::core::primitive::str::strip_prefix(s, "_"), s);
        ::core::result::Result::expect(
            $crate::UBig::from_str_with_radix_prefix(s),
            "invalid number",
        )
    }};
    ($val:ident base $radix:literal) => {{
        let s = ::core::stringify!($val);
        let s = ::core::option::Option::unwrap_or(::core::primitive::str::strip_prefix(s, "_"), s);
        ::core::result::Result::expect($crate::UBig::from_str_radix(s, $radix), "invalid number")
    }};
    ($val:literal) => {{
        let val: ::core::primitive::u128 = $val;
        <$crate::UBig as ::core::convert::From<::core::primitive::u128>>::from(val)
    }};
    ($val:literal base $radix:literal) => {{
        let s = ::core::stringify!($val);
        let s = ::core::option::Option::unwrap_or(::core::primitive::str::strip_prefix(s, "_"), s);
        ::core::result::Result::expect($crate::UBig::from_str_radix(s, $radix), "invalid number")
    }};
}

/// Create an `IBig` value.
///
/// For typical numbers: `ibig!(-100)`, `ibig!(0b101)`, `ibig!(-0o202)`, `ibig!(0x2ff)`.
///
/// For an arbitrary base, add `base N`, e.g.: `ibig!(-100 base 13)`, `ibig!(fp base 32)`.
///
/// For very large numbers the above may not compile. Then put an underscore before the digits:
/// `ibig!(-_10000000000000000000000000000000000000)`,
/// `ibig!(-_3ffffffffffffffffffffffffffffffffffffppp base 36)`,
/// ```
/// # use ibig::ibig;
/// assert_eq!(ibig!(17).to_str_radix(16), "11");
/// assert_eq!(ibig!(-0b101).to_str_radix(2), "-101");
/// assert_eq!(ibig!(-0o177).to_str_radix(8), "-177");
/// assert_eq!(ibig!(-_0x1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa).to_str_radix(16),
///            "-1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
/// assert_eq!(ibig!(-100 base 32).to_str_radix(16), "-400");
/// assert_eq!(ibig!(-ppppppppppppppppppppp base 32).to_str_radix(32),
///            "-ppppppppppppppppppppp");
/// ```
#[macro_export]
macro_rules! ibig {
    (- $val:ident) => {
        - <$crate::IBig as ::core::convert::From<$crate::UBig>>::from($crate::ubig!($val))
    };
    (- $val:ident base $radix:literal) => {
        - <$crate::IBig as ::core::convert::From<$crate::UBig>>::from(
            $crate::ubig!($val base $radix)
        )
    };
    (- $val:literal) => {
        - <$crate::IBig as ::core::convert::From<$crate::UBig>>::from($crate::ubig!($val))
    };
    (- $val:literal base $radix:literal) => {
        - <$crate::IBig as ::core::convert::From<$crate::UBig>>::from(
            $crate::ubig!($val base $radix)
        )
    };
    ($val:ident) => {
        <$crate::IBig as ::core::convert::From<$crate::UBig>>::from($crate::ubig!($val))
    };
    ($val:ident base $radix:literal) => {
        <$crate::IBig as ::core::convert::From<$crate::UBig>>::from(
            $crate::ubig!($val base $radix)
        )
    };
    ($val:literal) => {
        <$crate::IBig as ::core::convert::From<$crate::UBig>>::from($crate::ubig!($val))
    };
    ($val:literal base $radix:literal) => {
        <$crate::IBig as ::core::convert::From<$crate::UBig>>::from(
            $crate::ubig!($val base $radix)
        )
    };
}
