# Changelog

All notable changes to `ceres-solver` Rust crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

--

### Changed

- Bump `ceres-solver-sys` to `0.2.1` which causes turning logging off by default. We don't consider it as a breaking change, but by default you will see no output now when using "source" Cargo feature.

### Deprecated

--

### Removed

- - CI and "source" feature: Windows build removed. Probably it doesn't work anymore, help needed to fix it.

### Fixed

--

### Security

--

## [0.2.0] 2023-02-11

### Added

- `LossFunction::tukey()`.
- `solver` module with customizable `SolverOptions` and `SolverSummary` containing the solution statistics.
- Reusing of the parameter blocks in the residual blocks.
- Make parameter block constant if you don't need to vary it.
- Boundary conditions for the parameter blocks.
- More documentation and examples.

### Changed

- **breaking** `ceres-solver-sys` is updated to `0.2` which gives access to more APIs of the C++ interface. It caused a lot of breaking changes in the `ceres-solver` crate in many ways.
- **breaking** `CostFunction` is not public anymore, `CostFunctionType` is the only thing you need to know about.
- **breaking** `LossFunction` is changed from `enum` to an opaque `struct`.
- **breaking** `LossFunction::tolerant_loss()` renamed into `LossFunction::tolerant()`.
- **breaking** `parameters` module renamed into `parameter_block` and provides a different interface now.
- **breaking** Residual blocks are now built from `NllsProblem` and capture it until the builder releases the problem back adding the residual block to it.
- **breaking** Solution of the both problem is via `::solve(self, options: &SolverOptions)` now and returns structures with the parameters and the summary.
- More error types, they all use `thiserror` now.
- Many more **breaking** changes.

### Removed

- `loss::CustomLossFunction` and `loss::StockLossFunction`.
- Some more things.

## [0.1.2] 2023-01-24

### Added

- `CurveFitProblem1D::builder` which allows to build a curve fit problem with data point uncertainties.

## [0.1.1] 2023-01-20

### Added

- Cargo features for `ceres-solver` reflecting `ceres-solver-sys` https://github.com/light-curve/ceres-solver-rs/pull/3


### Fixed

- docs.rs build https://github.com/light-curve/ceres-solver-rs/pull/3

## [0.1.0] 2023-01-19

Initial release
