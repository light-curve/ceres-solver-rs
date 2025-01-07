//! Non-Linear Least Squares problem builder and solver.
//!
//! The diagram shows the lifecycle of a [NllsProblem]:
//! ```text
//!        x
//!        │ NllsProblem::new()
//!        │
//!     ┌──▼────────┐ .solve(self, options) ┌───────────────────┐
//! ┌──►│NllsProblem├──────────────────────►│NllsProblemSolution│
//! │   └──┬────────┘                       └───────────────────┘
//! │      │  .residual_block_builder(self)
//! │   ┌──▼─────────────────┐
//! │   │ResidualBlockBuilder│
//! │   └──▲─┬───────────────┘
//! │      │ │.set_cost(self, func, num_residuals)
//! │      └─┤
//! │      ▲ │
//! │      └─┤.set_loss(self, loss)
//! │        │
//! │      ▲ │
//! │      └─┤.set_parameters(self,
//! │        │
//! └────────┘.build_into_problem(self)
//! ```
//! <!-- https://asciiflow.com/#/share/eJytU1tqg0AU3cpwvyKIpPlq%2FcwCSml%2FB0TNDUivM2EerSFkF8WF5LN0NV1JR502NRpbSIajnEHuOXfOHXcg0hIhFpYoBEq3qCCGHYeKQ3wzny9CDltHF7d3jhmsjNtwYN2qOBeefr59sHsi%2FaBkRljGscDXWdD77jeOedTvRz4Ai7SkF5xppHXI5MYUUuiATVT8B66HE%2F9fTV%2BofUb1SZJtmv9x72UwCTa%2BrpLBcWwsUqiLlU0pyUjmz0lmC1qhaqMPRnquLzV270dvuWwcl53heEL14TrHdE%2Bk0SS51MbfqrUVeciELZPvBHRwUjaiVR%2FYUL5Da0BSa2%2FQ0J4iG5T%2BpbZJlftDDSqveUZtAlE7z6QQRvqRwh72XxPrJEg%3D) -->
//!
//! We start with [NllsProblem] with no residual blocks and cannot be solved. Next we should add a
//! residual block by calling [NllsProblem::residual_block_builder] which is a destructive method
//! which consumes the problem and returns a [ResidualBlockBuilder] which can be used to build a new
//! residual block. Here we add mandatory cost function [crate::cost::CostFunctionType] and
//! parameter blocks [crate::parameter_block::ParameterBlock]. We can also set optional loss
//! function [crate::loss::LossFunction]. Once we are done, we call
//! [ResidualBlockBuilder::build_into_problem] which returns previously consumed [NllsProblem].
//! Now we can optionally add more residual blocks repeating the process: call
//! [NllsProblem::residual_block_builder] consuming [NllsProblem], add what we need and rebuild the
//! problem. The only difference that now we can re-use parameter blocks used in the previous
//! residual blocks, adding them by their indexes. Once we are done, we can call
//! [NllsProblem::solve] which consumes the problem, solves it and returns [NllsProblemSolution]
//! which contains the solution and summary of the solver run. It returns an error if the problem
//! has no residual blocks.
//!
//! # Examples
//!
//! ## Multiple residual blocks with shared parameters
//!
//! Let's solve a problem of fitting a family of functions `y_ij = a + b_i * exp(c_i * x_ij)`:
//! all of them have the same offset `a`, but different scale parameters `b_i` and `c_i`,
//! `i in 0..=k-1` for `k` (`N_CURVES` bellow) different sets of data.
//!
//! ```rust
//! use ceres_solver::parameter_block::ParameterBlockOrIndex;
//! use ceres_solver::{CostFunctionType, NllsProblem, SolverOptions};
//!
//! // Get parameters, x, y and return tuple of function value and its derivatives
//! fn target_function(parameters: &[f64; 3], x: f64) -> (f64, [f64; 3]) {
//!     let [a, b, c] = parameters;
//!     let y = a + b * f64::exp(c * x);
//!     let dy_da = 1.0;
//!     let dy_db = f64::exp(c * x);
//!     let dy_dc = b * x * f64::exp(c * x);
//!     (y, [dy_da, dy_db, dy_dc])
//! }
//!
//! const N_OBS_PER_CURVE: usize = 100;
//! const N_CURVES: usize = 3;
//!
//! // True parameters
//! let a_true = -2.0;
//! let b_true: [_; N_CURVES] = [2.0, 2.0, -1.0];
//! let c_true: [_; N_CURVES] = [3.0, -1.0, 3.0];
//!
//! // Initial parameter guesses
//! let a_init = 0.0;
//! let b_init = 1.0;
//! let c_init = 1.0;
//!
//! // Generate data
//! let x = vec![
//!     (0..N_OBS_PER_CURVE)
//!         .map(|i| (i as f64) / (N_OBS_PER_CURVE as f64))
//!         .collect::<Vec<_>>();
//!     3
//! ];
//! let y: Vec<Vec<_>> = x
//!     .iter()
//!     .zip(b_true.iter().zip(c_true.iter()))
//!     .map(|(x, (&b, &c))| {
//!         x.iter()
//!             .map(|&x| {
//!                 let (y, _) = target_function(&[a_true, b, c], x);
//!                 // True value + "noise"
//!                 y + 0.001 + f64::sin(1e6 * x)
//!             })
//!             .collect()
//!     })
//!     .collect();
//!
//! // Build the problem
//! let mut problem = NllsProblem::new();
//! for (i, (x, y)) in x.into_iter().zip(y.into_iter()).enumerate() {
//!     let cost: CostFunctionType = Box::new(
//!         move |parameters: &[&[f64]],
//!               residuals: &mut [f64],
//!               mut jacobians: Option<&mut [Option<&mut [&mut [f64]]>]>| {
//!             assert_eq!(parameters.len(), 3);
//!             let a = parameters[0][0];
//!             let b = parameters[1][0];
//!             let c = parameters[2][0];
//!             // Number of residuls equal to the number of observations
//!             assert_eq!(residuals.len(), N_OBS_PER_CURVE);
//!             for (j, (&x, &y)) in x.iter().zip(y.iter()).enumerate() {
//!                 let (y_model, derivatives) = target_function(&[a, b, c], x);
//!                 residuals[j] = y - y_model;
//!                 // jacobians can be None, then you don't need to provide them
//!                 if let Some(jacobians) = jacobians.as_mut() {
//!                     // The size of the jacobians array is equal to the number of parameters,
//!                     // each element is Option<&mut [&mut [f64]]>
//!                     for (mut jacobian, &derivative) in jacobians.iter_mut().zip(&derivatives) {
//!                         if let Some(jacobian) = &mut jacobian {
//!                             // Each element in the jacobians array is slice of slices:
//!                             // the first index is for different residuals components,
//!                             // the second index is for different components of the parameter vector
//!                             jacobian[j][0] = -derivative;
//!                         }
//!                     }
//!                 }
//!             }
//!             true
//!         },
//!     );
//!     let a_parameter: ParameterBlockOrIndex = if i == 0 {
//!         vec![c_init].into()
//!     } else {
//!         0.into()
//!     };
//!     problem = problem
//!         .residual_block_builder()
//!         .set_cost(cost, N_OBS_PER_CURVE)
//!         .add_parameter(a_parameter)
//!         .add_parameter(vec![b_init])
//!         .add_parameter(vec![c_init])
//!         .build_into_problem()
//!         .unwrap()
//!         .0;
//! }
//!
//! // Solve the problem
//! let solution = problem.solve(&SolverOptions::default()).unwrap();
//! println!("Brief summary: {:?}", solution.summary);
//! // Getting parameter values
//! let a = solution.parameters[0][0];
//! assert!((a - a_true).abs() < 0.03);
//! let (b, c): (Vec<_>, Vec<_>) = solution.parameters[1..]
//!     .chunks(2)
//!     .map(|sl| (sl[0][0], sl[1][0]))
//!     .unzip();
//! for (b, &b_true) in b.iter().zip(b_true.iter()) {
//!     assert!((b - b_true).abs() < 0.03);
//! }
//! for (c, &c_true) in c.iter().zip(c_true.iter()) {
//!     assert!((c - c_true).abs() < 0.03);
//! }
//! ```
//!
//! ## Parameter constraints
//!
//! Let's find a minimum of the Himmelblau's function:
//! `f(x, y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2` with boundaries `x ∈ [0; 3.5], y ∈ [-1.8; 3.5]`
//! and initial guess `x = 3.45, y = -1.8`. This function have four global minima, all having the
//! the same value `f(x, y) = 0`, one of them is within the boundaries and another one is just
//! outside of them, near the initial guess. The solver converges to the corner of the boundary.
//!
//! ```rust
//! use ceres_solver::{CostFunctionType, NllsProblem, ParameterBlock, SolverOptions};
//!
//! const LOWER_X: f64 = 0.0;
//! const UPPER_X: f64 = 3.5;
//! const LOWER_Y: f64 = -1.8;
//! const UPPER_Y: f64 = 3.5;
//!
//! fn solve_himmelblau(initial_x: f64, initial_y: f64) -> (f64, f64) {
//!     let x_block = {
//!         let mut block = ParameterBlock::new(vec![initial_x]);
//!         block.set_all_lower_bounds(vec![LOWER_X]);
//!         block.set_all_upper_bounds(vec![UPPER_X]);
//!         block
//!     };
//!     let y_block = {
//!         let mut block = ParameterBlock::new(vec![initial_y]);
//!         block.set_all_lower_bounds(vec![LOWER_Y]);
//!         block.set_all_upper_bounds(vec![UPPER_Y]);
//!         block
//!     };
//!
//!     // You can skip type annotations in the closure definition, we use them for verbosity only.
//!     let cost: CostFunctionType = Box::new(
//!         move |parameters: &[&[f64]],
//!               residuals: &mut [f64],
//!               mut jacobians: Option<&mut [Option<&mut [&mut [f64]]>]>| {
//!             let x = parameters[0][0];
//!             let y = parameters[1][0];
//!             // residuals have the size of your data set, in our case it is two
//!             residuals[0] = x.powi(2) + y - 11.0;
//!             residuals[1] = x + y.powi(2) - 7.0;
//!             // jacobians can be None, then you don't need to provide them
//!             if let Some(jacobians) = jacobians {
//!                 // The size of the jacobians array is equal to the number of parameters,
//!                 // each element is Option<&mut [&mut [f64]]>
//!                 if let Some(d_dx) = &mut jacobians[0] {
//!                     // Each element in the jacobians array is slice of slices:
//!                     // the first index is for different residuals components,
//!                     // the second index is for different components of the parameter vector
//!                     d_dx[0][0] = 2.0 * x;
//!                     d_dx[1][0] = 1.0;
//!                 }
//!                 if let Some(d_dy) = &mut jacobians[1] {
//!                     d_dy[0][0] = 1.0;
//!                     d_dy[1][0] = 2.0 * y;
//!                 }
//!             }
//!             true
//!         },
//!     );
//!
//!     let solution = NllsProblem::new()
//!         .residual_block_builder() // create a builder for residual block
//!         .set_cost(cost, 2) // 2 is the number of residuals
//!         .set_parameters([x_block, y_block])
//!         .build_into_problem()
//!         .unwrap()
//!         .0 // build_into_problem returns a tuple (NllsProblem, ResidualBlockId)
//!         .solve(&SolverOptions::default()) // SolverOptions can be customized
//!         .unwrap(); // Err should happen only if we added no residual blocks
//!
//!     // Print the full solver report
//!     println!("{}", solution.summary.full_report());
//!
//!     (solution.parameters[0][0], solution.parameters[1][0])
//! }
//!
//! // The solver converges to the corner of the boundary rectangle.
//! let (x, y) = solve_himmelblau(3.4, -1.0);
//! assert_eq!(UPPER_X, x);
//! assert_eq!(LOWER_Y, y);
//!
//! // The solver converges to the global minimum inside the boundaries.
//! let (x, y) = solve_himmelblau(1.0, 1.0);
//! assert!((3.0 - x).abs() < 1e-8);
//! assert!((2.0 - y).abs() < 1e-8);
//! ```

use crate::cost::CostFunction;
use crate::cost::CostFunctionType;
use crate::error::{NllsProblemError, ParameterBlockStorageError, ResidualBlockBuildingError};
use crate::loss::LossFunction;
use crate::parameter_block::{ParameterBlockOrIndex, ParameterBlockStorage};
use crate::residual_block::{ResidualBlock, ResidualBlockId};
use crate::solver::{SolverOptions, SolverSummary};

use ceres_solver_sys::cxx::UniquePtr;
use ceres_solver_sys::ffi;
use std::pin::Pin;

/// Non-Linear Least Squares problem.
///
/// See [module-level documentation](crate::nlls_problem) building the instance of this type.
pub struct NllsProblem<'cost> {
    inner: UniquePtr<ffi::Problem<'cost>>,
    parameter_storage: ParameterBlockStorage,
    residual_blocks: Vec<ResidualBlock>,
}

impl<'cost> NllsProblem<'cost> {
    /// Crate a new non-linear least squares problem with no residual blocks.
    pub fn new() -> Self {
        Self {
            inner: ffi::new_problem(),
            parameter_storage: ParameterBlockStorage::new(),
            residual_blocks: Vec::new(),
        }
    }

    /// Capture this problem into a builder for a new residual block.
    pub fn residual_block_builder(self) -> ResidualBlockBuilder<'cost> {
        ResidualBlockBuilder {
            problem: self,
            cost: None,
            loss: None,
            parameters: Vec::new(),
        }
    }

    #[inline]
    fn inner(&self) -> &ffi::Problem<'cost> {
        self.inner
            .as_ref()
            .expect("Underlying C++ unique_ptr<Problem> must hold non-null pointer")
    }

    #[inline]
    fn inner_mut(&mut self) -> Pin<&mut ffi::Problem<'cost>> {
        self.inner
            .as_mut()
            .expect("Underlying C++ unique_ptr<Problem> must hold non-null pointer")
    }

    /// Set parameter block to be constant during the optimization. Parameter block must be already
    /// added to the problem, otherwise [ParameterBlockStorageError] returned.
    pub fn set_parameter_block_constant(
        &mut self,
        block_index: usize,
    ) -> Result<(), ParameterBlockStorageError> {
        let block_pointer = self.parameter_storage.get_block(block_index)?.pointer_mut();
        unsafe {
            self.inner_mut().SetParameterBlockConstant(block_pointer);
        }
        Ok(())
    }

    /// Set parameter block to be variable during the optimization. Parameter block must be already
    /// added to the problem, otherwise [ParameterBlockStorageError] returned.
    pub fn set_parameter_block_variable(
        &mut self,
        block_index: usize,
    ) -> Result<(), ParameterBlockStorageError> {
        let block_pointer = self.parameter_storage.get_block(block_index)?.pointer_mut();
        unsafe {
            self.inner_mut().SetParameterBlockVariable(block_pointer);
        }
        Ok(())
    }

    /// Check if parameter block is constant. Parameter block must be already added to the problem,
    /// otherwise [ParameterBlockStorageError] returned.
    pub fn is_parameter_block_constant(
        &self,
        block_index: usize,
    ) -> Result<bool, ParameterBlockStorageError> {
        let block_pointer = self.parameter_storage.get_block(block_index)?.pointer_mut();
        unsafe { Ok(self.inner().IsParameterBlockConstant(block_pointer)) }
    }

    /// Solve the problem.
    pub fn solve(
        mut self,
        options: &SolverOptions,
    ) -> Result<NllsProblemSolution, NllsProblemError> {
        if self.residual_blocks.is_empty() {
            return Err(NllsProblemError::NoResidualBlocks);
        }
        let mut summary = SolverSummary::new();
        ffi::solve(
            options
                .0
                .as_ref()
                .expect("Underlying C++ SolverOptions must hold non-null pointer"),
            self.inner_mut(),
            summary
                .0
                .as_mut()
                .expect("Underlying C++ unique_ptr<SolverSummary> must hold non-null pointer"),
        );
        Ok(NllsProblemSolution {
            parameters: self.parameter_storage.to_values(),
            summary,
        })
    }
}

impl Default for NllsProblem<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Solution of a non-linear least squares problem [NllsProblem].
pub struct NllsProblemSolution {
    /// Values of the parameters, in the same order as they were added to the problem.
    pub parameters: Vec<Vec<f64>>,
    /// Summary of the solver run.
    pub summary: SolverSummary,
}

/// Builder for a new residual block. It captures [NllsProblem] and returns it back with
/// [ResidualBlockBuilder::build_into_problem] call.
pub struct ResidualBlockBuilder<'cost> {
    problem: NllsProblem<'cost>,
    cost: Option<(CostFunctionType<'cost>, usize)>,
    loss: Option<LossFunction>,
    parameters: Vec<ParameterBlockOrIndex>,
}

impl<'cost> ResidualBlockBuilder<'cost> {
    /// Set cost function for the residual block.
    ///
    /// Arguments:
    /// * `func` - cost function, see [CostFunction] for details on how to implement it,
    /// * `num_residuals` - number of residuals, typically the same as the number of experiments.
    pub fn set_cost(
        mut self,
        func: impl Into<CostFunctionType<'cost>>,
        num_residuals: usize,
    ) -> Self {
        self.cost = Some((func.into(), num_residuals));
        self
    }

    /// Set loss function for the residual block.
    pub fn set_loss(mut self, loss: LossFunction) -> Self {
        self.loss = Some(loss);
        self
    }

    /// Set parameters for the residual block.
    ///
    /// The argument is an iterator over [ParameterBlockOrIndex] which can be either a new parameter
    /// block or an index of an existing parameter block.
    pub fn set_parameters<P>(mut self, parameters: impl IntoIterator<Item = P>) -> Self
    where
        P: Into<ParameterBlockOrIndex>,
    {
        self.parameters = parameters.into_iter().map(|p| p.into()).collect();
        self
    }

    /// Add a new parameter block to the residual block.
    ///
    /// The argument is either a new parameter block or an index of an existing parameter block.
    pub fn add_parameter<P>(mut self, parameter_block: P) -> Self
    where
        P: Into<ParameterBlockOrIndex>,
    {
        self.parameters.push(parameter_block.into());
        self
    }

    /// Build the residual block, add to the problem and return the problem back.
    ///
    /// Returns [ResidualBlockBuildingError] if:
    /// * cost function is not set,
    /// * no parameters are set,
    /// * any of the parameters is not a new parameter block or an index of an existing parameter.
    ///
    /// Otherwise returns the problem and the residual block id.
    pub fn build_into_problem(
        self,
    ) -> Result<(NllsProblem<'cost>, ResidualBlockId), ResidualBlockBuildingError> {
        let Self {
            mut problem,
            cost,
            loss,
            parameters,
        } = self;
        if parameters.is_empty() {
            return Err(ResidualBlockBuildingError::MissingParameters);
        }
        let parameter_indices = problem.parameter_storage.extend(parameters)?;
        let parameter_sizes: Vec<_> = parameter_indices
            .iter()
            // At this point we know that all parameter indices are valid.
            .map(|&index| problem.parameter_storage.blocks()[index].len())
            .collect();
        let parameter_pointers: Pin<Vec<_>> = Pin::new(
            parameter_indices
                .iter()
                // At this point we know that all parameter indices are valid.
                .map(|&index| problem.parameter_storage.blocks()[index].pointer_mut())
                .collect(),
        );

        // Create cost function
        let cost = if let Some((func, num_redisuals)) = cost {
            CostFunction::new(func, parameter_sizes, num_redisuals)
        } else {
            return Err(ResidualBlockBuildingError::MissingCost);
        };

        // Set residual block
        let residual_block_id = unsafe {
            ffi::add_residual_block(
                problem
                    .inner
                    .as_mut()
                    .expect("Underlying C++ unique_ptr<Problem> must hold non-null pointer"),
                cost.into_inner(),
                loss.map(|loss| loss.into_inner())
                    .unwrap_or_else(UniquePtr::null),
                parameter_pointers.as_ptr(),
                parameter_indices.len() as i32,
            )
        };
        problem.residual_blocks.push(ResidualBlock {
            id: residual_block_id.clone(),
            parameter_pointers,
        });

        // Set parameter bounds
        for &index in parameter_indices.iter() {
            let block = &problem.parameter_storage.blocks()[index];
            if let Some(lower_bound) = block.lower_bounds() {
                for (i, lower_bound) in lower_bound.iter().enumerate() {
                    if let Some(lower_bound) = lower_bound {
                        unsafe {
                            problem
                                .inner
                                .as_mut()
                                .expect(
                                    "Underlying C++ unique_ptr<Problem> must hold non-null pointer",
                                )
                                .SetParameterLowerBound(block.pointer_mut(), i as i32, *lower_bound)
                        }
                    }
                }
            }
        }
        for &index in parameter_indices.iter() {
            let block = &problem.parameter_storage.blocks()[index];
            if let Some(upper_bound) = block.upper_bounds() {
                for (i, upper_bound) in upper_bound.iter().enumerate() {
                    if let Some(upper_bound) = upper_bound {
                        unsafe {
                            problem
                                .inner
                                .as_mut()
                                .expect(
                                    "Underlying C++ unique_ptr<Problem> must hold non-null pointer",
                                )
                                .SetParameterUpperBound(block.pointer_mut(), i as i32, *upper_bound)
                        }
                    }
                }
            }
        }

        Ok((problem, residual_block_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cost::CostFunctionType;
    use crate::loss::{LossFunction, LossFunctionType};

    use approx::assert_abs_diff_eq;

    /// Adopted from c_api_tests.cc, ceres-solver version 2.1.0
    fn simple_end_to_end_test_with_loss(loss: LossFunction) {
        const NUM_OBSERVATIONS: usize = 67;
        const NDIM: usize = 2;
        let data: [[f64; NDIM]; NUM_OBSERVATIONS] = [
            0.000000e+00,
            1.133898e+00,
            7.500000e-02,
            1.334902e+00,
            1.500000e-01,
            1.213546e+00,
            2.250000e-01,
            1.252016e+00,
            3.000000e-01,
            1.392265e+00,
            3.750000e-01,
            1.314458e+00,
            4.500000e-01,
            1.472541e+00,
            5.250000e-01,
            1.536218e+00,
            6.000000e-01,
            1.355679e+00,
            6.750000e-01,
            1.463566e+00,
            7.500000e-01,
            1.490201e+00,
            8.250000e-01,
            1.658699e+00,
            9.000000e-01,
            1.067574e+00,
            9.750000e-01,
            1.464629e+00,
            1.050000e+00,
            1.402653e+00,
            1.125000e+00,
            1.713141e+00,
            1.200000e+00,
            1.527021e+00,
            1.275000e+00,
            1.702632e+00,
            1.350000e+00,
            1.423899e+00,
            1.425000e+00,
            1.543078e+00,
            1.500000e+00,
            1.664015e+00,
            1.575000e+00,
            1.732484e+00,
            1.650000e+00,
            1.543296e+00,
            1.725000e+00,
            1.959523e+00,
            1.800000e+00,
            1.685132e+00,
            1.875000e+00,
            1.951791e+00,
            1.950000e+00,
            2.095346e+00,
            2.025000e+00,
            2.361460e+00,
            2.100000e+00,
            2.169119e+00,
            2.175000e+00,
            2.061745e+00,
            2.250000e+00,
            2.178641e+00,
            2.325000e+00,
            2.104346e+00,
            2.400000e+00,
            2.584470e+00,
            2.475000e+00,
            1.914158e+00,
            2.550000e+00,
            2.368375e+00,
            2.625000e+00,
            2.686125e+00,
            2.700000e+00,
            2.712395e+00,
            2.775000e+00,
            2.499511e+00,
            2.850000e+00,
            2.558897e+00,
            2.925000e+00,
            2.309154e+00,
            3.000000e+00,
            2.869503e+00,
            3.075000e+00,
            3.116645e+00,
            3.150000e+00,
            3.094907e+00,
            3.225000e+00,
            2.471759e+00,
            3.300000e+00,
            3.017131e+00,
            3.375000e+00,
            3.232381e+00,
            3.450000e+00,
            2.944596e+00,
            3.525000e+00,
            3.385343e+00,
            3.600000e+00,
            3.199826e+00,
            3.675000e+00,
            3.423039e+00,
            3.750000e+00,
            3.621552e+00,
            3.825000e+00,
            3.559255e+00,
            3.900000e+00,
            3.530713e+00,
            3.975000e+00,
            3.561766e+00,
            4.050000e+00,
            3.544574e+00,
            4.125000e+00,
            3.867945e+00,
            4.200000e+00,
            4.049776e+00,
            4.275000e+00,
            3.885601e+00,
            4.350000e+00,
            4.110505e+00,
            4.425000e+00,
            4.345320e+00,
            4.500000e+00,
            4.161241e+00,
            4.575000e+00,
            4.363407e+00,
            4.650000e+00,
            4.161576e+00,
            4.725000e+00,
            4.619728e+00,
            4.800000e+00,
            4.737410e+00,
            4.875000e+00,
            4.727863e+00,
            4.950000e+00,
            4.669206e+00,
        ]
        .chunks_exact(NDIM)
        .map(|chunk| chunk.try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

        let cost: CostFunctionType = Box::new(move |parameters, residuals, mut jacobians| {
            let m = parameters[0][0];
            let c = parameters[1][0];
            for ((i, row), residual) in data.into_iter().enumerate().zip(residuals.iter_mut()) {
                let x = row[0];
                let y = row[1];
                *residual = y - f64::exp(m * x + c);
                if let Some(jacobians) = jacobians.as_mut() {
                    if let Some(d_dm) = jacobians[0].as_mut() {
                        d_dm[i][0] = -x * f64::exp(m * x + c);
                    }
                    if let Some(d_dc) = jacobians[1].as_mut() {
                        d_dc[i][0] = -f64::exp(m * x + c);
                    }
                }
            }
            true
        });

        let initial_guess = vec![vec![0.0], vec![0.0]];

        let NllsProblemSolution {
            parameters: solution,
            summary,
        } = NllsProblem::new()
            .residual_block_builder()
            .set_cost(cost, NUM_OBSERVATIONS)
            .set_parameters(initial_guess)
            .set_loss(loss)
            .build_into_problem()
            .unwrap()
            .0
            .solve(&SolverOptions::default())
            .unwrap();

        assert!(summary.is_solution_usable());
        println!("{}", summary.full_report());

        let m = solution[0][0];
        let c = solution[1][0];

        assert_abs_diff_eq!(0.3, m, epsilon = 0.02);
        assert_abs_diff_eq!(0.1, c, epsilon = 0.04);
    }

    #[test]
    fn simple_end_to_end_test_trivial_custom_loss() {
        let loss: LossFunctionType = Box::new(|squared_norm: f64, out: &mut [f64; 3]| {
            out[0] = squared_norm;
            out[1] = 1.0;
            out[2] = 0.0;
        });
        simple_end_to_end_test_with_loss(LossFunction::custom(loss));
    }

    #[test]
    fn simple_end_to_end_test_arctan_stock_loss() {
        simple_end_to_end_test_with_loss(LossFunction::arctan(1.0));
    }
}
