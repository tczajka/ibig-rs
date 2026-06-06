# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `count_ones`: population count

### Changed

- Renamed `next_power_of_two_in_place` to `next_power_of_two`.

## [0.0.1] - 2026-06-05

Work in progress. Low-level big-integer algorithms operating on slices of digits.

### Added

- The `Digit`/`SignedDigit` types: the platform's native unsigned/signed integer.
- Bit operations: bit access and assignment, bit width, trailing zeros/ones, power-of-two test, and rounding up to the next power of two.
- Bitwise logic on equal-length slices: NOT, AND, OR, XOR, and AND-NOT.
- Conversions to and from little- and big-endian bytes, for both unsigned magnitudes and signed two's-complement values.
- Length and sign helpers: canonical digit/byte lengths, sign extension, and the sign test for two's-complement values.
