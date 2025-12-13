# Changelog

All notable changes to `ceres-solver-src` Rust crate will be documented in this file.

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

## [0.5.0+ceres2.2.0-eigen3.4.0-glog0.7.1] 2025-12-13

### Changed

- **Build breking** Bump minimum supported Rust version (MSRV) to 1.85 to support MSRV-aware dependency resolution.
- Bump Rust edition from 2021 to 2024.

## [0.4.0+ceres2.2.0-eigen3.4.0-glog0.7.1] 2025-04-18

- **Breaking** Update `glog` to 0.7.1. It requires to define some additional flags to use, e.g. GLOG_USE_GLOG_EXPORT

## [0.3.0+ceres2.2.0-eigen3.4.0-glog0.7.0] 2024-02-26

### Notes

Despite the version name, glog 0.6.0 was used.

### Changed

- **Breaking** `ceres-solver` is updated to 2.2, which is backward incompatible with 2.1.0
- `glog` is being built without `unwind` support

## [0.2.0+ceres2.1.0-eigen3.4.0-glog0.6.0] 2023-02-28

### Changed

- Build Ceres' dependency [`glog`](https://github.com/google/glog) from source. It is much better than previous approach
  with built-in `miniglog` from Ceres because it wasn't configurable and always output something

### Removed

- CI: Windows build is removed, probably it doesn't build there anymore. Help needed to fix it.

## [0.1.1+ceres2.1.0-eigen3.4.0] 2023-02-09

### Added

- Include Eigen3 header files.
- Include miniglog header files.

## [0.1.0+ceres2.1.0-eigen3.4.0] 2023-01-19

Initial release
