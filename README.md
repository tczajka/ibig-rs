Big integer library.

The library implements arbitrarily large integer arithmetic in pure Rust.

The two integer types are `UBig` (for unsigned integers) and `IBig` (for signed integers).

Create numbers using the `ubig` and `ibig` macros
```rust
use ibig::prelude::*;
let a = ubig!(12345678);
let b = ubig!(0x10ff);
let c = ibig!(-azz base 36);
assert_eq!(c.to_string(), "-14255");
```

Parsing and formatting in any base 2-36 is supported.
```rust
let a: UBig = "1503321".parse()?;
let b = IBig::from_str_radix("-10ff", 16)?;
assert_eq!(format!("{:^10X}", b), "  -10FF   ");
assert_eq!(format!("hello {}", b.in_radix(4)), "hello -1003333");
```

Standard arithmetic operations are supported on values and on references.
```rust
assert_eq!(ubig!(100) * ubig!(200), ubig!(20000));
let a = ubig!(0xff);
assert_eq!(&a / &a, ubig!(1));
assert_eq!(ibig!(1) << 1000 >> 999, ibig!(2));
```
