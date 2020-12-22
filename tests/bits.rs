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
fn test_set_bit() {
    let mut a = ubig!(0);
    a.set_bit(3);
    assert_eq!(a, ubig!(0b1000));
    a.set_bit(129);
    assert_eq!(a, ubig!(_0x200000000000000000000000000000008));
    a.set_bit(1);
    assert_eq!(a, ubig!(_0x20000000000000000000000000000000a));
    a.set_bit(1);
    assert_eq!(a, ubig!(_0x20000000000000000000000000000000a));
    a.set_bit(127);
    assert_eq!(a, ubig!(_0x28000000000000000000000000000000a));
    a.set_bit(194);
    assert_eq!(
        a,
        ubig!(_0x400000000000000028000000000000000000000000000000a)
    );
}

#[test]
fn test_clear_bit() {
    let mut a = ubig!(_0x400000000000000028000000000000000000000000000000a);
    a.clear_bit(10000);
    assert_eq!(
        a,
        ubig!(_0x400000000000000028000000000000000000000000000000a)
    );
    a.clear_bit(194);
    assert_eq!(a, ubig!(_0x28000000000000000000000000000000a));
    a.clear_bit(1);
    assert_eq!(a, ubig!(_0x280000000000000000000000000000008));
    a.clear_bit(129);
    assert_eq!(a, ubig!(_0x80000000000000000000000000000008));
    a.clear_bit(127);
    assert_eq!(a, ubig!(0b1000));
    a.clear_bit(3);
    assert_eq!(a, ubig!(0));
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

#[test]
fn test_is_power_of_two() {
    assert_eq!(ubig!(0).is_power_of_two(), false);
    assert_eq!(ubig!(1).is_power_of_two(), true);
    assert_eq!(ubig!(16).is_power_of_two(), true);
    assert_eq!(ubig!(17).is_power_of_two(), false);
    assert_eq!(ubig!(_0x4000000000000000000000000000000000000000000000).is_power_of_two(), true);
    assert_eq!(ubig!(_0x5000000000000000000000000000000000000000000000).is_power_of_two(), false);
    assert_eq!(ubig!(_0x4000000000000000000000010000000000000000000000).is_power_of_two(), false);
}
