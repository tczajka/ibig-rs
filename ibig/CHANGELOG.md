# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Work in progress. This release is a ground-up rewrite of the library, with
substantial internal changes to representation and algorithms. Expect breaking
API changes.

## [0.3.6] - 2022-09-18

### Added

- GCD, greatest common divisor.
- Extended GCD (GCD with Bézout coefficients).
- Modular inverse.
- Modular division.

## [0.3.5] - 2022-03-12

### Changed

- Removed the dependency on `const_fn_assert`.

### Fixed

- Corrected too-strict lifetimes in modular exponentiation.

## [0.3.4] - 2021-11-03

### Added

- Optional `serde` support for `UBig` and `IBig`.

### Changed

- Rust 1.49+ is now required.
- Added an optional dependency on `serde`.

## [0.3.3] - 2021-10-28

### Added

- Mixed-type arithmetic with primitive integer types. Allows `x + 1` instead of `x + ubig!(1)`.

## [0.3.2] - 2021-05-02

### Changed

- Rust 1.47+ is now supported.
- Added a dependency on `cfg-if`.

## [0.3.1] - 2021-04-03

### Added

- Maximum supported length in bits: `UBig::MAX_BIT_LEN`.

### Changed

- Added a dependency on `const_fn_assert`.

### Fixed

- Broken build for `aarch64`, `mips64` and `powerpc64` fixed.

## [0.3.0] - 2021-03-29

### Changed

- Split into modules:
  - Moved `InRadix` to `fmt`.
  - Moved operator traits to `ops`.
  - Moved errors to `error`.
  - Moved distributions to `rand`.
- Shift left and right now only accepts `usize` for the number of bits, for consistency with other bit addressing operations and exponents.

### Removed

- Removed `prelude`.
- Removed deprecated `IBig::is_positive`, `IBig::is_negative`. Just compare with `ibig!(0)` instead.

## [0.2.2] - 2021-03-28

### Added

- Modular arithmetic: `ModuloRing`, `Modulo`.
- Conversions to floating point: `to_f32`, `to_f64`. Rounds to nearest, breaking ties to even.
- `From<bool>` for `IBig`.

## [0.2.1] - 2021-03-14

### Added

- Implemented num-traits traits.

### Changed

- Loosened the license to either MIT or Apache-2.0.
- Added an optional dependency on `num-traits 0.2.14`.
- Removed the dependency on `ascii`.

### Deprecated

- `IBig::is_positive`, `IBig::is_negative`. Just use `> ibig!(0)`, `< ibig!(0)`.

## [0.2.0] - 2021-03-11

### Removed

- Removed deprecated functions `to_str_radix`, `to_str_radix_uppercase`, `ilog2`.

## [0.1.2] - 2021-03-09

### Added

- `bit_len`.

### Changed

- Added a dependency on `static_assertions 1.1`.
- Bumped `rand` to `0.8.3`.
- Large division improved. Now uses a divide and conquer algorithm, O(n^1.47).
- Large `parse` improved using a divide and conquer algorithm, O(n^1.47).
- Large `to_string` improved using a divide and conquer algorithm, O(n^1.47).
- Other minor performance improvements.

### Deprecated

- `to_str_radix`, `to_str_radix_uppercase`. Use `in_radix(...)` instead.
- `ilog2`. Use `bit_len` instead.

## [0.1.1] - 2021-03-03

### Added

- Hashing.
- Exponentiation.
- Random sampling (optional dependency on `rand 0.8`).
- `factorial` example prints 1000000! in hexadecimal.

### Changed

- Multiplication improved, now uses Karatsuba and Toom-Cook-3 algorithms, O(n^1.47).

## [0.1.0] - 2021-02-25

The initial usable version. Operations on very large numbers are still slow.

### Added

- All basic arithmetic and bitwise operations.
- Parsing and formatting.
- Constructor macros.

[Unreleased]: https://github.com/tczajka/ibig/compare/v0.3.6...HEAD
[0.3.6]: https://github.com/tczajka/ibig/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/tczajka/ibig/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/tczajka/ibig/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/tczajka/ibig/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/tczajka/ibig/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/tczajka/ibig/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/tczajka/ibig/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/tczajka/ibig/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/tczajka/ibig/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/tczajka/ibig/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/tczajka/ibig/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/tczajka/ibig/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/tczajka/ibig/releases/tag/v0.1.0
