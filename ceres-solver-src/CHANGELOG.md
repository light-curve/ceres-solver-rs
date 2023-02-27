# Changelog

All notable changes to `ceres-solver-src` Rust crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

--

### Changed

- Build Ceres' dependency [`glog`](https://github.com/google/glog) from source. It is much better than previous approach with built-in `miniglog` from Ceres because it wasn't configurable and always output something

### Deprecated

--

### Removed

- CI: Windows build is removed, probably it doesn't build there anymore. Help needed to fix it.

### Fixed

--

### Security

--

## [0.1.1+ceres2.1.0-eigen3.4.0] 2023-02-09

### Added

- Include Eigen3 header files.
- Include miniglog header files.

## [0.1.0+ceres2.1.0-eigen3.4.0] 2023-01-19

Initial release
