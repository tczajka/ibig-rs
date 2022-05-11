use core::fmt::Debug;
use core::ops::{Add, Mul};
use ibig::{
    ops::{ExtendedGcd, Gcd},
    ubig,
};

fn test_gcd<'a, T, U>(a: &'a T, b: &'a T, c: &'a T)
where
    T: Gcd<T, Output = T> + ExtendedGcd<T, OutputGcd = T, OutputCoeff = U>,
    T: Gcd<&'a T, Output = T> + ExtendedGcd<&'a T, OutputGcd = T, OutputCoeff = U>,
    &'a T: Gcd<T, Output = T> + ExtendedGcd<T, OutputGcd = T, OutputCoeff = U>,
    &'a T: Gcd<&'a T, Output = T> + ExtendedGcd<&'a T, OutputGcd = T, OutputCoeff = U>,
    T: Clone + Debug + Eq,
    U: From<T> + Clone + Debug + Eq,
    U: Add<U, Output = U> + Mul<U, Output = U>,
{
    assert_eq!(a.gcd(b), *c);
    assert_eq!(a.clone().gcd(b), *c);
    assert_eq!(a.gcd(b.clone()), *c);
    assert_eq!(a.clone().gcd(b.clone()), *c);

    let (g, x, y) = a.extended_gcd(b);
    assert_eq!(&g, c);
    assert_eq!(x * U::from(a.clone()) + y * U::from(b.clone()), U::from(g));

    let (g, x, y) = a.extended_gcd(b.clone());
    assert_eq!(&g, c);
    assert_eq!(x * U::from(a.clone()) + y * U::from(b.clone()), U::from(g));
}

#[test]
fn test_gcd_ubig() {
    // test cases (x, y, gcd(x,y))
    let test_cases = [
        // trivial cases
        (ubig!(0), ubig!(0), ubig!(0)),
        (ubig!(0), ubig!(123), ubig!(123)),
        (ubig!(123), ubig!(0), ubig!(123)),
        (ubig!(1), ubig!(123), ubig!(1)),
        (ubig!(123), ubig!(1), ubig!(1)),
        (ubig!(123), ubig!(123), ubig!(123)),
        (
            ubig!(0),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
        ),
        (
            ubig!(1),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(1),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
        ),
        // small cases
        (ubig!(3), ubig!(7), ubig!(1)),
        (ubig!(8), ubig!(9), ubig!(1)),
        (ubig!(9), ubig!(8), ubig!(1)),
        (ubig!(42), ubig!(56), ubig!(14)),
        (ubig!(7966496), ubig!(314080416), ubig!(32)),
        // big cases
        (
            ubig!(0xffffffffffffffffffffffff1), // largest prime under 2^100
            ubig!(0x7ffffffffffffffffffffff8d), // largest prime under 2^99
            ubig!(1),
        ),
        (
            ubig!(0xffffffffffffffffffffffffffffff61), // largest prime under 2^128
            ubig!(0xffffffffffffffffffffffffffffff53), // second largest prime under 2^128
            ubig!(1),
        ),
        (
            ubig!(_0x3ffffffffffffffffffffffffffffffffffffd), // largest prime under 2^150
            ubig!(_0x1fffffffffffffffffffffffffffffffffffe1), // largest prime under 2^149
            ubig!(1),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321),
            ubig!(_0x2d),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321987654321987654321987654321),
            ubig!(_0x2d00000002d00000002d00000002d),
        ),
        (
            ubig!(_0x5a4653ca673768565b41f775d6947d55cf3813d1), // 3^100
            ubig!(_0x1000000000000000000000000000000000000000),
            ubig!(1),
        ),
    ];

    for (a, b, c) in &test_cases {
        test_gcd(a, b, c);
        test_gcd(b, a, c);
    }
}
