//! # ceres-solver-rs
//! ## Safe Rust bindings for [Ceres Solver](http://ceres-solver.org)
//!
//! Solve large and small non-linear optimization problems in Rust.
//! See [NllsProblem] for general non-linear least squares problem and
//! [CurveFitProblem1D] for a multiparametric 1-D curve fitting.
//!
//! # Examples
//!
//! Let's solve min[(x - 2)^2] problem
//!
//! ```rust
//! use ceres_solver::{CostFunctionType, NllsProblem, SolverOptions};
//!
//! // parameters vector consists of vector parameters, here we have a single 1-D parameter.
//! let true_parameters = vec![vec![2.0]];
//! let initial_parameters = vec![vec![0.0]];
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
//!
//! let solution = NllsProblem::new()
//!     .residual_block_builder() // create a builder for residual block
//!     .set_cost(cost, 1) // 1 is the number of residuals
//!     .set_parameters(initial_parameters)
//!     .build_into_problem()
//!     .unwrap()
//!     .0 // build_into_problem returns a tuple (NllsProblem, ResidualBlockId)
//!     // You can repeat .residual_block_builder() and .build_into_problem() calls to add more
//!     // residual blocks
//!     .solve(&SolverOptions::default()) // SolverOptions can be customized
//!     .unwrap(); // Err should happen only if we added no residual blocks
//!
//! // Print the full solver report
//! println!("{}", solution.summary.full_report());
//!
//! // The solution is a vector of parameter vectors, here we have a single 1-D parameter.
//! assert!(f64::abs(solution.parameters[0][0] - true_parameters[0][0]) < 1e-8);
//! ```
//!
//! See more details and examples in [nlls_problem] module documentation.
//!
//! We also provide a lighter interface for 1-D multiparameter curve fit problems via
//! [CurveFitProblem1D]. Let's generate data points and fit them for a quadratic function.
//!
//! ```rust
//! use ceres_solver::{CurveFitProblem1D, CurveFunctionType, SolverOptions};
//!
//! // A model to use for the cost function.
//! fn model(
//!     x: f64,
//!     parameters: &[f64],
//!     y: &mut f64,
//!     jacobians: Option<&mut [Option<f64>]>,
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
//! // Create and solve the problem.
//! let initial_guess = [0.0, 0.0, 0.0];
//! let solution =
//!     CurveFitProblem1D::new(cost, &x, &y, &initial_guess).solve(&SolverOptions::default());
//!
//! // Print the brief report
//! print!("{:?}", solution.summary);
//!
//! // Check the results.
//! for (true_value, actual_value) in true_parameters
//!     .into_iter()
//!     .zip(solution.parameters.into_iter())
//! {
//!     assert!(f64::abs(true_value - actual_value) < 0.1);
//! }
//! ```
//!
//! See another example in [curve_fit::CurveFitProblem1DBuilder]'s documentation.

pub use cost::CostFunctionType;
pub use curve_fit::{CurveFitProblem1D, CurveFunctionType};
pub use loss::{LossFunction, LossFunctionType};
pub use nlls_problem::NllsProblem;
pub use parameter_block::{ParameterBlock, ParameterBlockOrIndex};
pub use solver::SolverOptions;

pub mod cost;
pub mod curve_fit;
pub mod error;
pub mod loss;
pub mod nlls_problem;
pub mod parameter_block;
pub mod residual_block;
pub mod solver;
pub mod types;
