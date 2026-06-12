# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Bit operations: `BitIndex`, `DIGIT_BITS_USIZE`, `count_ones`
- Addition: `add`, `add_same_len`, `add_digit`, `add_carry`, `add_1`.
- Small bit shifts (less than a digit): `shl_small`, `shr_small`, `shl_small_signed`,
  `shr_small_signed`, `shl_small_digit`, `shl_small_signed_digit`.
- The sign-extension byte for a most-significant byte: `sign_extension_byte`.

### Changed

- Renamed `next_power_of_two_in_place` to `next_power_of_two`.
- `bit`, `bit_signed` and `set_bit` now take a `BitIndex` instead of a bit position `usize`.
- Replaced `bit_width`, `trailing_zeros` and `trailing_ones` with `highest_one`, `lowest_one`
  and `lowest_zero`, which return `Option<BitIndex>`.
- `sign_extension` now takes and returns `SignedDigit` instead of `Digit`.

## [0.0.1] - 2026-06-05

Initial version with very little functionality.

### Added

- The `Digit`/`SignedDigit` types: the platform's native unsigned/signed integer.
- Bit operations: bit access and assignment, bit width, trailing zeros/ones, power-of-two test, and rounding up to the next power of two.
- Bitwise logic on equal-length slices: NOT, AND, OR, XOR, and AND-NOT.
- Conversions to and from little- and big-endian bytes, for both unsigned magnitudes and signed two's-complement values.
- Length and sign helpers: canonical digit/byte lengths, sign extension, and the sign test for two's-complement values.
