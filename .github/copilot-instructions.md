# Copilot Instructions for ceres-solver-rs

## Project Overview

This repository provides safe Rust bindings for [Ceres Solver](http://ceres-solver.org), a C++ library for solving large-scale optimization problems. The project consists of three main crates:

- **`ceres-solver`**: Safe Rust bindings (main crate)
- **`ceres-solver-sys`**: Unsafe Rust bindings using [`cxx`](https://lib.rs/crates/cxx)
- **`ceres-solver-src`**: Optional crate to build and distribute a minimal static Ceres Solver library

## Build and Test Commands

### Building

```bash
# Build with system Ceres Solver (default)
cargo build

# Build with bundled Ceres Solver source
cargo build --features source

# Check all targets
cargo check --all-targets --workspace --features source
```

### Testing

```bash
# Test with bundled source
cargo test --features source

# Test with system Ceres Solver
cargo test --features system
```

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all --check

# Run clippy
cargo clippy --all-targets --workspace --no-default-features --features source -- -Dwarnings
```

### Pre-commit Hooks

The project uses pre-commit hooks. Run checks with:

```bash
pre-commit run --all-files
```

## Project Structure

```
.
├── src/                    # Main crate source (safe bindings)
│   ├── lib.rs             # Main library entry point with examples
│   ├── cost.rs            # Cost function types
│   ├── curve_fit.rs       # 1-D curve fitting utilities
│   ├── error.rs           # Error types (uses thiserror)
│   ├── loss.rs            # Loss function implementations
│   ├── nlls_problem.rs    # Non-linear least squares problem builder
│   ├── parameter_block.rs # Parameter block management
│   ├── residual_block.rs  # Residual block builder
│   ├── solver.rs          # Solver options and summary
│   └── types.rs           # Common types
├── ceres-solver-sys/      # Unsafe FFI bindings
│   └── src/
│       ├── lib.rs         # Rust FFI bindings
│       ├── lib.cpp        # C++ bridge code
│       └── lib.h          # C++ header
└── ceres-solver-src/      # Optional static library builder
```

## Code Style and Conventions

### General Rust Guidelines

- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `cargo fmt` for consistent formatting
- Pass `cargo clippy` with `-Dwarnings` (no warnings allowed)
- Check `Cargo.toml` for the current MSRV (minimum supported Rust version)

### Error Handling

- Use `thiserror` crate for error types
- Error enums are in `src/error.rs`
- Use descriptive error messages with context

### Documentation

- Use Rust doc comments (`//!` for module-level, `///` for item-level)
- Include examples in doc comments where appropriate
- See `src/lib.rs` for documentation style examples
- Include ASCII diagrams for complex workflows when helpful

### API Design Patterns

- **Builder Pattern**: Used extensively (e.g., `ResidualBlockBuilder`, problem builders)
- **Consuming Methods**: Many methods consume `self` and return a new or modified type (e.g., builder methods)
- **Type Safety**: Leverage Rust's type system to prevent misuse at compile time

### Dependencies

- Minimize new dependencies
- For FFI: `cxx` crate (version may be constrained for MSRV compatibility)
- For errors: `thiserror` crate

## Development Workflow

### Adding New Features

1. Check if the feature exists in the supported version of Ceres Solver C++ API
2. If adding FFI bindings:
   - Add C++ bridge code in `ceres-solver-sys/src/lib.cpp` and `lib.h`
   - Add Rust FFI bindings in `ceres-solver-sys/src/lib.rs` using `cxx`
3. Add safe Rust wrappers in appropriate `src/` files
4. Add comprehensive documentation with examples
5. Add tests in the same file or in `tests/` directory
6. Update `README.md` status checklist if implementing listed features
7. Run formatting and linting before committing
8. Update `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format

### Testing Strategy

- Unit tests in the same files as the code
- Integration tests in `tests/` directory (if present)
- Use `approx` crate for floating-point comparisons
- Test both with `source` and `system` features where applicable

### Changelog Maintenance

- Follow semantic versioning
- Categorize changes: Added, Changed, Deprecated, Removed, Fixed, Security
- Mark breaking changes with **Breaking** prefix

## CI/CD

- GitHub Actions workflow in `.github/workflows/test.yml`
- Tests run on Ubuntu and macOS (Windows not supported yet)
- Tests run with MSRV and stable Rust toolchains
- Pre-commit.ci runs formatting checks

## Common Tasks

### Adding a Cost Function Type

1. Add FFI bindings in `ceres-solver-sys/src/`
2. Add safe wrapper in `src/cost.rs`
3. Update examples in `src/lib.rs` if applicable
4. Add tests with various parameter configurations

### Adding Solver Options

1. Update FFI bindings in `ceres-solver-sys/src/lib.rs` and `.cpp`
2. Add options to `SolverOptions` or related builders in `src/solver.rs`
3. Ensure options are validated where necessary
4. Document the option with details from Ceres Solver documentation

### Fixing Build Issues

- For linking issues, check platform-specific configuration in CI workflow
- Ensure CMake is available for building from source
- Check `LIBRARY_PATH`, `LD_LIBRARY_PATH`, and `C_INCLUDE_PATH` for system Ceres

## Special Considerations

- **FFI Safety**: All FFI code is in `ceres-solver-sys`; keep `ceres-solver` safe
- **Memory Management**: Be careful with lifetime and ownership when crossing FFI boundary
- **C++ Features**: The project uses modern C++ features through `cxx`
- **Platform Support**: Ubuntu and macOS are fully supported; Windows support is limited
- **Version Constraints**: Some dependencies like `cxx` and `cxx-build` may be version-constrained for MSRV compatibility

## Resources

- [Ceres Solver Documentation](http://ceres-solver.org)
- [cxx crate documentation](https://docs.rs/cxx)
- Repository status checklist in `README.md` shows implemented features
