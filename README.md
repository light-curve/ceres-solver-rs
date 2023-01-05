# `ceres-solver-rs`
## Rust bindings for [Google's Ceres Solver](http://ceres-solver.org)

```shell
cargo add ceres-solver --features=ceres-solver-sys/source
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
 
To build Ceres Solver statically and link it to your project, use `ceres-solver-sys`' `source` Cargo feature,
for example by `cargo ... --features ceres-solver-sys/source`
or adding `ceres-sovler-sys = { version = "*", features = "source" }` into your `Cargo.toml`.