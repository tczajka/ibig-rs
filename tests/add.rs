use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use ibig::{ibig, ubig};

/// Test a + b = c in various ways.
fn test_add_sub<'a, T>(a: &'a T, b: &'a T, c: &'a T)
where
    T: Add<T, Output = T>,
    T: Add<&'a T, Output = T>,
    &'a T: Add<T, Output = T>,
    &'a T: Add<&'a T, Output = T>,
    T: AddAssign<T>,
    T: AddAssign<&'a T>,
    T: Sub<T, Output = T>,
    T: Sub<&'a T, Output = T>,
    &'a T: Sub<T, Output = T>,
    &'a T: Sub<&'a T, Output = T>,
    T: SubAssign<T>,
    T: SubAssign<&'a T>,
    T: Clone,
    T: Debug,
    T: Eq,
{
    assert_eq!(a + b, *c);
    assert_eq!(a.clone() + b, *c);
    assert_eq!(a + b.clone(), *c);
    assert_eq!(a.clone() + b.clone(), *c);

    let mut x = a.clone();
    x += b;
    assert_eq!(x, *c);

    let mut x = a.clone();
    x += b.clone();
    assert_eq!(x, *c);

    assert_eq!(c - a, *b);
    assert_eq!(c.clone() - a, *b);
    assert_eq!(c - a.clone(), *b);
    assert_eq!(c.clone() - a.clone(), *b);

    let mut x = c.clone();
    x -= a;
    assert_eq!(x, *b);

    let mut x = c.clone();
    x -= a.clone();
    assert_eq!(x, *b);
}

#[test]
fn test_add_sub_ubig() {
    let test_cases = [
        (ubig!(3), ubig!(4), ubig!(7)),
        (
            ubig!(0xffffffffffffffff),
            ubig!(1),
            ubig!(0x10000000000000000),
        ),
        (
            ubig!(0x10000000000000003),
            ubig!(4),
            ubig!(0x10000000000000007),
        ),
        (
            ubig!(0xeeeeeeeeeeeeeeeeffffffffffffffff),
            ubig!(1),
            ubig!(0xeeeeeeeeeeeeeeef0000000000000000),
        ),
        (
            ubig!(0xeeeeeeeeeeeeeeeeffffffffffffffff),
            ubig!(1),
            ubig!(0xeeeeeeeeeeeeeeef0000000000000000),
        ),
        (
            ubig!(0xffffffffffffffffffffffffffffffff),
            ubig!(2),
            ubig!(_0x100000000000000000000000000000001),
        ),
        (
            ubig!(0x88888888888888888888888888888888),
            ubig!(0x88888888888888888888888888888888),
            ubig!(_0x111111111111111111111111111111110),
        ),
        (
            ubig!(_0x888888888888888888888888888888888888888888888888),
            ubig!(0x88888888888888888888888888888888),
            ubig!(_0x888888888888888911111111111111111111111111111110),
        ),
        (
            ubig!(_0x888888888888888888888888888888888888888888888888),
            ubig!(0),
            ubig!(_0x888888888888888888888888888888888888888888888888),
        ),
    ];

    for (a, b, c) in &test_cases {
        test_add_sub(a, b, c);
        test_add_sub(b, a, c);
    }
}

#[test]
#[should_panic]
fn test_sub_ubig_overflow() {
    let _ = ubig!(3) - ubig!(4);
}

#[test]
fn test_add_sub_ibig() {
    let test_cases = [
        (ibig!(3), ibig!(4), ibig!(7)),
        (ibig!(3), ibig!(-4), ibig!(-1)),
        (ibig!(-3), ibig!(4), ibig!(1)),
        (ibig!(-3), ibig!(-4), ibig!(-7)),
        (
            ibig!(0x10000000000000000),
            ibig!(-4),
            ibig!(0xfffffffffffffffc),
        ),
        (
            ibig!(0x10000000000000000),
            ibig!(_0x200000000000000000000000000000000),
            ibig!(_0x200000000000000010000000000000000),
        ),
        (
            ibig!(-_0x200000000000000010000000000000000),
            ibig!(_0x200000000000000000000000000000000),
            ibig!(-0x10000000000000000),
        ),
        (
            ibig!(_0x200000000000000010000000000000000),
            ibig!(-_0x200000000000000000000000000000000),
            ibig!(0x10000000000000000),
        ),
    ];

    for (a, b, c) in &test_cases {
        test_add_sub(a, b, c);
        test_add_sub(b, a, c);
    }
}
