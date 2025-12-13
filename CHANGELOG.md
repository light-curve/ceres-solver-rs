# Changelog

All notable changes to `ceres-solver` Rust crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

--

### Changed

--

### Deprecated

--

### Removed

--

### Fixed

--

### Security

--

## [0.5.0] 2025-12-13

### Changed

- Bump `ceres-solver-sys` from `0.4.0` to `0.5.0`.
- **Build breking** Bump minimum supported Rust version (MSRV) to 1.85 to support MSRV-aware dependency resolution.
- Bump Rust edition from 2021 to 2024.

### Fixed

- Some clippy v0.1.83 lints
- Eigen headers detection on macOS with homebrew installation.

## [0.4.0] 2024-01-07

### Changed

- **Breaking** MSRV is changed from 1.57.0 to 1.67.0.

## [0.3.0] 2024-02-26

### Changed

- **Breaking** The only supported version of ceres-solver is 2.2 now, due to some breaking changes in the C++ API, this
  removes support of v2.0 and v2.1.

## [0.2.2] 2024-02-26

### Changed

- Bump `ceser-solver-sys` to `0.2.2`, which requires `ceres-solver` version to be between 2.0 and 2.1, because 2.2 is
  known to be incompatible.

## [0.2.1] 2023-03-02

### Changed

- Bump `ceres-solver-sys` to `0.2.1` which causes turning logging off by default. We don't consider it as a breaking
  change, but by default you will see no output now when using "source" Cargo feature.

### Removed

- CI and "source" feature: Windows build removed. Probably it doesn't work anymore, help needed to fix it.

## [0.2.0] 2023-02-11

### Added

- `LossFunction::tukey()`.
- `solver` module with customizable `SolverOptions` and `SolverSummary` containing the solution statistics.
- Reusing of the parameter blocks in the residual blocks.
- Make parameter block constant if you don't need to vary it.
- Boundary conditions for the parameter blocks.
- More documentation and examples.

### Changed

- **breaking** `ceres-solver-sys` is updated to `0.2` which gives access to more APIs of the C++ interface. It caused a
  lot of breaking changes in the `ceres-solver` crate in many ways.
- **breaking** `CostFunction` is not public anymore, `CostFunctionType` is the only thing you need to know about.
- **breaking** `LossFunction` is changed from `enum` to an opaque `struct`.
- **breaking** `LossFunction::tolerant_loss()` renamed into `LossFunction::tolerant()`.
- **breaking** `parameters` module renamed into `parameter_block` and provides a different interface now.
- **breaking** Residual blocks are now built from `NllsProblem` and capture it until the builder releases the problem
  back adding the residual block to it.
- **breaking** Solution of the both problem is via `::solve(self, options: &SolverOptions)` now and returns structures
  with the parameters and the summary.
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
