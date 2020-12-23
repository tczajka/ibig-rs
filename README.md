Big integer library.

The library implements arbitrarily large integer arithmetic in pure Rust.

The two integer types are `UBig` (for unsigned integers) and `IBig` (for signed integers).

Create numbers using the `ubig` and `ibig` macros
```rust
use ibig::prelude::*;
let a = ubig!(0x10ff);
let b = ibig!(-abcd base 32);
```

Parsing and formatting in any base 2-36 is supported.
```rust
let a = UBig::from_str_radix("10ff", 16)?;
assert_eq!(format!("{:^10X}", a), "   10FF   ");
assert_eq!(format!("{}", a.in_radix(4)), "1003333");
```

Arithmetic operations will soon arrive.
