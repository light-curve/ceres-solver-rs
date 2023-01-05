# `ceres-solver-sys`
## Low-level unsafe Rust bindings for [Google's Ceres Solver](http://ceres-solver.org)

Currently, we bind C API only using [`bindgen`](https://rust-lang.github.io/rust-bindgen/).
The minimum tested version of Ceres Solver is 1.14.0

### Cargo feature flags
- `system` (default) links a system copy of the Ceres Solver library. By default, it would be a synamic library, but you can tweak it with `pkg-config`
- `source` (optional) overrides `system` and links a static library file built by `ceres-solver-src` crate

Since this crate uses `bindgen` it requires `libclang` as build dependency.
For `system` feature it also required `pkg-config` to discover the library.