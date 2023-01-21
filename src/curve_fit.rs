use crate::cost::{CostFunction, CostFunctionType};
use crate::loss::LossFunction;
use crate::nlls_problem::NllsProblem;
use crate::residual_block::ResidualBlock;
use crate::types::Either;

pub type CurveFunctionType = Box<dyn Fn(f64, &[f64], &mut f64, Option<&mut [Option<f64>]>) -> bool>;

/// A wrapper for [NllsProblem] providing easier interface to solve an 1-D muliparameter curve fit
/// problem. Use it in two steps: create a new instance with [CurveFitProblem1D::new] and then
/// call a destructive method [CurveFitProblem1D::to_solution] to get a solution.
pub struct CurveFitProblem1D<'cost>(NllsProblem<'cost>);

impl<'cost> CurveFitProblem1D<'cost> {
    /// Creates a new instance of the `CurveFitProblem1D`.
    ///
    /// # Arguments
    /// - func - a function describing a curve. It must return [false] if it cannot calculate
    /// Jacobian, or [true] otherwise. It accepts the following parameters:
    ///   - x - an independent coordinate.
    ///   - parameters - a slice for the current value of the problem parameters. Note, that unlike
    ///   [NllsProblem] it is a 1-D slice.
    ///   - y - a mutable reference to output the function value.
    ///   - jacobians - an output Jacobian matrix, it (or any of its component) can be [None], which
    ///   means that the solver doesn't need it. Otherwise it has a 2-D shape, the top index
    ///   corresponds to a parameter component, the bottom index corresponds to a data point. So the
    ///   top-level slice inside [Some] has length of `parameters.len()`, while inner slices have
    ///   the same length as `x` and `y`.
    /// - x - independent coordinate values of data poitns.
    /// - y - values of data points.
    /// - parameters - a vector of the initial parameters. Note that unlike [NllsProblem] it is a
    ///   1-D vector of [f64].
    /// - loss - optional loss function.
    ///
    /// # Panics
    /// Panics if `x` and `y` have different sizes.
    pub fn new(
        func: impl Into<CurveFunctionType>,
        x: &'cost [f64],
        y: &'cost [f64],
        parameters: &[f64],
        loss: Option<LossFunction>,
    ) -> Self {
        assert_eq!(x.len(), y.len());
        let nlls_parameters: Vec<_> = parameters.iter().map(|&x| vec![x]).collect();
        let mut problem = NllsProblem::new();
        let block = ResidualBlock::new(
            nlls_parameters,
            Self::cost_function(x, y, None, func.into(), parameters.len()),
        )
        .change_loss(loss);
        problem.add_residual_block(block).unwrap();
        Self(problem)
    }

    /// Create a [CurveFitProblem1DBuilder] instance, see its docs for the details.
    pub fn builder<'param>() -> CurveFitProblem1DBuilder<'cost, 'param> {
        CurveFitProblem1DBuilder::new()
    }

    fn cost_function(
        x: &'cost [f64],
        y: &'cost [f64],
        inv_err: Option<&'cost [f64]>,
        curve_func: CurveFunctionType,
        num_parameters: usize,
    ) -> CostFunction<'cost> {
        let parameter_sizes = vec![1_usize; num_parameters];
        let n_obs = x.len();
        let cost: CostFunctionType = Box::new(move |parameters, residuals, mut jacobians| {
            let mut result = true;
            let mut f = 0.0;
            let mut jac: Option<Vec<Option<f64>>> = jacobians.as_ref().map(|jacobians| {
                jacobians
                    .iter()
                    .map(|der| der.as_ref().map(|_| 0.0))
                    .collect()
            });
            let parameters: Vec<_> = parameters.iter().map(|x| x[0]).collect();
            for ((((i, &x), &y), &inv_err), residual) in (0..n_obs)
                .zip(x.iter())
                .zip(y.iter())
                .zip(match inv_err {
                    Some(inv_err) => Either::Left(inv_err.iter()),
                    None => Either::Right(std::iter::repeat(&1.0)),
                })
                .zip(residuals.iter_mut())
            {
                result = curve_func(x, &parameters, &mut f, jac.as_mut().map(|d| &mut d[..]));
                *residual = inv_err * (y - f);
                if let Some(jacobians) = jacobians.as_mut() {
                    for (d_in, d_out) in jac.as_ref().unwrap().iter().zip(jacobians.iter_mut()) {
                        if let Some(d_out) = d_out.as_mut() {
                            d_out[i][0] = -inv_err * d_in.unwrap();
                        }
                    }
                }
            }
            result
        });
        CostFunction::new(cost, parameter_sizes, n_obs)
    }

    /// Solves the problem and returns a solution for the parameters.
    pub fn to_solution(mut self) -> Vec<f64> {
        // We know that we have well-defined problem
        let solution = self.0.solve().unwrap();
        // We have a single block
        let first_block_parameters = solution.into_iter().next().unwrap();
        // All parameters are 1D - compress to a single vector
        first_block_parameters.into_iter().map(|x| x[0]).collect()
    }
}

/// Builder for [CurveFitProblem1D].
///
/// # Example
///
/// ```rust
/// use ceres_solver::curve_fit::{CurveFitProblem1D, CurveFunctionType};
/// use ceres_solver::loss::LossFunction;
///
/// // Linear model
/// fn model(
///     x: f64,
///     parameters: &[f64],
///     y: &mut f64,
///     jacobians: Option<&mut [Option<f64>]>,
/// ) -> bool {
///     let &[a, b]: &[f64; 2] = parameters.try_into().unwrap();
///     *y = a * x + b;
///     if let Some(jacobians) = jacobians {
///         let [d_da, d_db]: &mut [Option<f64>; 2] = jacobians.try_into().unwrap();
///         if let Some(d_da) = d_da {
///             *d_da = x;
///         }
///         if let Some(d_db) = d_db {
///             *d_db = 1.0;
///         }
///     }
///     true
/// }
///
/// let a = 3.0;
/// let b = -2.0;
/// let x: Vec<_> = (0..100).map(|i| i as f64).collect();
/// let y: Vec<_> = x.iter().map(|&x| a * x + b).collect();
/// // optional data points inversed errors, assumed to be positive
/// let inversed_error: Vec<_> = x.iter().map(|&x| (x + 1.0) / 100.0).collect();
///
/// let func: CurveFunctionType = Box::new(model);
/// let problem = CurveFitProblem1D::builder()
///     // Model function
///     .func(func)
///     // Initial parameter guess
///     .parameters(&[1.0, 0.0])
///     // Data points, inversed errors are optional, if no given unity errors assumed.
///     .x(&x)
///     .y(&y)
///     .inversed_error(&inversed_error)
///     // Loss function is optional, if not given trivial loss is assumed.
///     .loss(LossFunction::cauchy(1.0))
///     .build()
///     .unwrap();
/// let solution = problem.to_solution();
///
/// assert!(f64::abs(a - solution[0]) < 1e-8);
/// assert!(f64::abs(b - solution[1]) < 1e-8);
/// ```
pub struct CurveFitProblem1DBuilder<'cost, 'param> {
    /// Model function
    pub func: Option<CurveFunctionType>,
    /// Independent coordinates for data
    pub x: Option<&'cost [f64]>,
    /// Values for data
    pub y: Option<&'cost [f64]>,
    /// optional inversed errors - square root of the weight
    pub inversed_error: Option<&'cost [f64]>,
    /// Initial parameters' guess
    pub parameters: Option<&'param [f64]>,
    /// optional loss function
    pub loss: Option<LossFunction>,
}

impl<'cost, 'param> CurveFitProblem1DBuilder<'cost, 'param> {
    pub fn new() -> Self {
        Self {
            func: None,
            x: None,
            y: None,
            inversed_error: None,
            parameters: None,
            loss: None,
        }
    }

    /// Add model function.
    pub fn func(mut self, func: impl Into<CurveFunctionType>) -> Self {
        self.func = Some(func.into());
        self
    }

    /// Add independent parameter values for the data points.
    pub fn x(mut self, x: &'cost [f64]) -> Self {
        self.x = Some(x);
        self
    }

    /// Add values for the data points.
    pub fn y(mut self, y: &'cost [f64]) -> Self {
        self.y = Some(y);
        self
    }

    /// Add optional inversed errors for the data points. They must to be positive: think about them
    /// as the inverse y's uncertainties, or square root of the data point weight. The residual
    /// would be `(y - model(x)) * inversed_error`. If not given, unity valueas are assumed.
    pub fn inversed_error(mut self, inv_err: &'cost [f64]) -> Self {
        self.inversed_error = Some(inv_err);
        self
    }

    /// Add initial parameter guess slice, it is borrowed until [CurveFitProblem1DBuilder::build()]
    /// call only, there it will be copied to the [CurveFitProblem1D] instance.
    pub fn parameters(mut self, parameters: &'param [f64]) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// Add optional loss function, if not given the trivial loss is assumed.
    pub fn loss(mut self, loss: LossFunction) -> Self {
        self.loss = Some(loss);
        self
    }

    /// Build the [CurveFitProblem1D] instance. Returns [Err] if one of the mandatory fields is
    /// missed or data slices have inconsistent lengths.
    pub fn build(self) -> Result<CurveFitProblem1D<'cost>, CurveFitProblemBuildError> {
        let func = self.func.ok_or(CurveFitProblemBuildError::MissingFunc)?;
        let x = self.x.ok_or(CurveFitProblemBuildError::MissingX)?;
        let y = self.y.ok_or(CurveFitProblemBuildError::MissingY)?;
        if x.len() != y.len() {
            return Err(CurveFitProblemBuildError::DataSizesDontMatch);
        }
        if let Some(inversed_error) = self.inversed_error {
            if inversed_error.len() != x.len() {
                return Err(CurveFitProblemBuildError::DataSizesDontMatch);
            }
        }
        let nlls_parameters: Vec<Vec<f64>> = self
            .parameters
            .ok_or(CurveFitProblemBuildError::MissedParameters)?
            .iter()
            .map(|&p| vec![p])
            .collect();
        let n_param = nlls_parameters.len();
        let mut problem = NllsProblem::new();
        let block = ResidualBlock::new(
            nlls_parameters,
            CurveFitProblem1D::cost_function(x, y, self.inversed_error, func, n_param),
        )
        .change_loss(self.loss);
        problem.add_residual_block(block).unwrap();
        Ok(CurveFitProblem1D(problem))
    }
}

impl<'cost, 'param> Default for CurveFitProblem1DBuilder<'cost, 'param> {
    fn default() -> Self {
        Self::new()
    }
}

/// Error for [CurveFitProblem1DBuilder].
#[derive(Debug)]
#[non_exhaustive]
pub enum CurveFitProblemBuildError {
    DataSizesDontMatch,
    MissingFunc,
    MissingX,
    MissingY,
    MissedParameters,
}

impl std::fmt::Display for CurveFitProblemBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::DataSizesDontMatch => "data arrays have different lengths",
            Self::MissingFunc => "function is missing",
            Self::MissingX => "x is missing",
            Self::MissingY => "y is missing",
            Self::MissedParameters => "initial parameters' guess are missing",
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for CurveFitProblemBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::LossFunctionType;

    use approx::assert_abs_diff_eq;
    use rand::{Rng, SeedableRng};

    fn curve_fit_problem_1d(loss: Option<LossFunction>) -> Vec<f64> {
        let (x, y): (Vec<_>, Vec<_>) = [
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
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .unzip();

        let func: CurveFunctionType = Box::new(
            |x: f64, parameters: &[f64], value: &mut f64, jac: Option<&mut [Option<f64>]>| {
                let m = parameters[0];
                let c = parameters[1];
                *value = f64::exp(m * x + c);
                if let Some(jac) = jac {
                    if let Some(d_dm) = jac[0].as_mut() {
                        *d_dm = x * f64::exp(m * x + c);
                    }
                    if let Some(d_dc) = jac[1].as_mut() {
                        *d_dc = f64::exp(m * x + c);
                    }
                }
                true
            },
        );
        let problem = CurveFitProblem1D::new(func, &x, &y, &[0.0, 0.0], loss);
        let solution = problem.to_solution();

        assert_abs_diff_eq!(0.3, solution[0], epsilon = 0.02);
        assert_abs_diff_eq!(0.1, solution[1], epsilon = 0.04);

        solution
    }

    #[test]
    fn test_curve_fit_problem_1d_trivial_loss() {
        curve_fit_problem_1d(None);
    }

    #[test]
    fn test_curve_fit_problem_1d_custom_arctan_loss() {
        let loss: LossFunctionType = Box::new(|squared_norm, out| {
            out[0] = f64::atan(squared_norm);
            out[1] = 1.0 / (squared_norm.powi(2) + 1.0);
            out[2] = -2.0 * squared_norm * out[1].powi(2);
        });
        let loss = LossFunction::custom(loss);
        curve_fit_problem_1d(Some(loss));
    }

    #[test]
    fn test_curve_fit_problem_2d_stock_arctan_loss() {
        let loss = LossFunction::arctan(1.0);
        curve_fit_problem_1d(Some(loss));
    }

    /// y = a * sin (b * x) + c
    fn model(
        x: f64,
        parameters: &[f64],
        y: &mut f64,
        jacobians: Option<&mut [Option<f64>]>,
    ) -> bool {
        let &[a, b, c]: &[f64; 3] = parameters.try_into().unwrap();
        *y = a * f64::sin(b * x) + c;
        if let Some(jacobians) = jacobians {
            let [d_da, d_db, d_dc]: &mut [Option<f64>; 3] = jacobians.try_into().unwrap();
            if let Some(d_da) = d_da {
                *d_da = f64::sin(b * x);
            }
            if let Some(d_db) = d_db {
                *d_db = a * b * f64::cos(b * x);
            }
            if let Some(d_dc) = d_dc {
                *d_dc = 1.0;
            }
        }
        true
    }

    #[test]
    fn compare_new_with_build() {
        const N: usize = 1000;

        const TRUE_PARAM: [f64; 3] = [1.5, std::f64::consts::PI, -1.0];

        let x: Vec<_> = (0..N).map(|i| i as f64 / N as f64).collect();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        let noise_level: f64 = 0.1;
        let y: Vec<_> = x
            .iter()
            .map(|&x| {
                let mut y = 0.0;
                model(x, &TRUE_PARAM, &mut y, None);
                let sigma = noise_level * rng.sample::<f64, _>(rand_distr::StandardNormal);
                y + sigma
            })
            .collect();
        let w = vec![noise_level.powi(-1); x.len()];

        let initial_guess = [0.0, 1.0, 0.0];

        let func: CurveFunctionType = Box::new(model);
        let solution_new = CurveFitProblem1D::new(func, &x, &y, &initial_guess, None).to_solution();

        let func: CurveFunctionType = Box::new(model);
        let solution_build = CurveFitProblem1D::builder()
            .func(func)
            .x(&x)
            .y(&y)
            .inversed_error(&w)
            .parameters(&initial_guess)
            .build()
            .unwrap()
            .to_solution();

        assert_abs_diff_eq!(&solution_new[..], &solution_build[..], epsilon = 1e-10);
        assert_abs_diff_eq!(&TRUE_PARAM[..], &solution_new[..], epsilon = 0.02);
    }
}
