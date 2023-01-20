# `ceres-solver-sys`
## Low-level unsafe Rust bindings for [Ceres Solver](http://ceres-solver.org)

[![Test](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml/badge.svg)](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/light-curve/ceres-solver-rs/master.svg)](https://results.pre-commit.ci/latest/github/light-curve/ceres-solver-rs/master)
![Crates.io](https://img.shields.io/crates/v/ceres-solver-sys)

Currently, we bind C API only using [`bindgen`](https://rust-lang.github.io/rust-bindgen/).
The minimal tested version of Ceres Solver is 1.14.0

### Cargo feature flags
- `system` (default) links a system copy of the Ceres Solver library. By default, it would be a synamic library, but you can tweak it with `pkg-config`
- `source` (optional) overrides `system` and links a static library file built by `ceres-solver-src` crate

Since this crate uses `bindgen` it requires `libclang` as build dependency.
For `system` feature it also required `pkg-config` to discover the library.
