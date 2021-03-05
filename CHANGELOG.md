# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Dependencies
* Added a dependency on `static_assertions 1.1`.

### Performance
* Large division improved. Now uses a divide and conquer algorithm.
* Unbalanced multiplication (large x small) avoids scanning memory many times.

## 0.1.1 - 2021-03-03

### New features
* Hashing.
* Exponentiation.
* Random sampling (optional dependency on `rand 0.8`).

### Performance
* Multiplication improved, now uses Karatsuba and Toom-Cook-3 algorithms.

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
