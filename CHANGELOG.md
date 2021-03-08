# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Deprecated features
* `UBig::to_str_radix`
* `UBig::to_str_radix_uppercase`
* `IBig::to_str_radix`
* `IBig::to_str_radix_uppercase`

### Dependencies
* Added a dependency on `static_assertions 1.1`.

### Performance
* Large division improved. Now uses a divide and conquer algorithm, O(n^1.47).
* Parsing large numbers improved using a divide and conquer algorithm, O(n^1.47).
* Unbalanced operations (large x small) improved, avoid scanning memory many times.

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
