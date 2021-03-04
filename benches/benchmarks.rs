#![feature(test)]

extern crate test;

use ibig::prelude::*;
use rand::prelude::*;
use test::{bench::Bencher, black_box};

/*
const MILLION_DECIMAL: usize = 3321928;
*/

fn random_ubig<R>(bits: usize, rng: &mut R) -> UBig
where
    R: Rng + ?Sized,
{
    rng.gen_range(ubig!(1) << (bits - 1)..ubig!(1) << bits)
}

fn bench_add(bits: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits, &mut rng);
    let b = random_ubig(bits, &mut rng);
    bencher.iter(|| black_box(&a) + black_box(&b));
}

#[bench]
fn bench_add_1e1(bencher: &mut Bencher) {
    bench_add(10, bencher);
}

#[bench]
fn bench_add_1e2(bencher: &mut Bencher) {
    bench_add(100, bencher);
}

#[bench]
fn bench_add_1e3(bencher: &mut Bencher) {
    bench_add(1_000, bencher);
}

#[bench]
fn bench_add_1e4(bencher: &mut Bencher) {
    bench_add(10_000, bencher);
}

#[bench]
fn bench_add_1e5(bencher: &mut Bencher) {
    bench_add(100_000, bencher);
}

#[bench]
fn bench_add_1e6(bencher: &mut Bencher) {
    bench_add(1_000_000, bencher);
}

fn bench_sub(bits: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits + 1, &mut rng);
    let b = random_ubig(bits, &mut rng);
    bencher.iter(|| black_box(&a) - black_box(&b));
}

#[bench]
fn bench_sub_1e1(bencher: &mut Bencher) {
    bench_sub(10, bencher);
}

#[bench]
fn bench_sub_1e2(bencher: &mut Bencher) {
    bench_sub(100, bencher);
}

#[bench]
fn bench_sub_1e3(bencher: &mut Bencher) {
    bench_sub(1_000, bencher);
}

#[bench]
fn bench_sub_1e4(bencher: &mut Bencher) {
    bench_sub(10_000, bencher);
}

#[bench]
fn bench_sub_1e5(bencher: &mut Bencher) {
    bench_sub(100_000, bencher);
}

#[bench]
fn bench_sub_1e6(bencher: &mut Bencher) {
    bench_sub(1_000_000, bencher);
}

fn bench_mul(bits_a: usize, bits_b: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits_a, &mut rng);
    let b = random_ubig(bits_b, &mut rng);
    bencher.iter(|| black_box(&a) * black_box(&b));
}

#[bench]
fn bench_mul_same_1e1(bencher: &mut Bencher) {
    bench_mul(10, 10, bencher);
}

#[bench]
fn bench_mul_same_1e2(bencher: &mut Bencher) {
    bench_mul(100, 100, bencher);
}

#[bench]
fn bench_mul_same_1e3(bencher: &mut Bencher) {
    bench_mul(1_000, 1_000, bencher);
}

#[bench]
fn bench_mul_same_1e4(bencher: &mut Bencher) {
    bench_mul(10_000, 10_000, bencher);
}

#[bench]
fn bench_mul_same_1e5(bencher: &mut Bencher) {
    bench_mul(100_000, 100_000, bencher);
}

#[bench]
fn bench_mul_same_1e6(bencher: &mut Bencher) {
    bench_mul(1_000_000, 1_000_000, bencher);
}

/*
#[bench]
fn bench_mul_million_decimal(bencher: &mut Bencher) {
    bench_mul(MILLION_DECIMAL, MILLION_DECIMAL, bencher);
}
*/

#[bench]
fn bench_mul_1e1_1e2(bencher: &mut Bencher) {
    bench_mul(10, 100, bencher);
}

#[bench]
fn bench_mul_1e1_1e3(bencher: &mut Bencher) {
    bench_mul(10, 1_000, bencher);
}

#[bench]
fn bench_mul_1e1_1e4(bencher: &mut Bencher) {
    bench_mul(10, 10_000, bencher);
}

#[bench]
fn bench_mul_1e1_1e5(bencher: &mut Bencher) {
    bench_mul(10, 100_000, bencher);
}

#[bench]
fn bench_mul_1e1_1e6(bencher: &mut Bencher) {
    bench_mul(10, 1_000_000, bencher);
}

#[bench]
fn bench_mul_1e1_1e7(bencher: &mut Bencher) {
    bench_mul(10, 10_000_000, bencher);
}

#[bench]
fn bench_mul_1e1_1e8(bencher: &mut Bencher) {
    bench_mul(10, 100_000_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e3(bencher: &mut Bencher) {
    bench_mul(100, 1_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e4(bencher: &mut Bencher) {
    bench_mul(100, 10_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e5(bencher: &mut Bencher) {
    bench_mul(100, 100_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e6(bencher: &mut Bencher) {
    bench_mul(100, 1_000_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e7(bencher: &mut Bencher) {
    bench_mul(100, 10_000_000, bencher);
}

#[bench]
fn bench_mul_1e2_1e8(bencher: &mut Bencher) {
    bench_mul(100, 100_000_000, bencher);
}

#[bench]
fn bench_mul_1e3_1e4(bencher: &mut Bencher) {
    bench_mul(1_000, 10_000, bencher);
}

#[bench]
fn bench_mul_1e3_1e5(bencher: &mut Bencher) {
    bench_mul(1_000, 100_000, bencher);
}

#[bench]
fn bench_mul_1e3_1e6(bencher: &mut Bencher) {
    bench_mul(1_000, 1_000_000, bencher);
}

#[bench]
fn bench_mul_1e3_1e7(bencher: &mut Bencher) {
    bench_mul(1_000, 10_000_000, bencher);
}

#[bench]
fn bench_mul_1e3_1e8(bencher: &mut Bencher) {
    bench_mul(1_000, 100_000_000, bencher);
}

#[bench]
fn bench_mul_1e4_1e5(bencher: &mut Bencher) {
    bench_mul(10_000, 100_000, bencher);
}

#[bench]
fn bench_mul_1e4_1e6(bencher: &mut Bencher) {
    bench_mul(10_000, 1_000_000, bencher);
}

#[bench]
fn bench_mul_1e5_1e6(bencher: &mut Bencher) {
    bench_mul(100_000, 1_000_000, bencher);
}

fn bench_div(bits_q: usize, bits_r: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits_q + bits_r, &mut rng);
    let b = random_ubig(bits_r, &mut rng);
    bencher.iter(|| black_box(&a).div_rem(black_box(&b)));
}

#[bench]
fn bench_div_same_1e1(bencher: &mut Bencher) {
    bench_div(10, 10, bencher);
}

#[bench]
fn bench_div_same_1e2(bencher: &mut Bencher) {
    bench_div(100, 100, bencher);
}

#[bench]
fn bench_div_same_1e3(bencher: &mut Bencher) {
    bench_div(1_000, 1_000, bencher);
}

#[bench]
fn bench_div_same_1e4(bencher: &mut Bencher) {
    bench_div(10_000, 10_000, bencher);
}

#[bench]
fn bench_div_same_1e5(bencher: &mut Bencher) {
    bench_div(100_000, 100_000, bencher);
}

/*
#[bench]
fn bench_div_same_1e6(bencher: &mut Bencher) {
    bench_div(1_000_000, 1_000_000, bencher);
}
*/

#[bench]
fn bench_div_1e1_1e2(bencher: &mut Bencher) {
    bench_div(10, 100, bencher);
}

#[bench]
fn bench_div_1e1_1e3(bencher: &mut Bencher) {
    bench_div(10, 1000, bencher);
}

#[bench]
fn bench_div_1e1_1e4(bencher: &mut Bencher) {
    bench_div(10, 10_000, bencher);
}

#[bench]
fn bench_div_1e1_1e5(bencher: &mut Bencher) {
    bench_div(10, 100_000, bencher);
}

#[bench]
fn bench_div_1e1_1e6(bencher: &mut Bencher) {
    bench_div(10, 1_000_000, bencher);
}

#[bench]
fn bench_div_1e2_1e1(bencher: &mut Bencher) {
    bench_div(100, 30, bencher);
}

#[bench]
fn bench_div_1e2_1e3(bencher: &mut Bencher) {
    bench_div(100, 1000, bencher);
}

#[bench]
fn bench_div_1e2_1e4(bencher: &mut Bencher) {
    bench_div(100, 10_000, bencher);
}

#[bench]
fn bench_div_1e2_1e5(bencher: &mut Bencher) {
    bench_div(100, 100_000, bencher);
}

#[bench]
fn bench_div_1e2_1e6(bencher: &mut Bencher) {
    bench_div(100, 1_000_000, bencher);
}

#[bench]
fn bench_div_1e3_1e1(bencher: &mut Bencher) {
    bench_div(1_000, 30, bencher);
}

#[bench]
fn bench_div_1e3_1e2(bencher: &mut Bencher) {
    bench_div(1_000, 100, bencher);
}

#[bench]
fn bench_div_1e3_1e4(bencher: &mut Bencher) {
    bench_div(1_000, 10_000, bencher);
}

#[bench]
fn bench_div_1e3_1e5(bencher: &mut Bencher) {
    bench_div(1_000, 100_000, bencher);
}

#[bench]
fn bench_div_1e3_1e6(bencher: &mut Bencher) {
    bench_div(1_000, 1_000_000, bencher);
}

#[bench]
fn bench_div_1e4_1e1(bencher: &mut Bencher) {
    bench_div(10_000, 30, bencher);
}

#[bench]
fn bench_div_1e4_1e2(bencher: &mut Bencher) {
    bench_div(10_000, 100, bencher);
}

#[bench]
fn bench_div_1e4_1e3(bencher: &mut Bencher) {
    bench_div(10_000, 1_000, bencher);
}

#[bench]
fn bench_div_1e4_1e5(bencher: &mut Bencher) {
    bench_div(10_000, 100_000, bencher);
}

#[bench]
fn bench_div_1e4_1e6(bencher: &mut Bencher) {
    bench_div(10_000, 1_000_000, bencher);
}

#[bench]
fn bench_div_1e5_1e1(bencher: &mut Bencher) {
    bench_div(100_000, 30, bencher);
}

#[bench]
fn bench_div_1e5_1e2(bencher: &mut Bencher) {
    bench_div(100_000, 100, bencher);
}

#[bench]
fn bench_div_1e5_1e3(bencher: &mut Bencher) {
    bench_div(100_000, 1_000, bencher);
}

#[bench]
fn bench_div_1e5_1e4(bencher: &mut Bencher) {
    bench_div(100_000, 10_000, bencher);
}

#[bench]
fn bench_div_1e5_1e6(bencher: &mut Bencher) {
    bench_div(100_000, 1_000_000, bencher);
}

#[bench]
fn bench_div_1e6_1e1(bencher: &mut Bencher) {
    bench_div(1_000_000, 30, bencher);
}

#[bench]
fn bench_div_1e6_1e2(bencher: &mut Bencher) {
    bench_div(1_000_000, 100, bencher);
}

#[bench]
fn bench_div_1e6_1e3(bencher: &mut Bencher) {
    bench_div(1_000_000, 1_000, bencher);
}

#[bench]
fn bench_div_1e6_1e4(bencher: &mut Bencher) {
    bench_div(1_000_000, 10_000, bencher);
}

#[bench]
fn bench_div_1e6_1e5(bencher: &mut Bencher) {
    bench_div(1_000_000, 100_000, bencher);
}

/*
#[bench]
fn bench_div_million_decimal(bencher: &mut Bencher) {
    bench_div(MILLION_DECIMAL, MILLION_DECIMAL, bencher);
}
*/

#[bench]
fn bench_rem_1e5_1e1(bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(100_000 + 30, &mut rng);
    let b = random_ubig(30, &mut rng);
    bencher.iter(|| black_box(&a) % black_box(&b));
}

fn bench_to_str_radix(bits: usize, radix: u32, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits, &mut rng);
    bencher.iter(|| black_box(&a).to_str_radix(radix));
}

#[bench]
fn bench_to_hex_10(bencher: &mut Bencher) {
    bench_to_str_radix(10, 16, bencher);
}

#[bench]
fn bench_to_hex_100(bencher: &mut Bencher) {
    bench_to_str_radix(100, 16, bencher);
}

#[bench]
fn bench_to_hex_1000(bencher: &mut Bencher) {
    bench_to_str_radix(1000, 16, bencher);
}

#[bench]
fn bench_to_hex_10000(bencher: &mut Bencher) {
    bench_to_str_radix(10000, 16, bencher);
}

#[bench]
fn bench_to_hex_100000(bencher: &mut Bencher) {
    bench_to_str_radix(100000, 16, bencher);
}

#[bench]
fn bench_to_hex_1000000(bencher: &mut Bencher) {
    bench_to_str_radix(1000000, 16, bencher);
}

#[bench]
fn bench_to_dec_10(bencher: &mut Bencher) {
    bench_to_str_radix(10, 10, bencher);
}

#[bench]
fn bench_to_dec_100(bencher: &mut Bencher) {
    bench_to_str_radix(100, 10, bencher);
}

#[bench]
fn bench_to_dec_1000(bencher: &mut Bencher) {
    bench_to_str_radix(1000, 10, bencher);
}

#[bench]
fn bench_to_dec_10000(bencher: &mut Bencher) {
    bench_to_str_radix(10000, 10, bencher);
}

#[bench]
fn bench_to_dec_100000(bencher: &mut Bencher) {
    bench_to_str_radix(100000, 10, bencher);
}

fn bench_from_str_radix(bits: usize, radix: u32, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits, &mut rng);
    let s = a.to_str_radix(radix);
    bencher.iter(|| UBig::from_str_radix(black_box(&s), radix));
}

#[bench]
fn bench_from_hex_10(bencher: &mut Bencher) {
    bench_from_str_radix(10, 16, bencher);
}

#[bench]
fn bench_from_hex_100(bencher: &mut Bencher) {
    bench_from_str_radix(100, 16, bencher);
}

#[bench]
fn bench_from_hex_1000(bencher: &mut Bencher) {
    bench_from_str_radix(1000, 16, bencher);
}

#[bench]
fn bench_from_hex_10000(bencher: &mut Bencher) {
    bench_from_str_radix(10000, 16, bencher);
}

#[bench]
fn bench_from_hex_100000(bencher: &mut Bencher) {
    bench_from_str_radix(100000, 16, bencher);
}

#[bench]
fn bench_from_hex_1000000(bencher: &mut Bencher) {
    bench_from_str_radix(1000000, 16, bencher);
}

#[bench]
fn bench_from_dec_10(bencher: &mut Bencher) {
    bench_from_str_radix(10, 10, bencher);
}

#[bench]
fn bench_from_dec_100(bencher: &mut Bencher) {
    bench_from_str_radix(100, 10, bencher);
}

#[bench]
fn bench_from_dec_1000(bencher: &mut Bencher) {
    bench_from_str_radix(1000, 10, bencher);
}

#[bench]
fn bench_from_dec_10000(bencher: &mut Bencher) {
    bench_from_str_radix(10000, 10, bencher);
}

#[bench]
fn bench_from_dec_100000(bencher: &mut Bencher) {
    bench_from_str_radix(100000, 10, bencher);
}

fn bench_pow(a: UBig, b: usize, bencher: &mut Bencher) {
    bencher.iter(|| black_box(&a).pow(black_box(b)));
}

#[bench]
fn bench_pow_3_10(bencher: &mut Bencher) {
    bench_pow(ubig!(3), 10, bencher);
}

#[bench]
fn bench_pow_3_100(bencher: &mut Bencher) {
    bench_pow(ubig!(3), 100, bencher);
}

#[bench]
fn bench_pow_3_1000(bencher: &mut Bencher) {
    bench_pow(ubig!(3), 1000, bencher);
}

#[bench]
fn bench_pow_3_10000(bencher: &mut Bencher) {
    bench_pow(ubig!(3), 10000, bencher);
}

#[bench]
fn bench_pow_3_100000(bencher: &mut Bencher) {
    bench_pow(ubig!(3), 100000, bencher);
}
