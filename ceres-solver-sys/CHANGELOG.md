# Changelog

All notable changes to `ceres-solver-sys` Rust crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

--

### Changed

-

### Deprecated

--

### Removed

-

### Fixed

--

### Security

--

## [0.2.1] 2023-02-28

### Changed

- Bump `ceres-solver-src` to `0.2.0`. This replaces `miniglog` with `glog` and makes logging configurable. We don't consider it as a breaking change, but by default you will see no output now when using "source" Cargo feature.
- "source" feature: discover `eigen` using `pkg-config`.

### Removed

- CI and "source" feature: Windows build removed. Probably it doesn't work anymore, help needed to fix it.

## [0.2.0] 2023-02-09

### Added

- Support of a lot of new functions and classes throught C++ API.
- `v2_1` Cargo feature which must be enabled to use the crate with Ceres 2.1.0+.

### Changed

-  **breaking** the wrapper is rewritten from binding C API to C++ API with `cxx`. This change is backward incompatible in many ways.
- Minimum supported Ceres version is 2.0.0.
- Minimum supported Rust version is 1.57.0.

### Fixed

- Building issues with system library.

## [0.1.0] 2023-01-19

Initial release
