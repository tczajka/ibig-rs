use ibig::prelude::*;
use rand::{distributions::uniform::Uniform, prelude::*};

#[test]
fn test_uniform_ubig() {
    let mut rng = StdRng::seed_from_u64(1);

    let distr = Uniform::from(ubig!(3)..ubig!(7));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ubig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ubig!(6));

    let distr = Uniform::from(ubig!(3)..=ubig!(7));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ubig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ubig!(7));

    let distr = Uniform::from(ubig!(0b100) << 128..ubig!(0b1000) << 128);
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert!(x >= ubig!(0b100) << 128 && x < ubig!(0b101) << 128);
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert!(x >= ubig!(0b111) << 128 && x < ubig!(0b1000) << 128);
}

#[test]
fn test_uniform_ibig() {
    let mut rng = StdRng::seed_from_u64(1);

    let distr = Uniform::from(ibig!(-7)..ibig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ibig!(-7));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ibig!(2));

    let distr = Uniform::from(ibig!(-7)..=ibig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ibig!(-7));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ibig!(3));
}

#[test]
fn test_random_arithmetic() {
    let mut rng = StdRng::seed_from_u64(3);
    let p = ubig!(1000000007);

    for iter in 0..100 {
        let big = iter > 80;
        let range = if big { 1000000 } else { 100000 };
        let len_a: u32 = (&mut rng).gen_range(1000..range);
        let len_b: u32 = (&mut rng).gen_range(1000..range);
        let a = (&mut rng).gen_range(ubig!(100)..ubig!(1) << len_a);
        let b = (&mut rng).gen_range(ubig!(100)..ubig!(1) << len_b);
        let c = (&mut rng).sample(Uniform::new(ubig!(0), &a));
        let radix = (&mut rng).gen_range(2..=36);

        assert_eq!((&a + &b) % &p, ((&a % &p) + (&b % &p)) % &p);
        assert_eq!(&a + &b - &a, b);
        assert_eq!((&a * &b) % &p, ((&a % &p) * (&b % &p)) % &p);
        let (q, r) = (&a * &b + &c).div_rem(&a);
        assert_eq!(q, b);
        assert_eq!(r, c);
        assert_eq!(
            UBig::from_str_radix(&a.in_radix(radix).to_string(), radix).unwrap(),
            a
        )
    }
}
