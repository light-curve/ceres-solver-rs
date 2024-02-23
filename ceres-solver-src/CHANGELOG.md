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

## [0.3.0+cerese2.2.0-eigen3.4.0-glog0.7.0] 2024-02-26

### Changed

- **Breaking** `ceres-solver` is updated to 2.2, which is backward incompatible with 2.1.0
- `glog` is being built without `unwind` support

## [0.2.0+ceres2.1.0-eigen3.4.0-glog0.6.0] 2023-02-28

### Changed

- Build Ceres' dependency [`glog`](https://github.com/google/glog) from source. It is much better than previous approach with built-in `miniglog` from Ceres because it wasn't configurable and always output something

### Removed

- CI: Windows build is removed, probably it doesn't build there anymore. Help needed to fix it.

## [0.1.1+ceres2.1.0-eigen3.4.0] 2023-02-09

### Added

- Include Eigen3 header files.
- Include miniglog header files.

## [0.1.0+ceres2.1.0-eigen3.4.0] 2023-01-19

Initial release
