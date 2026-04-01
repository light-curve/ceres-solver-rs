# CLAUDE.md

## Project Overview

Safe Rust bindings for [Ceres Solver](http://ceres-solver.org), a C++ library for solving large-scale non-linear least squares (NLLS) optimization problems. The project is a Cargo workspace with three crates:

- **`ceres-solver`**: Safe Rust bindings (main crate, `src/`)
- **`ceres-solver-sys`**: Unsafe FFI bindings via the `cxx` crate (`ceres-solver-sys/`)
- **`ceres-solver-src`**: Builds and statically links bundled Ceres Solver from source (`ceres-solver-src/`)

MSRV: **1.85.0** (Rust 2024 edition)

## Build and Test Commands

```bash
# Build with bundled Ceres (preferred for development)
cargo build --features source

# Build with system-installed Ceres
cargo build

# Check all targets
cargo check --all-targets --workspace --features source

# Test with bundled source
cargo test --features source

# Test with system Ceres
cargo test --features system

# Format code
cargo fmt --all

# Lint (no warnings allowed)
cargo clippy --all-targets --workspace --no-default-features --features source -- -Dwarnings

# Run all pre-commit checks
pre-commit run --all-files
```

## Project Structure

```
src/                    # Main crate (safe Rust API)
│   lib.rs              # Library entry point with top-level examples
│   nlls_problem.rs     # Core: NllsProblem builder
│   curve_fit.rs        # 1-D curve fitting convenience wrapper
│   solver.rs           # SolverOptions / SolverSummary
│   cost.rs             # CostFunction type definitions
│   loss.rs             # Loss functions (Huber, Cauchy, Tukey, …)
│   parameter_block.rs  # ParameterBlock with bounds management
│   residual_block.rs   # ResidualBlock ID types
│   error.rs            # Error types (thiserror)
│   types.rs            # Shared helper types
ceres-solver-sys/src/
│   lib.rs              # Rust FFI declarations (cxx)
│   lib.cpp             # C++ bridge implementation
│   lib.h               # C++ header
ceres-solver-src/       # CMake-based static library builder
.github/workflows/test.yml  # CI (Ubuntu + macOS, MSRV + stable)
```

## Code Conventions

- Standard Rust naming: `snake_case` for functions/variables, `PascalCase` for types
- **Builder pattern** throughout (e.g., `ResidualBlockBuilder`, `SolverOptionsBuilder`)
- **Consuming methods** — builders take `self` and return a new/modified type
- Error types use `thiserror`; all error enums live in `src/error.rs`
- Doc comments: `//!` for module-level, `///` for items; include examples where useful
- Minimize new dependencies; for MSRV compatibility, versions of `cxx`/`cxx-build` may be pinned

## Adding New Features

1. Check the feature exists in the supported Ceres Solver C++ API (2.2)
2. If adding FFI:
   - Update `ceres-solver-sys/src/lib.h` and `lib.cpp` (C++ bridge)
   - Update `ceres-solver-sys/src/lib.rs` (Rust `cxx` declarations)
3. Add safe Rust wrappers in the appropriate `src/` file
4. Write tests (unit tests in the same file; integration tests under `tests/`)
5. Use the `approx` crate for floating-point comparisons in tests
6. Update the feature status checklist in `README.md` if applicable
7. Add an entry to `CHANGELOG.md` (Keep a Changelog format, semantic versioning)

## CI

GitHub Actions runs on Ubuntu and macOS against MSRV (1.85) and stable Rust:
- `cargo-fmt` — formatting check
- `cargo-clippy` — lint check (`-Dwarnings`)
- `ceres-built-from-source` — tests with `--features source`
- `system-ceres` — tests with system-installed Ceres

Windows support is limited/experimental and not part of CI.

## FFI Safety Notes

- All unsafe FFI code is isolated in `ceres-solver-sys`; `ceres-solver` must remain safe
- Parameter blocks use pin semantics to ensure stable memory addresses across the FFI boundary
- Use modern C++17 features through `cxx`; avoid raw pointer arithmetic in new code
