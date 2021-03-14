# ibig

[![crate](https://img.shields.io/crates/v/ibig.svg)](https://crates.io/crates/ibig)
[![tests](https://github.com/tczajka/ibig-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/tczajka/ibig-rs/actions/workflows/tests.yml)

A big integer library with good performance.

The library implements efficient large integer arithmetic in pure Rust.

The two integer types are `UBig` (for unsigned integers) and `IBig` (for signed integers).

## Example

```rust
use ibig::prelude::*;

let a = ubig!(12345678);
let b = ubig!(0x10ff);
let c = ibig!(-azz base 36);
let d: UBig = "15033211231241234523452345345787".parse()?;

assert_eq!(c.to_string(), "-14255");
assert_eq!(
    (a * b.pow(10)).in_radix(16).to_string(),
    "1589bda8effbfc495d8d73c83d8b27f94954e"
);
assert_eq!(
    format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
    "hello 0x1a7e7c487267d2658a93"
);
```

## Benchmarks

[Benchmarks](https://github.com/tczajka/bigint-benchmark-rs) contains a quick benchmark of
Rust big integer libraries.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
