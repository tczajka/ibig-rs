# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Features
* `From<bool>` for `IBig`.

## 0.2.1 - 2021-03-14

### License

* Loosened the license to either MIT or Apache-2.0.

### Features
* Implemented num-traits traits.

### Deprecated features
* `IBig::is_positive`, `IBig::is_negative`. Just use `> ibig!(0)`, `< ibig!(0)`.

### Dependencies
* Added optional dependency on `num-traits 0.2.14`.
* Removed dependency on `ascii`.

## 0.2.0 - 2021-03-11

### Removed features
* Removed deprecated functions `to_str_radix`, `to_str_radix_uppercase`, `ilog2`.

## 0.1.2 - 2021-03-09

### New features
* `bit_len`

### Deprecated features
* `to_str_radix`, `to_str_radix_uppercase`. Use `in_radix(...)` instead.
* `ilog2`. Use `bit_len` instead.

### Dependencies
* Added a dependency on `static_assertions 1.1`.
* Bumped `rand` to `0.8.3`.

### Performance
* Large division improved. Now uses a divide and conquer algorithm, O(n^1.47).
* Large `parse` improved using a divide and conquer algorithm, O(n^1.47).
* Large `to_string` improved using a divide and conquer algorithm, O(n^1.47).
* Other minor performance improvements.

## 0.1.1 - 2021-03-03

### New features
* Hashing.
* Exponentiation.
* Random sampling (optional dependency on `rand 0.8`).

### Performance
* Multiplication improved, now uses Karatsuba and Toom-Cook-3 algorithms, O(n^1.47).

### Examples
* `factorial` prints 1000000! in hexadecimal.

## 0.1.0 - 2021-02-25

The initial usable version.

### Features
* All basic arithmetic and bitwise operations.
* Parsing and formatting.
* Constructor macros.

### Performance
* Operations on very large numbers are still slow.
