pub use cxx;

#[cxx::bridge(namespace = "ceres")]
pub mod ffi {
    // The explicit lifetimes make some signatures more verbose.
    #![allow(clippy::needless_lifetimes)]
    // False positive https://github.com/rust-lang/rust-clippy/issues/13360
    #![allow(clippy::needless_maybe_sized)]
    // False positive, I believe
    #![allow(clippy::missing_safety_doc)]

    #[repr(u32)]
    enum MinimizerType {
        LINE_SEARCH,
        TRUST_REGION,
    }

    #[repr(u32)]
    enum LineSearchDirectionType {
        STEEPEST_DESCENT,
        NONLINEAR_CONJUGATE_GRADIENT,
        LBFGS,
        BFGS,
    }

    #[repr(u32)]
    enum LineSearchType {
        ARMIJO,
        WOLFE,
    }

    #[repr(u32)]
    enum NonlinearConjugateGradientType {
        FLETCHER_REEVES,
        POLAK_RIBIERE,
        HESTENES_STIEFEL,
    }

    #[repr(u32)]
    enum LineSearchInterpolationType {
        BISECTION,
        QUADRATIC,
        CUBIC,
    }

    #[repr(u32)]
    enum TrustRegionStrategyType {
        LEVENBERG_MARQUARDT,
        DOGLEG,
    }

    #[repr(u32)]
    enum DoglegType {
        TRADITIONAL_DOGLEG,
        SUBSPACE_DOGLEG,
    }

    #[repr(u32)]
    enum LinearSolverType {
        DENSE_NORMAL_CHOLESKY,
        DENSE_QR,
        SPARSE_NORMAL_CHOLESKY,
        DENSE_SCHUR,
        SPARSE_SCHUR,
        ITERATIVE_SCHUR,
        CGNR,
    }

    #[repr(u32)]
    enum PreconditionerType {
        IDENTITY,
        JACOBI,
        SCHUR_JACOBI,
        SCHUR_POWER_SERIES_EXPANSION,
        CLUSTER_JACOBI,
        CLUSTER_TRIDIAGONAL,
        SUBSET,
    }

    #[repr(u32)]
    enum VisibilityClusteringType {
        CANONICAL_VIEWS,
        SINGLE_LINKAGE,
    }

    #[repr(u32)]
    enum DenseLinearAlgebraLibraryType {
        EIGEN,
        LAPACK,
        CUDA,
    }

    #[repr(u32)]
    enum SparseLinearAlgebraLibraryType {
        SUITE_SPARSE,
        EIGEN_SPARSE,
        ACCELERATE_SPARSE,
        CUDA_SPARSE,
        NO_SPARSE,
    }

    #[repr(u32)]
    enum LoggingType {
        SILENT,
        PER_MINIMIZER_ITERATION,
    }

    #[repr(u32)]
    enum DumpFormatType {
        CONSOLE,
        TEXTFILE,
    }

    extern "Rust" {
        type RustCostFunction<'cost>;
        unsafe fn evaluate(
            self: &RustCostFunction,
            parameters: *const *const f64,
            residuals: *mut f64,
            jacobians: *mut *mut f64,
        ) -> bool;

        type RustLossFunction;
        unsafe fn evaluate(self: &RustLossFunction, sq_norm: f64, out: *mut f64);
    }

    unsafe extern "C++" {
        include!("ceres-solver-sys/src/lib.h");

        type MinimizerType;
        type LineSearchDirectionType;
        type LineSearchType;
        type NonlinearConjugateGradientType;
        type LineSearchInterpolationType;
        type TrustRegionStrategyType;
        type DoglegType;
        type LinearSolverType;
        type PreconditionerType;
        type VisibilityClusteringType;
        type DenseLinearAlgebraLibraryType;
        type SparseLinearAlgebraLibraryType;
        type LoggingType;
        type DumpFormatType;

        type CallbackCostFunction<'cost>;
        /// Creates new C++ cost function from Rust cost function;
        fn new_callback_cost_function<'cost>(
            inner: Box<RustCostFunction<'cost>>,
            num_residuals: i32,
            parameter_block_sizes: &[i32],
        ) -> UniquePtr<CallbackCostFunction<'cost>>;

        type LossFunction;
        /// Creates new C++ loss function from Rust loss function;
        fn new_callback_loss_function(inner: Box<RustLossFunction>) -> UniquePtr<LossFunction>;
        /// Creates stock TrivialLoss.
        fn new_trivial_loss() -> UniquePtr<LossFunction>;
        /// Creates stock HuberLoss.
        fn new_huber_loss(a: f64) -> UniquePtr<LossFunction>;
        /// Creates stock SoftLOneLoss.
        fn new_soft_l_one_loss(a: f64) -> UniquePtr<LossFunction>;
        /// Creates stock CauchyLoss.
        fn new_cauchy_loss(a: f64) -> UniquePtr<LossFunction>;
        /// Creates stock ArctanLoss.
        fn new_arctan_loss(a: f64) -> UniquePtr<LossFunction>;
        /// Creates stock TolerantLoss.
        fn new_tolerant_loss(a: f64, b: f64) -> UniquePtr<LossFunction>;
        /// Creates stock TukeyLoss.
        fn new_tukey_loss(a: f64) -> UniquePtr<LossFunction>;

        type ResidualBlockId;

        type Problem<'cost>;
        /// Set parameter to be constant.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn SetParameterBlockConstant(self: Pin<&mut Problem>, values: *const f64);
        /// Set parameter to vary.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn SetParameterBlockVariable(self: Pin<&mut Problem>, values: *mut f64);
        /// Check if parameter is constant.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn IsParameterBlockConstant(self: &Problem, values: *const f64) -> bool;
        /// Set lower bound for a component of a parameter block.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn SetParameterLowerBound(
            self: Pin<&mut Problem>,
            values: *mut f64,
            index: i32,
            lower_bound: f64,
        );
        /// Set upper bound for a component of a parameter block.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn SetParameterUpperBound(
            self: Pin<&mut Problem>,
            values: *mut f64,
            index: i32,
            upper_bound: f64,
        );
        fn NumParameterBlocks(self: &Problem) -> i32;
        fn NumParameters(self: &Problem) -> i32;
        fn NumResidualBlocks(self: &Problem) -> i32;
        fn NumResiduals(self: &Problem) -> i32;
        /// Number of components of the parameter.
        ///
        /// # Safety
        /// `values` must point to already added parameter block.
        unsafe fn ParameterBlockSize(self: &Problem, values: *const f64) -> i32;
        /// Checks if problem has a given parameter.
        ///
        /// # Safety
        /// It should be safe to call this function with any pointer.
        unsafe fn HasParameterBlock(self: &Problem, values: *const f64) -> bool;
        /// Creates new Problem.
        fn new_problem<'cost>() -> UniquePtr<Problem<'cost>>;
        /// Adds a residual block to the problem.
        ///
        /// # Safety
        /// `parameter_blocks` must outlive `problem`.
        unsafe fn add_residual_block<'cost>(
            problem: Pin<&mut Problem<'cost>>,
            cost_function: UniquePtr<CallbackCostFunction<'cost>>,
            loss_function: UniquePtr<LossFunction>,
            parameter_blocks: *const *mut f64,
            num_parameter_blocks: i32,
        ) -> SharedPtr<ResidualBlockId>;

        type SolverOptions;
        fn is_valid(self: &SolverOptions, error: Pin<&mut CxxString>) -> bool;
        fn set_minimizer_type(self: Pin<&mut SolverOptions>, minimizer_type: MinimizerType);
        fn set_line_search_direction_type(
            self: Pin<&mut SolverOptions>,
            line_search_direction_type: LineSearchDirectionType,
        );
        fn set_line_search_type(self: Pin<&mut SolverOptions>, line_search_type: LineSearchType);
        fn set_nonlinear_conjugate_gradient_type(
            self: Pin<&mut SolverOptions>,
            nonlinear_conjugate_gradient_type: NonlinearConjugateGradientType,
        );
        fn set_max_lbfgs_rank(self: Pin<&mut SolverOptions>, max_rank: i32);
        fn set_use_approximate_eigenvalue_bfgs_scaling(self: Pin<&mut SolverOptions>, yes: bool);
        fn set_line_search_interpolation_type(
            self: Pin<&mut SolverOptions>,
            line_search_interpolation_type: LineSearchInterpolationType,
        );
        fn set_min_line_search_step_size(self: Pin<&mut SolverOptions>, step_size: f64);
        fn set_line_search_sufficient_function_decrease(
            self: Pin<&mut SolverOptions>,
            sufficient_decrease: f64,
        );
        fn set_max_line_search_step_contraction(
            self: Pin<&mut SolverOptions>,
            max_step_contraction: f64,
        );
        fn set_min_line_search_step_contraction(
            self: Pin<&mut SolverOptions>,
            min_step_contraction: f64,
        );
        fn set_max_num_line_search_direction_restarts(
            self: Pin<&mut SolverOptions>,
            max_num_restarts: i32,
        );
        fn set_line_search_sufficient_curvature_decrease(
            self: Pin<&mut SolverOptions>,
            sufficient_curvature_decrease: f64,
        );
        fn set_max_line_search_step_expansion(
            self: Pin<&mut SolverOptions>,
            max_step_expansion: f64,
        );
        fn set_trust_region_strategy_type(
            self: Pin<&mut SolverOptions>,
            trust_region_strategy_type: TrustRegionStrategyType,
        );
        fn set_dogleg_type(self: Pin<&mut SolverOptions>, dogleg_type: DoglegType);
        fn set_use_nonmonotonic_steps(self: Pin<&mut SolverOptions>, yes: bool);
        fn set_max_consecutive_nonmonotonic_steps(
            self: Pin<&mut SolverOptions>,
            max_consecutive_nonmonotonic_steps: i32,
        );
        fn set_max_num_iterations(self: Pin<&mut SolverOptions>, max_num_iterations: i32);
        fn set_max_solver_time_in_seconds(
            self: Pin<&mut SolverOptions>,
            max_solver_time_in_seconds: f64,
        );
        fn set_num_threads(self: Pin<&mut SolverOptions>, num_threads: i32);
        fn set_initial_trust_region_radius(
            self: Pin<&mut SolverOptions>,
            initial_trust_region_radius: f64,
        );
        fn set_max_trust_region_radius(self: Pin<&mut SolverOptions>, max_trust_region_radius: f64);
        fn set_min_trust_region_radius(self: Pin<&mut SolverOptions>, min_trust_region_radius: f64);
        fn set_min_relative_decrease(self: Pin<&mut SolverOptions>, min_relative_decrease: f64);
        fn set_min_lm_diagonal(self: Pin<&mut SolverOptions>, min_lm_diagonal: f64);
        fn set_max_lm_diagonal(self: Pin<&mut SolverOptions>, max_lm_diagonal: f64);
        fn set_max_num_consecutive_invalid_steps(
            self: Pin<&mut SolverOptions>,
            max_num_consecutive_invalid_steps: i32,
        );
        fn set_function_tolerance(self: Pin<&mut SolverOptions>, function_tolerance: f64);
        fn set_gradient_tolerance(self: Pin<&mut SolverOptions>, gradient_tolerance: f64);
        fn set_parameter_tolerance(self: Pin<&mut SolverOptions>, parameter_tolerance: f64);
        fn set_linear_solver_type(
            self: Pin<&mut SolverOptions>,
            linear_solver_type: LinearSolverType,
        );
        fn set_preconditioner_type(
            self: Pin<&mut SolverOptions>,
            preconditioner_type: PreconditionerType,
        );
        fn set_visibility_clustering_type(
            self: Pin<&mut SolverOptions>,
            visibility_clustering_type: VisibilityClusteringType,
        );
        fn set_residual_blocks_for_subset_preconditioner(
            self: Pin<&mut SolverOptions>,
            residual_blocks: &[SharedPtr<ResidualBlockId>],
        );
        fn set_dense_linear_algebra_library_type(
            self: Pin<&mut SolverOptions>,
            dense_linear_algebra_library_type: DenseLinearAlgebraLibraryType,
        );
        fn set_sparse_linear_algebra_library_type(
            self: Pin<&mut SolverOptions>,
            sparse_linear_algebra_library_type: SparseLinearAlgebraLibraryType,
        );
        fn set_logging_type(self: Pin<&mut SolverOptions>, logging_type: LoggingType);
        fn set_minimizer_progress_to_stdout(self: Pin<&mut SolverOptions>, yes: bool);
        fn set_trust_region_minimizer_iterations_to_dump(
            self: Pin<&mut SolverOptions>,
            iterations_to_dump: &[i32],
        );
        fn set_trust_region_problem_dump_directory(
            self: Pin<&mut SolverOptions>,
            directory: Pin<&CxxString>,
        );
        fn set_trust_region_problem_dump_format_type(
            self: Pin<&mut SolverOptions>,
            trust_region_problem_dump_format_type: DumpFormatType,
        );
        fn set_check_gradients(self: Pin<&mut SolverOptions>, yes: bool);
        fn set_gradient_check_relative_precision(
            self: Pin<&mut SolverOptions>,
            gradient_check_relative_precision: f64,
        );
        fn set_gradient_check_numeric_derivative_relative_step_size(
            self: Pin<&mut SolverOptions>,
            gradient_check_numeric_derivative_relative_step_size: f64,
        );
        fn set_update_state_every_iteration(self: Pin<&mut SolverOptions>, yes: bool);

        /// Create an instance wrapping Solver::Options.
        fn new_solver_options() -> UniquePtr<SolverOptions>;

        type SolverSummary;
        fn brief_report(self: &SolverSummary) -> UniquePtr<CxxString>;
        fn full_report(self: &SolverSummary) -> UniquePtr<CxxString>;
        fn is_solution_usable(self: &SolverSummary) -> bool;
        fn initial_cost(self: &SolverSummary) -> f64;
        fn final_cost(self: &SolverSummary) -> f64;
        fn fixed_cost(self: &SolverSummary) -> f64;
        fn num_successful_steps(self: &SolverSummary) -> i32;
        fn num_unsuccessful_steps(self: &SolverSummary) -> i32;
        fn num_inner_iteration_steps(self: &SolverSummary) -> i32;
        fn num_line_search_steps(self: &SolverSummary) -> i32;
        /// Create an instance wrapping Solver::Summary.
        fn new_solver_summary() -> UniquePtr<SolverSummary>;

        /// Wrapper for Solve() function.
        fn solve(
            options: &SolverOptions,
            problem: Pin<&mut Problem>,
            summary: Pin<&mut SolverSummary>,
        );
    }
}

pub struct RustCostFunction<'cost>(
    pub Box<dyn Fn(*const *const f64, *mut f64, *mut *mut f64) -> bool + 'cost>,
);

impl RustCostFunction<'_> {
    pub fn evaluate(
        &self,
        parameters: *const *const f64,
        residuals: *mut f64,
        jacobians: *mut *mut f64,
    ) -> bool {
        (self.0)(parameters, residuals, jacobians)
    }
}

impl<'cost> From<Box<dyn Fn(*const *const f64, *mut f64, *mut *mut f64) -> bool + 'cost>>
    for RustCostFunction<'cost>
{
    fn from(
        value: Box<dyn Fn(*const *const f64, *mut f64, *mut *mut f64) -> bool + 'cost>,
    ) -> Self {
        Self(value)
    }
}

pub struct RustLossFunction(pub Box<dyn Fn(f64, *mut f64)>);

impl RustLossFunction {
    pub fn evaluate(&self, sq_norm: f64, out: *mut f64) {
        (self.0)(sq_norm, out)
    }
}

impl From<Box<dyn Fn(f64, *mut f64)>> for RustLossFunction {
    fn from(value: Box<dyn Fn(f64, *mut f64)>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::slice_from_raw_parts_mut;

    use approx::assert_abs_diff_eq;
    use cxx::UniquePtr;

    // y = (x - 3), J = 1
    fn cost_evaluate(
        parameters: *const *const f64,
        residuals: *mut f64,
        jacobians: *mut *mut f64,
    ) -> bool {
        let x = unsafe { **parameters };
        unsafe {
            *residuals = x - 3.0;
        }
        if !jacobians.is_null() {
            let d_dx = unsafe { *jacobians };
            if !d_dx.is_null() {
                unsafe {
                    *d_dx = 1.0;
                }
            }
        }
        true
    }

    // Just the trivial loss
    fn loss_evaluate(sq_norm: f64, out: *mut f64) {
        let out = slice_from_raw_parts_mut(out, 3);
        unsafe {
            (*out)[0] = sq_norm;
            (*out)[1] = 1.0;
            (*out)[2] = 0.0;
        }
    }

    fn end_to_end(loss: UniquePtr<ffi::LossFunction>) {
        let parameter_block_sizes = [1];
        let mut x_init = [0.0];
        let parameter_blocks = [&mut x_init as *mut f64];

        let rust_cost_function = RustCostFunction(Box::new(cost_evaluate));
        let cost_function = ffi::new_callback_cost_function(
            Box::new(rust_cost_function),
            1,
            &parameter_block_sizes,
        );

        let mut problem = ffi::new_problem();
        unsafe {
            ffi::add_residual_block(
                problem.as_mut().unwrap(),
                cost_function,
                loss,
                parameter_blocks.as_ptr(),
                parameter_blocks.len() as i32,
            );
        }

        let mut options = ffi::new_solver_options();
        options
            .as_mut()
            .unwrap()
            .set_logging_type(ffi::LoggingType::SILENT);

        let mut summary = ffi::new_solver_summary();
        ffi::solve(
            options.as_ref().unwrap(),
            problem.as_mut().unwrap(),
            summary.as_mut().unwrap(),
        );

        assert_abs_diff_eq!(x_init[0], 3.0, epsilon = 1e-8);
    }

    #[test]
    fn end_to_end_no_loss() {
        end_to_end(UniquePtr::null());
    }

    #[test]
    fn end_to_end_custom_loss() {
        let rust_loss_function = RustLossFunction(Box::new(loss_evaluate));
        let loss_function = ffi::new_callback_loss_function(Box::new(rust_loss_function));
        end_to_end(loss_function);
    }

    #[test]
    fn end_to_end_stock_loss() {
        end_to_end(ffi::new_arctan_loss(1.0));
    }
}
