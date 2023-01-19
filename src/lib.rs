//! # ceres-solver-rs
//! ## Safe Rust bindings for [Ceres Solver](http://ceres-solver.org)
//!
//! Solve large and small non-linear optimization problems in Rust.
//! See [nlls_problem::NllsProblem] for general non-linear least squares problem and
//! [curve_fit::CurveFitProblem1D] for a multiparametric 1-D curve fitting.
//!
//! # Examples
//!
//! Let's solve min[(x - 2)^2] problem
//!
//! ```rust
//! use ceres_solver::{CostFunction, CostFunctionType, NllsProblem, ResidualBlock};
//!
//! // parameters vector consists of vector parameters, here we have a single 1-D parameter.
//! let true_parameters = vec![vec![2.0]];
//! let initial_parameters = vec![vec![0.0]];
//! // This must be equal to initial_parameters.iter().map(|v| v.len()).collect();
//! let parameter_sizes = [1];
//!
//! // You can skip type annotations in the closure definition, we use them for verbosity only.
//! let cost: CostFunctionType = Box::new(
//!     move |parameters: &[&[f64]],
//!           residuals: &mut [f64],
//!           mut jacobians: Option<&mut [Option<&mut [&mut [f64]]>]>| {
//!         // residuals have the size of your data set, in our case it is just 1
//!         residuals[0] = parameters[0][0] - 2.0;
//!         // jacobians can be None, then you don't need to provide them
//!         if let Some(jacobians) = jacobians {
//!             // The size of the jacobians array is equal to the number of parameters,
//!             // each element is Option<&mut [&mut [f64]]>
//!             if let Some(d_dx) = &mut jacobians[0] {
//!                 // Each element in the jacobians array is slice of slices:
//!                 // the first index is for different residuals components,
//!                 // the second index is for different components of the parameter vector
//!                 d_dx[0][0] = 1.0;
//!             }
//!         }
//!         true
//!     },
//! );
//! // 1 is the number of residuals.
//! let cost_function = CostFunction::new(cost, parameter_sizes, 1);
//!
//! let mut problem = NllsProblem::new();
//!
//! // There could be many residual blocks, each has its own parameters.
//! problem
//!     .add_residual_block(ResidualBlock::new(initial_parameters, cost_function))
//!     .unwrap();
//! // solution is a vector of parameters, one per residual block. Type annotation is not needed.
//! let solution: Vec<Vec<Vec<f64>>> = problem.solve().unwrap();
//!
//! assert!(f64::abs(solution[0][0][0] - true_parameters[0][0]) < 1e-8);
//! ```
//!
//! We also provide a lighter interface for 1-D multiparameter curve fit problems via
//! [CurveFitProblem1D]. Let's generate data points and fit them for a quadratic function.
//!
//! ```rust
//! use ceres_solver::{CurveFitProblem1D, CurveFunctionType};
//!
//! // A model to use for the cost function.
//! fn model(
//!     x: f64,
//!     parameters: &[f64],
//!     y: &mut f64,
//!     mut jacobians: Option<&mut [Option<f64>]>,
//! ) -> bool {
//!     let &[a, b, c]: &[f64; 3] = parameters.try_into().unwrap();
//!     *y = a * x.powi(2) + b * x + c;
//!     if let Some(jacobians) = jacobians {
//!         let [d_da, d_db, d_dc]: &mut [Option<f64>; 3] = jacobians.try_into().unwrap();
//!         if let Some(d_da) = d_da {
//!             *d_da = x.powi(2);
//!         }
//!         if let Some(d_db) = d_db {
//!             *d_db = x;
//!         }
//!         if let Some(d_dc) = d_dc {
//!             *d_dc = 1.0;
//!         }
//!     }
//!     true
//! }
//!
//! let true_parameters = [1.0, 2.0, 3.0];
//!
//! // Generate data points.
//! let x: Vec<_> = (0..100).map(|i| i as f64 * 0.01).collect();
//! let y: Vec<_> = x
//!     .iter()
//!     .map(|&x| {
//!         let mut y = 0.0;
//!         model(x, &true_parameters, &mut y, None);
//!         // True value + "noise"
//!         y + 0.001 + f64::sin(1e6 * x)
//!     })
//!     .collect();
//!
//! // Wrap the model to be a cost function.
//! let cost: CurveFunctionType = Box::new(model);
//!
//! // Solve it!
//! let initial_guess = [0.0, 0.0, 0.0];
//! let solution = CurveFitProblem1D::new(cost, &x, &y, &initial_guess, None).to_solution();
//!
//! // Check the results.
//! for (true_value, actual_value) in true_parameters.into_iter().zip(solution.into_iter()) {
//!     assert!(f64::abs(true_value - actual_value) < 0.1);
//! }
//! ```

pub use cost::{CostFunction, CostFunctionType};
pub use curve_fit::{CurveFitProblem1D, CurveFunctionType};
pub use loss::{LossFunction, LossFunctionType};
pub use nlls_problem::NllsProblem;
pub use parameters::Parameters;
pub use residual_block::ResidualBlock;

pub mod cost;
pub mod curve_fit;
pub mod error;
pub mod loss;
pub mod nlls_problem;
pub mod parameters;
pub mod residual_block;
pub mod types;
