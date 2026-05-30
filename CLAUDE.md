# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`ibig` is a pure-Rust arbitrary-precision integer library (`no_std` compatible). The public types are `UBig` (unsigned), `IBig` (signed), and `Modulo`/`ModuloRing` (modular arithmetic). The crate aims for good performance, so much of the code is concerned with algorithm selection by operand size and per-architecture word primitives.

## Commands

```bash
cargo test --all-features                 # run all tests
cargo test --all-features <name>          # run tests matching <name>
cargo test --all-features --test mul      # run one integration test file (tests/mul.rs)

cargo build --no-default-features --features "rand, serde"   # verify no_std build
cargo fmt --all -- --check                # formatting check (CI-enforced)
cargo clippy --all-features --tests -- -D warnings           # lint (CI-enforced; warnings are errors)
cargo bench --features rand               # Criterion benchmarks (benches/benchmarks.rs)
```

Feature flags: `std` (default), `rand`, `serde`. Tests `tests/random.rs` and `tests/serde.rs` require their respective features (hence `--all-features` everywhere).

### Testing across word sizes

The word size is normally chosen from the target architecture, but it can be forced for testing. CI runs the full suite under each:

```bash
RUSTFLAGS='--cfg force_bits="16"' cargo test --all-features   # also 32, 64
```

Always consider whether a change behaves correctly at all three word sizes (16/32/64), since algorithm thresholds and NTT primes differ per size.

### Minimal dependency versions

`Cargo.lock.min` pins the oldest supported dependency versions. CI copies it over `Cargo.lock` and checks against rustc 1.61 (the MSRV) and stable. If you change dependencies, update both `Cargo.toml` and `Cargo.lock.min`.

### dev-tools

`dev-tools/` is a separate workspace member holding offline code-generation utilities, not part of the library. `cargo run -p dev-tools --bin ntt_primes` regenerates the NTT prime constants used in `src/arch/*/ntt.rs`.

## Architecture

### Number representation

- `UBig` wraps a private `Repr` enum (`src/ubig.rs`): `Small(Word)` for single-word values, `Large(Buffer)` otherwise. `Large` invariants: length ≥ 2, no leading zero word, compact capacity. Code branches on `repr()` constantly — preserve these invariants when constructing a `Large`.
- `IBig` (`src/ibig.rs`) is a `Sign` + magnitude (`UBig`).
- `Word` is the architecture's native unsigned integer (`u16`/`u32`/`u64`); numbers are little-endian `Word` slices.
- `Buffer` (`src/buffer.rs`) is the growable `Vec<Word>` backing `Large`. It deliberately over-allocates (~12.5% slack) and panics on capacity overflow rather than reallocating in hot paths — use `push` only with pre-reserved capacity, `push_may_reallocate` otherwise. `UBig::MAX_LEN` caps the word count.

### Architecture-specific code (`src/arch/`)

`src/arch/mod.rs` uses `cfg_if!` to select an implementation module (`x86`, `x86_64`, or `generic_{16,32,64}_bit`) based on `force_bits`, then `target_arch`, then `target_pointer_width`. Each module re-exports `word` (the `Word`/`SignedWord`/`DoubleWord` types and primitives), `add`, `digits`, and `ntt` (number-theoretic-transform primes/roots tuned to the word size). Anything word-size- or CPU-dependent (carry handling, multiword add) lives here; the rest of the crate is generic over `Word`.

### Algorithm selection by size

The big-integer operations dispatch to different algorithms based on operand length. Thresholds are `const`s near the top of each module:

- **Multiplication** (`src/mul/`): schoolbook (`simple`) → Karatsuba → Toom-3 → NTT (`ntt`), with cutoffs `MAX_LEN_SIMPLE`/`MAX_LEN_KARATSUBA` in `src/mul/mod.rs`.
- **Division** (`src/div/`): `simple` (schoolbook) vs `divide_conquer` (recursive).
- **Radix conversion** (`src/parse/`, `src/fmt/`): `power_two` (bases that are powers of 2, bit-shift based) vs `non_power_two` (general, divide-and-conquer for large numbers).

When changing a threshold or adding an algorithm, the `const_assert!`s wiring adjacent thresholds together (e.g. `MAX_LEN_SIMPLE + 1 >= karatsuba::MIN_LEN`) must stay satisfied.

### Scratch memory (`src/memory.rs`)

Recursive algorithms (Karatsuba, Toom-3, NTT, divide-and-conquer division) need temporary `Word` scratch space. Rather than allocating per recursion, a single `MemoryAllocation` is sized up front and sub-slices are handed out via `Memory` regions. Functions expose a `*_memory_requirement` helper computing the needed `Layout`. Follow this pattern for new recursive algorithms instead of allocating inline.

### Operator trait boilerplate (`src/helper_macros.rs`)

Arithmetic operators are implemented once for the owned/owned case, and the macros in `helper_macros.rs` (`forward_binop_first_arg_by_value`, etc.) generate the `&A op B`, `A op &B`, `&A op &B` variants by forwarding. The user-facing `ubig!` / `ibig!` literal macros live in `src/macros.rs`.

### File layout convention

Per operation, low-level slice-on-`Word` algorithms live in a topic file or directory (e.g. `add.rs`, `mul/`, `div/`) while the public `UBig`/`IBig` trait impls live in the `*_ops.rs` counterpart (`add_ops.rs`, `mul_ops.rs`, `div_ops.rs`, `shift_ops.rs`). Modular arithmetic is fully contained in `src/modular/`.

## Conventions

- Public API changes must be recorded in `CHANGELOG.md`; note breaking changes explicitly.
- MSRV is rustc 1.61 — avoid newer standard-library APIs and language features.
- The crate is `no_std`; use `alloc` (e.g. `alloc::vec::Vec`) rather than `std`, and gate any `std`-only code behind `#[cfg(feature = "std")]`.
