use core::fmt::Debug;
use ibig::{ibig, ubig, ops::Gcd};

fn test_gcd<'a, T>(a: &'a T, b: &'a T, c: &'a T)
where
    T: Gcd<T, Output = T>,
    T: Gcd<&'a T, Output = T>,
    &'a T: Gcd<T, Output = T>,
    &'a T: Gcd<&'a T, Output = T>,
    T: Clone,
    T: Debug,
    T: Eq,
{
    assert_eq!(a.gcd(b), *c);
    assert_eq!(a.clone().gcd(b), *c);
    assert_eq!(a.gcd(b.clone()), *c);
    assert_eq!(a.clone().gcd(b.clone()), *c);    
}

#[test]
fn test_gcd_ubig() {
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
            ubig!(_0x123456789123456789123456789123456789)
        ),
        (
            ubig!(1),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(1)
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789)
        ),

        // small cases
        (ubig!(3), ubig!(7), ubig!(1)),
        (ubig!(8), ubig!(9), ubig!(1)),
        (ubig!(9), ubig!(8), ubig!(1)),
        (ubig!(42), ubig!(56), ubig!(14)),
        (ubig!(7966496), ubig!(314080416), ubig!(32)),

        // big cases
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321),
            ubig!(_0x2d)
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321987654321987654321987654321),
            ubig!(_0x2d00000002d00000002d00000002d)
        ),
        (
            ubig!(_0x5a4653ca673768565b41f775d6947d55cf3813d1), // 3^100
            ubig!(_0x1000000000000000000000000000000000000000),
            ubig!(1)
        ),
    ];

    for (a, b, c) in &test_cases {
        test_gcd(a, b, c);
        test_gcd(b, a, c);
    }
}
