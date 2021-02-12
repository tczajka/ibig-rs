use core::{
    fmt::Debug,
    ops::{Mul, MulAssign},
};
use ibig::prelude::*;

fn test_mul<'a, T>(a: &'a T, b: &'a T, c: &'a T)
where
    T: Mul<T, Output = T>,
    T: Mul<&'a T, Output = T>,
    &'a T: Mul<T, Output = T>,
    &'a T: Mul<&'a T, Output = T>,
    T: MulAssign<T>,
    T: MulAssign<&'a T>,
    T: Clone,
    T: Debug,
    T: Eq,
{
    assert_eq!(a * b, *c);
    assert_eq!(a.clone() * b, *c);
    assert_eq!(a * b.clone(), *c);
    assert_eq!(a.clone() * b.clone(), *c);

    let mut x = a.clone();
    x *= b;
    assert_eq!(x, *c);

    let mut x = a.clone();
    x *= b.clone();
    assert_eq!(x, *c);
}

#[test]
fn test_mul_ubig() {
    let test_cases = [
        (ubig!(0), ubig!(4), ubig!(0)),
        (ubig!(3), ubig!(4), ubig!(12)),
        (ubig!(0x123456789abc), ubig!(0x444333222111fff), ubig!(0x4daae4d8531f8de7e1fb5ae544)),
        (ubig!(0), ubig!(1) << 100, ubig!(0)),
        (
            ubig!(1),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789)
        ),
        (
            ubig!(0x10),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x1234567891234567891234567891234567890)
        ),
        (
            ubig!(0x1000000000000000),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789000000000000000)
        ),
        (
            ubig!(_0x123456789123456789123456789123456789123456789123456789),
            ubig!(_0xabcdefabcdefabcdefabcdefabcdef),
            ubig!(_0xc379ab6dbd40ef67e528bfffd3039491348e20491348e20491348d5ccf67db24c3a1cca8f7891375de7)
        ),
    ];

    for (a, b, c) in &test_cases {
        test_mul(a, b, c);
        test_mul(b, a, c);
    }
}
