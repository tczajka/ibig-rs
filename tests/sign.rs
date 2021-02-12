use core::cmp::Ordering;
use ibig::prelude::*;

#[test]
fn test_neg() {
    assert_eq!(-ibig!(123), ibig!(-123));
    assert_eq!(-ibig!(-123), ibig!(123));
    assert_eq!(-ibig!(0), ibig!(0));

    assert_eq!(-&ibig!(123), ibig!(-123));
}

#[test]
fn test_abs() {
    assert_eq!(ibig!(123).abs(), ibig!(123));
    assert_eq!(ibig!(-123).abs(), ibig!(123));

    assert_eq!((&ibig!(-123)).abs(), ibig!(123));
}

#[test]
fn test_unsigned_abs() {
    assert_eq!(ibig!(123).unsigned_abs(), ubig!(123));
    assert_eq!(ibig!(-123).unsigned_abs(), ubig!(123));

    assert_eq!((&ibig!(-123)).unsigned_abs(), ubig!(123));
}

#[test]
fn test_signum() {
    assert_eq!(ibig!(-500).signum(), ibig!(-1));
    assert_eq!(ibig!(0).signum(), ibig!(0));
    assert_eq!(ibig!(500).signum(), ibig!(1));
}

#[test]
fn test_cmp() {
    assert_eq!(ubig!(500).cmp(&ubig!(500)), Ordering::Equal);
    assert!(ubig!(100) < ubig!(500));
    assert!(ubig!(500) > ubig!(100));
    assert!(ubig!(0x10000000000000000) > ubig!(100));
    assert!(ubig!(100) < ubig!(_0x100000000000000000000000000000000));
    assert!(
        ubig!(_0x100000000000000020000000000000003) < ubig!(_0x100000000000000030000000000000002)
    );
    assert!(
        ubig!(_0x100000000000000030000000000000002) > ubig!(_0x100000000000000020000000000000003)
    );
    assert_eq!(
        ubig!(_0x100000000000000030000000000000002)
            .cmp(&ubig!(_0x100000000000000030000000000000002)),
        Ordering::Equal
    );

    assert_eq!(ibig!(500).cmp(&ibig!(500)), Ordering::Equal);
    assert_eq!(ibig!(-500).cmp(&ibig!(-500)), Ordering::Equal);
    assert!(ibig!(5) < ibig!(10));
    assert!(ibig!(10) > ibig!(5));
    assert!(ibig!(-5) < ibig!(10));
    assert!(ibig!(-15) < ibig!(10));
    assert!(ibig!(10) > ibig!(-5));
    assert!(ibig!(10) > ibig!(-15));
    assert!(ibig!(-10) < ibig!(-5));
    assert!(ibig!(-5) > ibig!(-10));
}
