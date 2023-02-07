//! Loss functions for [NllsProblem](crate::nlls_problem::NllsProblem) and
//! [CurveFitProblem1D](crate::curve_fit::CurveFitProblem1D).
//!
//! Loss function is a function applied to a squared norm of the problem, it could help in reducing
//! outliers data for better convergence. There are two types of them: ones built from custom
//! functions boxed into [LossFunctionType] and Ceres stock functions having one or two
//! scale parameters.

use ceres_solver_sys::cxx::UniquePtr;
use ceres_solver_sys::ffi;

pub type LossFunctionType = Box<dyn Fn(f64, &mut [f64; 3])>;

/// Loss function for [NllsProblem](crate::nlls_problem::NllsProblem) and
/// [CurveFitProblem1D](crate::curve_fit::CurveFitProblem1D), it is a transformation of the squared
/// residuals which is generally used to make the solver less sensitive to outliers. This enum has
/// two flavours: user specified function and Ceres stock function.
pub struct LossFunction(UniquePtr<ffi::LossFunction>);

impl LossFunction {
    /// Create a [LossFunction] to handle a custom loss function.
    ///
    /// # Arguments
    /// - func - a boxed function which accepts two arguments: non-negative squared residual, and
    ///  an array of 0) loss function value, 1) its first, and 2) its second derivatives. See
    /// details at
    /// <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12LossFunctionE>.
    pub fn custom(func: impl Into<LossFunctionType>) -> Self {
        let safe_func = func.into();
        let rust_func: Box<dyn Fn(f64, *mut f64)> = Box::new(move |sq_norm, out_ptr| {
            let out = unsafe { &mut *(out_ptr as *mut [f64; 3]) };
            safe_func(sq_norm, out);
        });
        let inner = ffi::new_callback_loss_function(Box::new(rust_func.into()));
        Self(inner)
    }

    /// Huber loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres9HuberLossE>.
    pub fn huber(a: f64) -> Self {
        Self(ffi::new_huber_loss(a))
    }

    /// Soft L1 loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12SoftLOneLossE>.
    pub fn soft_l1(a: f64) -> Self {
        Self(ffi::new_soft_l_one_loss(a))
    }

    /// log(1+s) loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres10CauchyLossE>.
    pub fn cauchy(a: f64) -> Self {
        Self(ffi::new_cauchy_loss(a))
    }

    /// Arctangent loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres10ArctanLossE>.
    pub fn arctan(a: f64) -> Self {
        Self(ffi::new_arctan_loss(a))
    }

    /// Tolerant loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12TolerantLossE>.
    pub fn tolerant(a: f64, b: f64) -> Self {
        Self(ffi::new_tolerant_loss(a, b))
    }

    /// Tukey loss function
    pub fn tukey(a: f64) -> Self {
        Self(ffi::new_tukey_loss(a))
    }

    pub fn into_inner(self) -> UniquePtr<ffi::LossFunction> {
        self.0
    }
}
