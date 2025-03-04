# `ceres-solver-rs`
## Rust bindings for [Ceres Solver](http://ceres-solver.org)

[![Test](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml/badge.svg)](https://github.com/light-curve/ceres-solver-rs/actions/workflows/test.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/light-curve/ceres-solver-rs/master.svg)](https://results.pre-commit.ci/latest/github/light-curve/ceres-solver-rs/master)
![docs.rs](https://img.shields.io/docsrs/ceres-solver)
![Crates.io](https://img.shields.io/crates/v/ceres-solver)

```shell
cargo add ceres-solver --features=source
```

Ceres Solver is a C++ library for large optimization problems.
It can be used to solve Non-linear Least Squares problems with constraints and general optimization problems.
Here we provide a Rust binding for this library.

The bindings require Ceres Solver version 2.2, but the bindings may work with older or new versions.
Please submit an issue if you need a support of other versions.

This project consists of three crates:
- `ceres-solver` is a safe Rust bindings
- `ceres-solver-sys` is an unsafe Rust bindings written with the usage of [`cxx`](https://lib.rs/crates/cxx)
- `ceres-solver-src` is an optional no-code crate to build and distribute a minimal static Ceres Solver library

To build Ceres Solver statically and link it to your project, use `source` Cargo feature, which would add `ceres-solver-src` dependency into your project.

### Status of the binding support

Current implementation of the binding is not complete.
The following list shows the status of the binding support:

- Non-linear Least squares
  - [x] `Problem` - basic class for NLLS, supports adding residual blocks, setting boundary conditions, marking parameter blocks to be constant/variable, and solving the problem
  - [x] `CostFunction` - user provides both residual and Jacobian
  - [ ] `SizedCostFunction` - same but with the residual vector shape is known at compile time
  - [ ] `AutoDiffCostFunction` - user provides residual and Jacobian is computed by automatic differentiation
  - [ ] `DynamicAutoDiffCostFunction` - same but with the residual vector shape is unknown at compile time
  - [ ] `NumericDiffCostFunction` - user provides residual and Jacobian is computed by numerical differentiation
  - [ ] `CostFunctionToFunctor` and `DynamicCostFunctionToFunctor` - adapter to use `CostFunction` as a mix of all other cost functions
  - [ ] `ConditionedCostFunction` - adapter to use `CostFunction` with different conditioning
  - [ ] `GradientChecker` - helper class to check the correctness of the Jacobian
  - [ ] `NormalPrior` - changes a cost function to use a covariance matrix instead of a simple scalar product
  - [x] `LossFunction` - a function applied to the squared norm of the residual vector, both custom and Ceres stack loss functions are supported
  - [ ] `Manifold`, `AutoDiffManifold`
  - [ ] `EvaluationCallback`
- Solver - `Solver` class itself is not implemented, but the following nested classes are supported:
  - `Solver::Options`
    - [x] Minimizer options
    - [x] Line search options
    - [x] Trust region options
    - [x] Linear solver options
    - [x] Preconditioner options
    - [x] Sparse and dense linear algebra library selection
    - [x] Setting of the number of threads
    - [ ] Bundle adjustment options
    - [x] Logging options
    - [x] Validation of the options
    - [ ] Callbacks
  - `Solver::Summary`
    - [x] Brief and full reports
    - [x] Cost function evaluation statistics
    - [ ] Time statistics
- [ ] Jets
- [ ] Covariance estimation
- [ ] General unconstrained minimization

Please don't hesitate to create an issue to request prioritization of any functionality that may have been prioritized.
