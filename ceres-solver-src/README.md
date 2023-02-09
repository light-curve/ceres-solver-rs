# `ceres-solver-src`
## Rust distribution of [Ceres Solver](http://ceres-solver.org)

[![Test](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml/badge.svg)](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/light-curve/ceres-solver-rs/master.svg)](https://results.pre-commit.ci/latest/github/light-curve/ceres-solver-rs/master)
![Crates.io](https://img.shields.io/crates/v/ceres-solver-src)

Builds a minimalistic static library of Ceres Solver.
We build it using vendored versions of Ceres Solver and Eigen, so no internet access is required.
It still requires Ceres Solver build dependencies: `cmake` and C++17 compatible compiler.
