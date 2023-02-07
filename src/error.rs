//! Error enums.

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResidualBlockBuildingError {
    #[error("No cost function set for residual block")]
    MissingCost,
    #[error("No parameters set for residual block")]
    MissingParameters,
    #[error(transparent)]
    ParameterBlockStorageError(#[from] ParameterBlockStorageError),
}

#[derive(Debug, Error)]
pub enum ParameterBlockStorageError {
    #[error("Index of ParameterBlock out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },
}

#[derive(Debug, Error)]
pub enum SolverOptionsBuildingError {
    #[error("SolverOptions is invalid: {0}")]
    Invalid(String),
}

/// Error for [crate::curve_fit::CurveFitProblem1DBuilder].
#[derive(Debug, Error)]
pub enum CurveFitProblemBuildError {
    #[error("Data arrays x, y, or inverse_error have different lengths")]
    DataSizesDontMatch,
    #[error("Cost function is missed")]
    FuncMissed,
    #[error("Independent parameter x is missed")]
    XMissed,
    #[error("Dependent parameter y is missed")]
    YMissed,
    #[error("Initial parameters' guess are missed")]
    ParametersMissed,
    #[error("Lower boundary size doesn't match the number of parameters")]
    LowerBoundarySizeMismatch,
    #[error("Upper boundary size doesn't match the number of parameters")]
    UpperBoundarySizeMismatch,
}
