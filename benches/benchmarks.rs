#![feature(test)]

extern crate test;

use ibig::prelude::*;
use rand::prelude::*;
use test::{bench::Bencher, black_box};

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
fn bench_add_10(bencher: &mut Bencher) {
    bench_add(10, bencher);
}

#[bench]
fn bench_add_100(bencher: &mut Bencher) {
    bench_add(100, bencher);
}

#[bench]
fn bench_add_1000(bencher: &mut Bencher) {
    bench_add(1000, bencher);
}

#[bench]
fn bench_add_10000(bencher: &mut Bencher) {
    bench_add(10000, bencher);
}

#[bench]
fn bench_add_100000(bencher: &mut Bencher) {
    bench_add(100000, bencher);
}

#[bench]
fn bench_add_1000000(bencher: &mut Bencher) {
    bench_add(1000000, bencher);
}

fn bench_sub(bits: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits + 1, &mut rng);
    let b = random_ubig(bits, &mut rng);
    bencher.iter(|| black_box(&a) - black_box(&b));
}

#[bench]
fn bench_sub_10(bencher: &mut Bencher) {
    bench_sub(10, bencher);
}

#[bench]
fn bench_sub_100(bencher: &mut Bencher) {
    bench_sub(100, bencher);
}

#[bench]
fn bench_sub_1000(bencher: &mut Bencher) {
    bench_sub(1000, bencher);
}

#[bench]
fn bench_sub_10000(bencher: &mut Bencher) {
    bench_sub(10000, bencher);
}

#[bench]
fn bench_sub_100000(bencher: &mut Bencher) {
    bench_sub(100000, bencher);
}

#[bench]
fn bench_sub_1000000(bencher: &mut Bencher) {
    bench_sub(1000000, bencher);
}

fn bench_mul(bits_a: usize, bits_b: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits_a, &mut rng);
    let b = random_ubig(bits_b, &mut rng);
    bencher.iter(|| black_box(&a) * black_box(&b));
}

#[bench]
fn bench_mul_10(bencher: &mut Bencher) {
    bench_mul(10, 10, bencher);
}

#[bench]
fn bench_mul_100(bencher: &mut Bencher) {
    bench_mul(100, 100, bencher);
}

#[bench]
fn bench_mul_1000(bencher: &mut Bencher) {
    bench_mul(1000, 1000, bencher);
}

#[bench]
fn bench_mul_10000(bencher: &mut Bencher) {
    bench_mul(10000, 10000, bencher);
}

#[bench]
fn bench_mul_100000(bencher: &mut Bencher) {
    bench_mul(100000, 100000, bencher);
}

#[bench]
fn bench_mul_small_100(bencher: &mut Bencher) {
    bench_mul(100, 10, bencher);
}

#[bench]
fn bench_mul_small_1000(bencher: &mut Bencher) {
    bench_mul(1000, 10, bencher);
}

#[bench]
fn bench_mul_small_10000(bencher: &mut Bencher) {
    bench_mul(10000, 10, bencher);
}

#[bench]
fn bench_mul_small_100000(bencher: &mut Bencher) {
    bench_mul(100000, 10, bencher);
}

#[bench]
fn bench_mul_small_1000000(bencher: &mut Bencher) {
    bench_mul(1000000, 10, bencher);
}

fn bench_div(bits_a: usize, bits_b: usize, bencher: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1);
    let a = random_ubig(bits_a, &mut rng);
    let b = random_ubig(bits_b, &mut rng);
    bencher.iter(|| black_box(&a).div_rem(black_box(&b)));
}

#[bench]
fn bench_div_10(bencher: &mut Bencher) {
    bench_div(20, 10, bencher);
}

#[bench]
fn bench_div_100(bencher: &mut Bencher) {
    bench_div(200, 100, bencher);
}

#[bench]
fn bench_div_1000(bencher: &mut Bencher) {
    bench_div(2000, 1000, bencher);
}

#[bench]
fn bench_div_10000(bencher: &mut Bencher) {
    bench_div(20000, 10000, bencher);
}

#[bench]
fn bench_div_100000(bencher: &mut Bencher) {
    bench_div(200000, 100000, bencher);
}

#[bench]
fn bench_div_small_100(bencher: &mut Bencher) {
    bench_div(100, 10, bencher);
}

#[bench]
fn bench_div_small_1000(bencher: &mut Bencher) {
    bench_div(1000, 10, bencher);
}

#[bench]
fn bench_div_small_10000(bencher: &mut Bencher) {
    bench_div(10000, 10, bencher);
}

#[bench]
fn bench_div_small_100000(bencher: &mut Bencher) {
    bench_div(100000, 10, bencher);
}

#[bench]
fn bench_div_small_1000000(bencher: &mut Bencher) {
    bench_div(1000000, 10, bencher);
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
