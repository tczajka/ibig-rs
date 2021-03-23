use ibig::{modular::ModuloRing, prelude::*};

#[test]
fn test_modulus() {
    let ring = ModuloRing::new(&ubig!(100));
    assert_eq!(ring.modulus(), ubig!(100));

    let ring = ModuloRing::new(&ubig!(10).pow(100));
    assert_eq!(ring.modulus(), ubig!(10).pow(100));
}

#[test]
fn test_clone() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let x = ring1.from(512);
    let y = x.clone();
    assert!(x == y);
    let mut z = ring1.from(513);
    assert!(x != z);
    z.clone_from(&x);
    assert!(x == z);

    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring2.from(512);
    let y = x.clone();
    assert!(x == y);
    let mut z = ring2.from(513);
    assert!(x != z);
    z.clone_from(&x);
    assert!(x == z);

    let mut x = ring1.from(512);
    let y = ring2.from(1);
    x.clone_from(&y);
    assert!(x == y);

    let ring3 = ModuloRing::new(&ubig!(10).pow(100));
    let x = ring2.from(1);
    let mut y = ring3.from(2);
    y.clone_from(&x);
    assert!(x == y);
}

#[test]
fn test_convert() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(6);
    assert!(x == ring.from(&ubig!(306)));
    assert!(x != ring.from(&ubig!(313)));
    assert!(x == ring.from(&ubig!(_18297381723918723981723981723906)));
    assert!(x != ring.from(&ubig!(_18297381723918723981723981723913)));
    assert!(x == ring.from(ubig!(_18297381723918723981723981723906)));
    assert!(x == ring.from(ibig!(_18297381723918723981723981723906)));
    assert!(x == ring.from(ibig!(-_18297381723918723981723981723994)));
    assert!(x == ring.from(&ibig!(-_18297381723918723981723981723994)));
    assert!(x == ring.from(106u8));
    assert!(x == ring.from(106u16));
    assert!(x == ring.from(1006u32));
    assert!(x == ring.from(10000000006u64));
    assert!(x == ring.from(1000000000000000000006u128));
    assert!(x == ring.from(106usize));
    assert!(x == ring.from(6i8));
    assert!(x == ring.from(-94i8));
    assert!(x == ring.from(-94i16));
    assert!(x == ring.from(-94i32));
    assert!(x == ring.from(-94i64));
    assert!(x == ring.from(-94i128));
    assert!(x == ring.from(-94isize));

    assert!(ring.from(0) == ring.from(false));
    assert!(ring.from(1) == ring.from(true));
}

#[test]
fn test_negate() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(-1234);
    let y = -&x;
    assert_eq!(y.residue(), ubig!(34));
    let y = -x;
    assert_eq!(y.residue(), ubig!(34));

    let ring = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring.from(ibig!(-_33333123456789012345678901234567890));
    let y = -&x;
    assert!(y == ring.from(ubig!(_44444123456789012345678901234567890)));
    assert_eq!(y.residue(), ubig!(_123456789012345678901234567890));
    let y = -x;
    assert!(y == ring.from(ubig!(_44444123456789012345678901234567890)));
}

#[test]
fn test_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(100));
    assert!(ring1 == ring1);
    assert!(ring1 != ring2);
}

#[test]
#[should_panic]
fn test_cmp_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(200));
    let x = ring1.from(5);
    let y = ring2.from(5);
    let _ = x == y;
}

#[test]
fn test_add_sub() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let test_cases = [
        (ring1.from(1), ring1.from(2), ring1.from(3)),
        (ring1.from(99), ring1.from(5), ring1.from(4)),
        (ring1.from(99), ring1.from(99), ring1.from(98)),
        (
            ring2.from(ubig!(111111111111111111111111111111)),
            ring2.from(ubig!(222222222222222223333333333333)),
            ring2.from(ubig!(333333333333333334444444444444)),
        ),
        (
            ring2.from(ubig!(111111111111111111111111111111)),
            ring2.from(ubig!(888888888888888888888888888889)),
            ring2.from(ubig!(0)),
        ),
        (
            ring2.from(ubig!(999999999999999999999999999999)),
            ring2.from(ubig!(999999999999999999999999999997)),
            ring2.from(ubig!(999999999999999999999999999996)),
        ),
    ];

    let all_test_cases = test_cases
        .iter()
        .map(|(a, b, c)| (a, b, c))
        .chain(test_cases.iter().map(|(a, b, c)| (b, a, c)));

    for (a, b, c) in all_test_cases {
        assert!(a + b == *c);
        assert!(a.clone() + b == *c);
        assert!(a + b.clone() == *c);
        assert!(a.clone() + b.clone() == *c);
        let mut x = a.clone();
        x += b;
        assert!(x == *c);
        let mut x = a.clone();
        x += b.clone();
        assert!(x == *c);

        assert!(c - a == *b);
        assert!(c.clone() - a == *b);
        assert!(c - a.clone() == *b);
        assert!(c.clone() - a.clone() == *b);
        let mut x = c.clone();
        x -= a;
        assert!(x == *b);
        let mut x = c.clone();
        x -= a.clone();
        assert!(x == *b);
    }
}

#[test]
#[should_panic]
fn test_add_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(200));
    let x = ring1.from(5);
    let y = ring2.from(5);
    let _ = x + y;
}

#[test]
#[should_panic]
fn test_sub_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(200));
    let x = ring1.from(5);
    let y = ring2.from(5);
    let _ = x - y;
}
