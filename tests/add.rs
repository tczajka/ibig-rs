use ibig::prelude::*;

#[test]
fn test_add_ubig() {
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
    ];

    let test = |a: &UBig, b: &UBig, c: &UBig| {
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
    };

    for (a, b, c) in &test_cases {
        test(a, b, c);
        test(b, a, c);
    }
}
