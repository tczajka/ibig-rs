use ibig::prelude::*;

#[test]
fn test_trailing_zeros() {
    assert_eq!(ubig!(0).trailing_zeros(), None);
    assert_eq!(ubig!(0xf0000).trailing_zeros(), Some(16));
    assert_eq!(
        ubig!(_0xfffffffffffffffffffff00000000000000000000000000000000000000000000000000)
            .trailing_zeros(),
        Some(200)
    );

    assert_eq!(ibig!(0).trailing_zeros(), None);
    assert_eq!(ibig!(0xf0000).trailing_zeros(), Some(16));
    assert_eq!(ibig!(-0xf0000).trailing_zeros(), Some(16));
}

#[test]
fn test_ilog2() {
    assert_eq!(ubig!(0).ilog2(), None);
    assert_eq!(ubig!(0xf0000).ilog2(), Some(19));
    assert_eq!(
        ubig!(_0xfffffffffffffffffffff00000000000000000000000000000000000000000000000000).ilog2(),
        Some(283)
    );
}
