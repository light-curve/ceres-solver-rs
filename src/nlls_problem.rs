use crate::error::Error;
use crate::residual_block::ResidualBlock;

use ceres_solver_sys as sys;
use std::os::raw::c_int;

/// Non-Linear Least Squares problem.
///
/// You use it in three steps:
/// - [NllsProblem::new] creates a new empty instance of the problem.
/// - [NllsProblem::add_residual_block] adds a new [ResidualBlock], each with its own parameters,
/// cost and loss function.
/// - [NllsProblem::solve] solves the problem and returns a vector consists of parameters for each
/// residual block.
///
/// [NllsProblem::solve] call invalidates the [NllsProblem] instance, so calling this method twice
/// would return [Err]. [Err] also is returned if no residual block have been added to the problem.
pub struct NllsProblem<'cost> {
    inner: *mut sys::ceres_problem_t,
    status: ProblemStatus<'cost>,
}

/// [NllsProblem]'s internal representation of [ResidualBlock]
pub struct ProblemBlock<'cost> {
    // We don't use it, but it may be useful in the future
    #[allow(dead_code)]
    id: *mut sys::ceres_residual_block_id_t,
    residual_block: ResidualBlock<'cost>,
}

/// [NllsProblem]'s state
pub enum ProblemStatus<'cost> {
    /// The problem is blank, at least one residual block must be added via
    /// [NllsProblem::add_residual_block].
    Uninitialized,
    /// The problem is ready for [NllsProblem::solve] call, but you may add more residual blocks
    /// via [NllsProblem::add_residual_block].
    Ready { blocks: Vec<ProblemBlock<'cost>> },
    /// The problem is solved, drop this one and create a new.
    Solved,
}

impl<'cost> NllsProblem<'cost> {
    /// A new [NllsProblem] having [ProblemStatus::Uninitialized] state.
    pub fn new() -> Self {
        Self {
            // Safety: C API
            inner: unsafe { sys::ceres_create_problem() },
            status: ProblemStatus::Uninitialized,
        }
    }

    /// Adds a residual block to the problem.
    ///
    /// Returns [Err] if [NllsProblem] has [ProblemStatus::Solved] state.
    pub fn add_residual_block(&mut self, mut block: ResidualBlock<'cost>) -> Result<(), Error> {
        if matches!(self.status, ProblemStatus::Solved) {
            return Err(Error::ProblemAlreadySolved);
        }
        // Safety: C API
        let id = unsafe {
            sys::ceres_problem_add_residual_block(
                self.inner,
                Some(crate::cost::ffi_cost_function),
                block.cost_function.cost_function_data(),
                block.loss_function.as_ref().map(|loss| loss.ffi_function()),
                block
                    .loss_function
                    .as_mut()
                    .map(|loss| loss.ffi_user_data())
                    .unwrap_or(std::ptr::null_mut()),
                block.cost_function.num_residuals() as c_int,
                block.parameters.len() as c_int,
                block.parameters.sizes_c_int_mut().as_mut_ptr(),
                block.parameters.pointers_mut().as_mut_ptr(),
            )
        };
        let block = ProblemBlock {
            id,
            residual_block: block,
        };
        match &mut self.status {
            ProblemStatus::Uninitialized => {
                self.status = ProblemStatus::Ready {
                    blocks: vec![block],
                };
            }
            ProblemStatus::Ready { blocks } => blocks.push(block),
            ProblemStatus::Solved => unreachable!(),
        };
        Ok(())
    }

    /// Solves the problem and returns a vector of solutions, one per residual block. Returns [Err]
    /// if problem doesn't have [ProblemStatus::Ready] state.
    pub fn solve(&mut self) -> Result<Vec<Vec<Vec<f64>>>, Error> {
        match &mut self.status {
            ProblemStatus::Uninitialized => Err(Error::ProblemNotReady),
            ProblemStatus::Ready { blocks } => {
                // SAFETY: C API
                unsafe {
                    sys::ceres_solve(self.inner);
                }
                let mut new_blocks = vec![];
                std::mem::swap(blocks, &mut new_blocks);
                let solution = new_blocks
                    .into_iter()
                    .map(|ProblemBlock { residual_block, .. }| {
                        residual_block.parameters.to_values()
                    })
                    .collect();
                self.status = ProblemStatus::Solved;
                Ok(solution)
            }
            ProblemStatus::Solved => Err(Error::ProblemAlreadySolved),
        }
    }

    /// Returns the status of the problem.
    pub fn status(&self) -> &ProblemStatus {
        &self.status
    }

    /// Returns the number of residual blocks for unsolved problem, zero otherwise.
    pub fn num_blocks(&self) -> usize {
        match &self.status {
            ProblemStatus::Uninitialized => 0,
            ProblemStatus::Ready { blocks } => blocks.len(),
            ProblemStatus::Solved => 0,
        }
    }
}

impl<'cost> Drop for NllsProblem<'cost> {
    /// Calls C destructor.
    fn drop(&mut self) {
        // SAFETY: C API
        unsafe { sys::ceres_free_problem(self.inner) }
    }
}

impl<'cost> Default for NllsProblem<'cost> {
    /// Returns blank instance.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cost::{CostFunction, CostFunctionType};
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

        let parameters = vec![vec![0.0], vec![0.0]];
        let parameter_sizes = [1, 1];

        let mut problem = NllsProblem::new();
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
        let cost_function = CostFunction::new(cost, parameter_sizes, NUM_OBSERVATIONS);
        problem
            .add_residual_block(ResidualBlock::new(parameters, cost_function).set_loss(loss))
            .unwrap();

        let solution = problem.solve().unwrap();
        let m = solution[0][0][0];
        let c = solution[0][1][0];

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
