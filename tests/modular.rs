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
    let x = ring1.from(ubig!(512));
    let y = x.clone();
    assert!(x == y);
    let mut z = ring1.from(ubig!(513));
    assert!(x != z);
    z.clone_from(&x);
    assert!(x == z);

    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring2.from(ubig!(512));
    let y = x.clone();
    assert!(x == y);
    let mut z = ring2.from(ubig!(513));
    assert!(x != z);
    z.clone_from(&x);
    assert!(x == z);

    let mut x = ring1.from(ubig!(512));
    let y = ring2.from(ubig!(1));
    x.clone_from(&y);
    assert!(x == y);

    let ring3 = ModuloRing::new(&ubig!(10).pow(100));
    let x = ring2.from(ubig!(1));
    let mut y = ring3.from(ubig!(2));
    y.clone_from(&x);
    assert!(x == y);
}

#[test]
fn test_convert() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(ubig!(512));
    assert!(x == ring.from(&ubig!(312)));
    assert!(x != ring.from(&ubig!(313)));
    assert!(x == ring.from(&ubig!(_18297381723918723981723981723912)));
    assert!(x != ring.from(&ubig!(_18297381723918723981723981723913)));
    assert!(x == ring.from(ubig!(_18297381723918723981723981723912)));
    assert!(x == ring.from(ibig!(_18297381723918723981723981723912)));
    assert!(x == ring.from(ibig!(-_18297381723918723981723981723988)));
    assert!(x == ring.from(&ibig!(-_18297381723918723981723981723988)));
}

#[test]
fn test_negate() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(ibig!(-1234));
    let y = -x;
    assert_eq!(y.residue(), ubig!(34));

    let ring = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring.from(ibig!(-_33333123456789012345678901234567890));
    let y = -x;
    assert!(y == ring.from(ubig!(_44444123456789012345678901234567890)));
    assert_eq!(y.residue(), ubig!(_123456789012345678901234567890));
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
    let x = ring1.from(ubig!(5));
    let y = ring2.from(ubig!(5));
    let _ = x == y;
}
