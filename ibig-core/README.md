# ibig-core

[![crates.io](https://img.shields.io/crates/v/ibig-core.svg)](https://crates.io/crates/ibig-core)
[![docs.rs](https://docs.rs/ibig-core/badge.svg)](https://docs.rs/ibig-core)

Core big-integer algorithms.

This crate provides the fundamental arithmetic algorithms — addition, subtraction,
multiplication and division — operating on sequences of `Digit`s, where a `Digit` is the
platform's native unsigned integer type ([`UNative`](https://crates.io/crates/unative)).

It is the low-level foundation of the [`ibig`](https://crates.io/crates/ibig) big-integer
library and is in early development.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
