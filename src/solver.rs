//! Structures for solver configuration and report.

use crate::error::SolverOptionsBuildingError;
use crate::residual_block::ResidualBlockId;

use ceres_solver_sys::cxx::{let_cxx_string, UniquePtr};
use ceres_solver_sys::ffi;
pub use ceres_solver_sys::ffi::{
    DenseLinearAlgebraLibraryType, DoglegType, DumpFormatType, LineSearchDirectionType,
    LineSearchInterpolationType, LineSearchType, LinearSolverType, LoggingType, MinimizerType,
    NonlinearConjugateGradientType, PreconditionerType, SparseLinearAlgebraLibraryType,
    TrustRegionStrategyType, VisibilityClusteringType,
};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;
use std::pin::Pin;

pub struct SolverOptions(pub(crate) UniquePtr<ffi::SolverOptions>);

impl SolverOptions {
    pub fn builder() -> SolverOptionsBuilder {
        SolverOptionsBuilder::new()
    }
}

impl Default for SolverOptions {
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

pub struct SolverOptionsBuilder(pub(crate) UniquePtr<ffi::SolverOptions>);

impl SolverOptionsBuilder {
    pub fn new() -> Self {
        let slf = Self(ffi::new_solver_options());
        // Remove annoying output from ceres
        slf.logging_type(LoggingType::SILENT)
    }

    pub fn build(self) -> Result<SolverOptions, SolverOptionsBuildingError> {
        self.validate()?;
        Ok(SolverOptions(self.0))
    }

    pub fn validate(&self) -> Result<(), SolverOptionsBuildingError> {
        let_cxx_string!(msg = "");
        if self.0.is_valid(msg.as_mut()) {
            Ok(())
        } else {
            Err(SolverOptionsBuildingError::Invalid(
                msg.to_string_lossy().into_owned(),
            ))
        }
    }

    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    fn inner_mut(&mut self) -> Pin<&mut ffi::SolverOptions> {
        self.0
            .as_mut()
            .expect("Underlying C++ unique_ptr<SolverOptions> must not hold nullptr")
    }

    #[inline]
    pub fn minimizer_type(mut self, minimizer_type: MinimizerType) -> Self {
        self.inner_mut().set_minimizer_type(minimizer_type);
        self
    }

    #[inline]
    pub fn line_search_direction_type(
        mut self,
        line_search_direction_type: LineSearchDirectionType,
    ) -> Self {
        self.inner_mut()
            .set_line_search_direction_type(line_search_direction_type);
        self
    }

    #[inline]
    pub fn line_search_type(mut self, line_search_type: LineSearchType) -> Self {
        self.inner_mut().set_line_search_type(line_search_type);
        self
    }

    #[inline]
    pub fn nonlinear_conjugate_gradient_type(
        mut self,
        nonlinear_conjugate_gradient_type: NonlinearConjugateGradientType,
    ) -> Self {
        self.inner_mut()
            .set_nonlinear_conjugate_gradient_type(nonlinear_conjugate_gradient_type);
        self
    }

    #[inline]
    pub fn max_lbfgs_rank(mut self, max_rank: i32) -> Self {
        self.inner_mut().set_max_lbfgs_rank(max_rank);
        self
    }

    #[inline]
    pub fn use_approximate_eigenvalue_bfgs_scaling(mut self, yes: bool) -> Self {
        self.inner_mut()
            .set_use_approximate_eigenvalue_bfgs_scaling(yes);
        self
    }

    #[inline]
    pub fn line_search_interpolation_type(
        mut self,
        line_search_interpolation_type: LineSearchInterpolationType,
    ) -> Self {
        self.inner_mut()
            .set_line_search_interpolation_type(line_search_interpolation_type);
        self
    }

    #[inline]
    pub fn min_line_search_step_size(mut self, step_size: f64) -> Self {
        self.inner_mut().set_min_line_search_step_size(step_size);
        self
    }

    #[inline]
    pub fn line_search_sufficient_function_decrease(mut self, sufficient_decrease: f64) -> Self {
        self.inner_mut()
            .set_line_search_sufficient_function_decrease(sufficient_decrease);
        self
    }

    #[inline]
    pub fn max_line_search_step_contraction(mut self, max_step_contraction: f64) -> Self {
        self.inner_mut()
            .set_max_line_search_step_contraction(max_step_contraction);
        self
    }

    #[inline]
    pub fn min_line_search_step_contraction(mut self, min_step_contraction: f64) -> Self {
        self.inner_mut()
            .set_min_line_search_step_contraction(min_step_contraction);
        self
    }

    #[inline]
    pub fn max_num_line_search_direction_restarts(mut self, max_num_restarts: i32) -> Self {
        self.inner_mut()
            .set_max_num_line_search_direction_restarts(max_num_restarts);
        self
    }

    #[inline]
    pub fn line_search_sufficient_curvature_decrease(
        mut self,
        sufficient_curvature_decrease: f64,
    ) -> Self {
        self.inner_mut()
            .set_line_search_sufficient_curvature_decrease(sufficient_curvature_decrease);
        self
    }

    #[inline]
    pub fn max_line_search_step_expansion(mut self, max_step_expansion: f64) -> Self {
        self.inner_mut()
            .set_max_line_search_step_expansion(max_step_expansion);
        self
    }

    #[inline]
    pub fn trust_region_strategy_type(
        mut self,
        trust_region_strategy_type: TrustRegionStrategyType,
    ) -> Self {
        self.inner_mut()
            .set_trust_region_strategy_type(trust_region_strategy_type);
        self
    }

    #[inline]
    pub fn dogleg_type(mut self, dogleg_type: DoglegType) -> Self {
        self.inner_mut().set_dogleg_type(dogleg_type);
        self
    }

    #[inline]
    pub fn use_nonmonotonic_steps(mut self, yes: bool) -> Self {
        self.inner_mut().set_use_nonmonotonic_steps(yes);
        self
    }

    #[inline]
    pub fn max_consecutive_nonmonotonic_steps(
        mut self,
        max_consecutive_nonmonotonic_steps: i32,
    ) -> Self {
        self.inner_mut()
            .set_max_consecutive_nonmonotonic_steps(max_consecutive_nonmonotonic_steps);
        self
    }

    #[inline]
    pub fn max_num_iterations(mut self, max_num_iterations: i32) -> Self {
        self.inner_mut().set_max_num_iterations(max_num_iterations);
        self
    }

    #[inline]
    pub fn max_solver_time_in_seconds(mut self, max_solver_time_in_seconds: f64) -> Self {
        self.inner_mut()
            .set_max_solver_time_in_seconds(max_solver_time_in_seconds);
        self
    }

    #[inline]
    pub fn num_threads(mut self, num_threads: i32) -> Self {
        self.inner_mut().set_num_threads(num_threads);
        self
    }

    #[inline]
    pub fn initial_trust_region_radius(mut self, initial_trust_region_radius: f64) -> Self {
        self.inner_mut()
            .set_initial_trust_region_radius(initial_trust_region_radius);
        self
    }

    #[inline]
    pub fn max_trust_region_radius(mut self, max_trust_region_radius: f64) -> Self {
        self.inner_mut()
            .set_max_trust_region_radius(max_trust_region_radius);
        self
    }

    #[inline]
    pub fn min_trust_region_radius(mut self, min_trust_region_radius: f64) -> Self {
        self.inner_mut()
            .set_min_trust_region_radius(min_trust_region_radius);
        self
    }

    #[inline]
    pub fn min_relative_decrease(mut self, min_relative_decrease: f64) -> Self {
        self.inner_mut()
            .set_min_relative_decrease(min_relative_decrease);
        self
    }

    #[inline]
    pub fn min_lm_diagonal(mut self, min_lm_diagonal: f64) -> Self {
        self.inner_mut().set_min_lm_diagonal(min_lm_diagonal);
        self
    }

    #[inline]
    pub fn max_lm_diagonal(mut self, max_lm_diagonal: f64) -> Self {
        self.inner_mut().set_max_lm_diagonal(max_lm_diagonal);
        self
    }

    #[inline]
    pub fn max_num_consecutive_invalid_steps(
        mut self,
        max_num_consecutive_invalid_steps: i32,
    ) -> Self {
        self.inner_mut()
            .set_max_num_consecutive_invalid_steps(max_num_consecutive_invalid_steps);
        self
    }

    #[inline]
    pub fn function_tolerance(mut self, function_tolerance: f64) -> Self {
        self.inner_mut().set_function_tolerance(function_tolerance);
        self
    }

    #[inline]
    pub fn gradient_tolerance(mut self, gradient_tolerance: f64) -> Self {
        self.inner_mut().set_gradient_tolerance(gradient_tolerance);
        self
    }

    #[inline]
    pub fn parameter_tolerance(mut self, parameter_tolerance: f64) -> Self {
        self.inner_mut()
            .set_parameter_tolerance(parameter_tolerance);
        self
    }

    #[inline]
    pub fn linear_solver_type(mut self, linear_solver_type: LinearSolverType) -> Self {
        self.inner_mut().set_linear_solver_type(linear_solver_type);
        self
    }

    #[inline]
    pub fn preconditioner_type(mut self, preconditioner_type: PreconditionerType) -> Self {
        self.inner_mut()
            .set_preconditioner_type(preconditioner_type);
        self
    }

    #[inline]
    pub fn visibility_clustering_type(
        mut self,
        visibility_clustering_type: VisibilityClusteringType,
    ) -> Self {
        self.inner_mut()
            .set_visibility_clustering_type(visibility_clustering_type);
        self
    }

    #[inline]
    pub fn residual_blocks_for_subset_preconditioner(
        mut self,
        residual_blocks: &[ResidualBlockId],
    ) -> Self {
        self.inner_mut()
            .set_residual_blocks_for_subset_preconditioner(residual_blocks);
        self
    }

    #[inline]
    pub fn dense_linear_algebra_library_type(
        mut self,
        dense_linear_algebra_library_type: DenseLinearAlgebraLibraryType,
    ) -> Self {
        self.inner_mut()
            .set_dense_linear_algebra_library_type(dense_linear_algebra_library_type);
        self
    }

    #[inline]
    pub fn sparse_linear_algebra_library_type(
        mut self,
        sparse_linear_algebra_library_type: SparseLinearAlgebraLibraryType,
    ) -> Self {
        self.inner_mut()
            .set_sparse_linear_algebra_library_type(sparse_linear_algebra_library_type);
        self
    }

    #[inline]
    pub fn logging_type(mut self, logging_type: LoggingType) -> Self {
        self.inner_mut().set_logging_type(logging_type);
        self
    }

    #[inline]
    pub fn minimizer_progress_to_stdout(mut self, yes: bool) -> Self {
        self.inner_mut().set_minimizer_progress_to_stdout(yes);
        self
    }

    #[inline]
    pub fn trust_region_minimizer_iterations_to_dump(mut self, iterations_to_dump: &[i32]) -> Self {
        self.inner_mut()
            .set_trust_region_minimizer_iterations_to_dump(iterations_to_dump);
        self
    }

    #[inline]
    pub fn trust_region_problem_dump_directory(mut self, directory: impl AsRef<Path>) -> Self {
        let os_string: &OsStr = directory.as_ref().as_ref();
        let bytes: Cow<[u8]>;
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            bytes = os_string.as_bytes().into();
        }
        #[cfg(target_family = "wasm")]
        {
            use std::os::wasi::ffi::OsStrExt;
            bytes = os_string.as_bytes().into();
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            bytes = os_string
                .encode_wide()
                .flat_map(|c| c.to_le_bytes().into_iter())
                .collect::<Vec<_>>()
                .into();
        }
        let_cxx_string!(cxx_string = bytes);
        self.inner_mut()
            .set_trust_region_problem_dump_directory(cxx_string.into_ref());
        self
    }

    #[inline]
    pub fn trust_region_problem_dump_format_type(
        mut self,
        trust_region_problem_dump_format_type: DumpFormatType,
    ) -> Self {
        self.inner_mut()
            .set_trust_region_problem_dump_format_type(trust_region_problem_dump_format_type);
        self
    }

    #[inline]
    pub fn check_gradients(mut self, yes: bool) -> Self {
        self.inner_mut().set_check_gradients(yes);
        self
    }

    #[inline]
    pub fn gradient_check_relative_precision(
        mut self,
        gradient_check_relative_precision: f64,
    ) -> Self {
        self.inner_mut()
            .set_gradient_check_relative_precision(gradient_check_relative_precision);
        self
    }

    #[inline]
    pub fn gradient_check_numeric_derivative_relative_step_size(
        mut self,
        gradient_check_numeric_derivative_relative_step_size: f64,
    ) -> Self {
        self.inner_mut()
            .set_gradient_check_numeric_derivative_relative_step_size(
                gradient_check_numeric_derivative_relative_step_size,
            );
        self
    }

    #[inline]
    pub fn update_state_every_iteration(mut self, yes: bool) -> Self {
        self.inner_mut().set_update_state_every_iteration(yes);
        self
    }
}

impl Default for SolverOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SolverSummary(pub(crate) UniquePtr<ffi::SolverSummary>);

impl SolverSummary {
    pub fn new() -> Self {
        Self(ffi::new_solver_summary())
    }

    fn inner(&self) -> &ffi::SolverSummary {
        self.0
            .as_ref()
            .expect("Underlying C++ unique_ptr<SolverSummary> must not hold nullptr")
    }

    pub fn brief_report(&self) -> String {
        self.inner().brief_report().to_string_lossy().into()
    }

    pub fn full_report(&self) -> String {
        self.inner().full_report().to_string_lossy().into()
    }

    #[inline]
    pub fn is_solution_usable(&self) -> bool {
        self.inner().is_solution_usable()
    }

    #[inline]
    pub fn initial_cost(&self) -> f64 {
        self.inner().initial_cost()
    }

    #[inline]
    pub fn final_cost(&self) -> f64 {
        self.inner().final_cost()
    }

    #[inline]
    pub fn fixed_cost(&self) -> f64 {
        self.inner().fixed_cost()
    }

    #[inline]
    pub fn num_successful_steps(&self) -> i32 {
        self.inner().num_successful_steps()
    }

    #[inline]
    pub fn num_unsuccessful_steps(&self) -> i32 {
        self.inner().num_unsuccessful_steps()
    }

    #[inline]
    pub fn num_inner_iteration_steps(&self) -> i32 {
        self.inner().num_inner_iteration_steps()
    }

    #[inline]
    pub fn num_line_search_steps(&self) -> i32 {
        self.inner().num_line_search_steps()
    }
}

impl std::fmt::Debug for SolverSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SolverSummary {{ brief_report: {:?} }}",
            self.brief_report()
        )
    }
}

impl Default for SolverSummary {
    fn default() -> Self {
        Self::new()
    }
}
