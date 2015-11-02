# `bn`

Big number library for sake of [Octavo](https://github.io/libOctavo/octavo).

# How to use that

**YOU SHOULD NOT USE THAT!!!**

This is created solely for Octavo usage and should not be considered as general
purpose big numbers library. This use fixed size arrays of limbs and doesn't
even warn on overflows or underflows, it just drop them into `/dev/null`.

## What should I use for big numbers?

There are some options:

- [`num::BigInt` and `num::BigUint`][num-crate] - native Rust library that was
  extracted from `rust-deprecated`. Slow, but works on stable.
- [`ramp`][ramp] - fast Rust library for big numbers. Part of it use `asm!` so
  it needs Rust nightly.
- [GMP][gmp] - Rust bindings to GNU Multi Precision library.

# License

See [LICENSE](LICENSE)
