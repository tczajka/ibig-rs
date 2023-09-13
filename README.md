# ibig

[![crate](https://img.shields.io/crates/v/ibig.svg)](https://crates.io/crates/ibig)
[![docs](https://docs.rs/ibig/badge.svg)](https://docs.rs/ibig)
![rustc 1.61+](https://img.shields.io/badge/rustc-1.61%2B-informational.svg)
[![tests](https://github.com/tczajka/ibig-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/tczajka/ibig-rs/actions/workflows/tests.yml)

A big integer library with good performance.

The library implements efficient large integer arithmetic in pure Rust.

The two integer types are `UBig` (for unsigned integers) and `IBig` (for signed integers).

Modular arithmetic is implemented by the `Modulo` type.

## Examples

```rust
use ibig::{ibig, ubig, ModuloRing, UBig};

let a = ubig!(12345678);
let b = ubig!(0x10ff);
let c = ibig!(-azz base 36);
let d: UBig = "15033211231241234523452345345787".parse()?;
let e = 2 * &b + 1;
let f = a * b.pow(10);

assert_eq!(e, ubig!(0x21ff));
assert_eq!(c.to_string(), "-14255");
assert_eq!(
    f.in_radix(16).to_string(),
    "1589bda8effbfc495d8d73c83d8b27f94954e"
);
assert_eq!(
    format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
    "hello 0x1a7e7c487267d2658a93"
);

let ring = ModuloRing::new(&ubig!(10000));
let x = ring.from(12345);
let y = ring.from(55443);
assert_eq!(format!("{}", x - y), "6902 (mod 10000)");
```

## Optional dependencies

* `std` (default): for `std::error::Error`.
* `rand`: random number generation.
* `serde`: serialization and deserialization.

## Benchmarks

[Benchmarks](https://github.com/tczajka/bigint-benchmark-rs) contains a quick benchmark of
Rust big integer libraries.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
