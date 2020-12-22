use ibig::prelude::*;

#[test]
fn test_bit() {
    assert_eq!(ubig!(0).bit(0), false);
    assert_eq!(ubig!(0).bit(1000), false);
    assert_eq!(ubig!(0b11101).bit(0), true);
    assert_eq!(ubig!(0b11101).bit(1), false);
    assert_eq!(ubig!(0b11101).bit(4), true);
    assert_eq!(ubig!(0b11101).bit(5), false);
    assert_eq!(ubig!(0b11101).bit(1000), false);

    assert_eq!(ubig!(_0xffffffffffffffffffffffffffffffff).bit(127), true);
    assert_eq!(ubig!(_0xffffffffffffffffffffffffffffffff).bit(128), false);
}

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
