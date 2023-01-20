# `ceres-solver-rs`
## Rust bindings for [Ceres Solver](http://ceres-solver.org)

[![Test](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml/badge.svg)](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/light-curve/ceres-solver-rs/master.svg)](https://results.pre-commit.ci/latest/github/light-curve/ceres-solver-rs/master)
![docs.rs](https://img.shields.io/docsrs/ceres-solver)
![Crates.io](https://img.shields.io/crates/v/ceres-solver)

```shell
cargo add ceres-solver --features=source
```

Ceres Solver is a C++ library for large optimization problems.
It can be used to solve Non-linear Least Squares problems with constraints and general optimization problems.
Here we provide a Rust binding for this library.
Current implementation is built upon Ceres Solver's C API which has very limited scope:
Non-linear Least Squares problems with analytical derivatives and custom loss function.

The earliest Ceres Solver version tested is 1.14.0, but the bindings may work with older versions

This project consists of three crates:
- `ceres-solver` is a safe Rust bindings
- `ceres-solver-sys` is an unsafe Rust bindings generated with `bindgen`
- `ceres-solver-src` is an optional no-code crate to build and distribute static Ceres Solver library

To build Ceres Solver statically and link it to your project, use `source` Cargo feature, which would add `ceres-solver-src` dependency to your project.
