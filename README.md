# ibig

[![crate](https://img.shields.io/crates/v/ibig.svg)](https://crates.io/crates/ibig)
[![tests](https://github.com/tczajka/ibig-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/tczajka/ibig-rs/actions)

Big integer library.

The library implements arbitrarily large integer arithmetic in pure Rust.

The two integer types are `UBig` (for unsigned integers) and `IBig` (for signed integers).

```rust
use ibig::prelude::*;

let a = ubig!(12345678);
let b = ubig!(0x10ff);
let c = ibig!(-azz base 36);
let d: UBig = "15033211231241234523452345345787".parse()?;

assert_eq!(c.to_string(), "-14255");
assert_eq!(
    (a * b.pow(10)).to_str_radix(16),
    "1589bda8effbfc495d8d73c83d8b27f94954e"
);
assert_eq!(
    format!("hello {:#x}", d % ubig!(0xabcd1234134132451345)),
    "hello 0x1a7e7c487267d2658a93"
);
```
