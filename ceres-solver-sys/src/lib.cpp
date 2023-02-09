//
// Created by Konstantin Malanchev on 2023-01-24.
//

#include <glog/logging.h>

#include "ceres-solver-sys/src/lib.h"
#include "ceres-solver-sys/src/lib.rs.h"

namespace ceres {
    CallbackCostFunction::CallbackCostFunction(rust::Box<RustCostFunction> inner,
                                               int num_residuals,
                                               rust::Slice<const int32_t> parameter_block_sizes):
        inner(std::move(inner)) {
        set_num_residuals(num_residuals);
        for (auto block_size : parameter_block_sizes) {
            mutable_parameter_block_sizes()->push_back(block_size);
        }
    }
    bool CallbackCostFunction::Evaluate(double const* const* parameters,
                                        double* residuals,
                                        double** jacobians) const {
        return inner->evaluate(parameters, residuals, jacobians);
    }
    std::unique_ptr<CallbackCostFunction> new_callback_cost_function(rust::Box<RustCostFunction> inner,
                                                                     int num_residuals,
                                                                     rust::Slice<const int32_t> parameter_block_sizes) {
        return std::make_unique<CallbackCostFunction>(std::move(inner), num_residuals, std::move(parameter_block_sizes));
    }

    CallbackLossFunction::CallbackLossFunction(rust::Box<RustLossFunction> inner):
        inner(std::move(inner)) {}
    void CallbackLossFunction::Evaluate(double sq_norm, double out[3]) const {
        inner->evaluate(sq_norm, out);
    }
    std::unique_ptr<LossFunction> new_callback_loss_function(rust::Box<RustLossFunction> inner) {
        return std::make_unique<CallbackLossFunction>(std::move(inner));
    }
    std::unique_ptr<LossFunction> new_trivial_loss() {
        return std::make_unique<TrivialLoss>();
    }
    std::unique_ptr<LossFunction> new_huber_loss(double a) {
        return std::make_unique<HuberLoss>(a);
    }
    std::unique_ptr<LossFunction> new_soft_l_one_loss(double a) {
        return std::make_unique<SoftLOneLoss>(a);
    }
    std::unique_ptr<LossFunction> new_cauchy_loss(double a) {
        return std::make_unique<CauchyLoss>(a);
    }
    std::unique_ptr<LossFunction> new_arctan_loss(double a) {
        return std::make_unique<ArctanLoss>(a);
    }
    std::unique_ptr<LossFunction> new_tolerant_loss(double a, double b) {
        return std::make_unique<TolerantLoss>(a, b);
    }
    std::unique_ptr<LossFunction> new_tukey_loss(double a) {
        return std::make_unique<TukeyLoss>(a);
    }

    std::unique_ptr<Problem> new_problem() {
        return std::make_unique<Problem>();
    }
    std::shared_ptr<ResidualBlockId> add_residual_block(Problem& problem,
                                                        std::unique_ptr<CallbackCostFunction> cost_function,
                                                        std::unique_ptr<LossFunction> loss_function,
                                                        double* const* const parameter_blocks,
                                                        int num_parameter_blocks) {
        auto block_id = problem.AddResidualBlock(cost_function.release(),
                                                 loss_function.release(),
                                                 parameter_blocks,
                                                 num_parameter_blocks);
        return std::make_shared<ResidualBlockId>(block_id);
    }

    SolverOptions::SolverOptions():
        inner(Solver::Options()) {}
    bool SolverOptions::is_valid(std::string& error) const {
        return inner.IsValid(&error);
    }
    void SolverOptions::set_minimizer_type(MinimizerType minimizer_type) {
        inner.minimizer_type = minimizer_type;
    }
    void SolverOptions::set_line_search_direction_type(LineSearchDirectionType line_search_direction_type) {
        inner.line_search_direction_type = line_search_direction_type;
    }
    void SolverOptions::set_line_search_type(LineSearchType line_search_type) {
        inner.line_search_type = line_search_type;
    }
    void SolverOptions::set_nonlinear_conjugate_gradient_type(NonlinearConjugateGradientType nonlinear_conjugate_gradient_type) {
        inner.nonlinear_conjugate_gradient_type = nonlinear_conjugate_gradient_type;
    }
    void SolverOptions::set_max_lbfgs_rank(int max_rank) {
        inner.max_lbfgs_rank = max_rank;
    }
    void SolverOptions::set_use_approximate_eigenvalue_bfgs_scaling(bool yes) {
        inner.use_approximate_eigenvalue_bfgs_scaling = yes;
    }
    void SolverOptions::set_line_search_interpolation_type(LineSearchInterpolationType line_search_interpolation_type) {
        inner.line_search_interpolation_type = line_search_interpolation_type;
    }
    void SolverOptions::set_min_line_search_step_size(double step_size) {
        inner.min_line_search_step_size = step_size;
    }
    void SolverOptions::set_line_search_sufficient_function_decrease(double sufficient_decrease) {
        inner.line_search_sufficient_function_decrease = sufficient_decrease;
    }
    void SolverOptions::set_max_line_search_step_contraction(double max_step_contraction) {
        inner.max_line_search_step_contraction = max_step_contraction;
    }
    void SolverOptions::set_min_line_search_step_contraction(double min_step_contraction) {
        inner.min_line_search_step_contraction = min_step_contraction;
    }
    void SolverOptions::set_max_num_line_search_direction_restarts(int max_restarts) {
        inner.max_num_line_search_direction_restarts = max_restarts;
    }
    void SolverOptions::set_line_search_sufficient_curvature_decrease(double sufficient_curvature_decrease) {
        inner.line_search_sufficient_curvature_decrease = sufficient_curvature_decrease;
    }
    void SolverOptions::set_max_line_search_step_expansion(double max_step_expansion) {
        inner.max_line_search_step_expansion = max_step_expansion;
    }
    void SolverOptions::set_trust_region_strategy_type(TrustRegionStrategyType trust_region_strategy_type) {
        inner.trust_region_strategy_type = trust_region_strategy_type;
    }
    void SolverOptions::set_dogleg_type(DoglegType dogleg_type) {
        inner.dogleg_type = dogleg_type;
    }
    void SolverOptions::set_use_nonmonotonic_steps(bool yes) {
        inner.use_nonmonotonic_steps = yes;
    }
    void SolverOptions::set_max_consecutive_nonmonotonic_steps(int max_steps) {
        inner.max_consecutive_nonmonotonic_steps = max_steps;
    }
    void SolverOptions::set_max_num_iterations(int max_iterations) {
        inner.max_num_iterations = max_iterations;
    }
    void SolverOptions::set_max_solver_time_in_seconds(double max_time) {
        inner.max_solver_time_in_seconds = max_time;
    }
    void SolverOptions::set_num_threads(int num_threads) {
        inner.num_threads = num_threads;
    }
    void SolverOptions::set_initial_trust_region_radius(double radius) {
        inner.initial_trust_region_radius = radius;
    }
    void SolverOptions::set_max_trust_region_radius(double radius) {
        inner.max_trust_region_radius = radius;
    }
    void SolverOptions::set_min_trust_region_radius(double radius) {
        inner.min_trust_region_radius = radius;
    }
    void SolverOptions::set_min_relative_decrease(double relative_decrease) {
        inner.min_relative_decrease = relative_decrease;
    }
    void SolverOptions::set_min_lm_diagonal(double lm_diagonal) {
        inner.min_lm_diagonal = lm_diagonal;
    }
    void SolverOptions::set_max_lm_diagonal(double lm_diagonal) {
        inner.max_lm_diagonal = lm_diagonal;
    }
    void SolverOptions::set_max_num_consecutive_invalid_steps(int max_steps) {
        inner.max_num_consecutive_invalid_steps = max_steps;
    }
    void SolverOptions::set_function_tolerance(double tolerance) {
        inner.function_tolerance = tolerance;
    }
    void SolverOptions::set_gradient_tolerance(double tolerance) {
        inner.gradient_tolerance = tolerance;
    }
    void SolverOptions::set_parameter_tolerance(double tolerance) {
        inner.parameter_tolerance = tolerance;
    }
    void SolverOptions::set_linear_solver_type(LinearSolverType linear_solver_type) {
        inner.linear_solver_type = linear_solver_type;
    }
    void SolverOptions::set_preconditioner_type(PreconditionerType preconditioner_type) {
        inner.preconditioner_type = preconditioner_type;
    }
    void SolverOptions::set_visibility_clustering_type(VisibilityClusteringType visibility_clustering_type) {
        inner.visibility_clustering_type = visibility_clustering_type;
    }
    void SolverOptions::set_residual_blocks_for_subset_preconditioner(rust::Slice<const std::shared_ptr<ResidualBlockId>> residual_blocks) {
        inner.residual_blocks_for_subset_preconditioner.clear();
        for (auto &block : residual_blocks) {
            inner.residual_blocks_for_subset_preconditioner.insert(*block);
        }
    }
    void SolverOptions::set_dense_linear_algebra_library_type(DenseLinearAlgebraLibraryType dense_linear_algebra_library_type) {
        inner.dense_linear_algebra_library_type = dense_linear_algebra_library_type;
    }
    void SolverOptions::set_sparse_linear_algebra_library_type(SparseLinearAlgebraLibraryType sparse_linear_algebra_library_type) {
        inner.sparse_linear_algebra_library_type = sparse_linear_algebra_library_type;
    }
    void SolverOptions::set_logging_type(LoggingType logging_type) {
        inner.logging_type = logging_type;
    }
    void SolverOptions::set_minimizer_progress_to_stdout(bool yes) {
        inner.minimizer_progress_to_stdout = yes;
    }
    void SolverOptions::set_trust_region_minimizer_iterations_to_dump(rust::Slice<const int32_t> iterations) {
        inner.trust_region_minimizer_iterations_to_dump.clear();
        for (const auto& iteration : iterations) {
            inner.trust_region_minimizer_iterations_to_dump.push_back(iteration);
        }
    }
    void SolverOptions::set_trust_region_problem_dump_directory(const std::string& directory) {
        inner.trust_region_problem_dump_directory = directory;
    }
    void SolverOptions::set_trust_region_problem_dump_format_type(DumpFormatType trust_region_problem_dump_format_type) {
        inner.trust_region_problem_dump_format_type = trust_region_problem_dump_format_type;
    }
    void SolverOptions::set_check_gradients(bool yes) {
        inner.check_gradients = yes;
    }
    void SolverOptions::set_gradient_check_relative_precision(double precision) {
        inner.gradient_check_relative_precision = precision;
    }
    void SolverOptions::set_gradient_check_numeric_derivative_relative_step_size(double step_size) {
        inner.gradient_check_numeric_derivative_relative_step_size = step_size;
    }
    void SolverOptions::set_update_state_every_iteration(bool yes) {
        inner.update_state_every_iteration = yes;
    }
    std::unique_ptr<SolverOptions> new_solver_options() {
        return std::make_unique<SolverOptions>();
    }

    SolverSummary::SolverSummary():
        inner(Solver::Summary()) {}
    std::unique_ptr<std::string> SolverSummary::brief_report() const {
        return std::make_unique<std::string>(inner.BriefReport());
    }
    std::unique_ptr<std::string> SolverSummary::full_report() const {
        return std::make_unique<std::string>(inner.FullReport());
    }
    bool SolverSummary::is_solution_usable() const {
        return inner.IsSolutionUsable();
    }
    double SolverSummary::initial_cost() const {
        return inner.initial_cost;
    }
    double SolverSummary::final_cost() const {
        return inner.final_cost;
    }
    double SolverSummary::fixed_cost() const {
        return inner.fixed_cost;
    }
    int SolverSummary::num_successful_steps() const {
        return inner.num_successful_steps;
    }
    int SolverSummary::num_unsuccessful_steps() const {
        return inner.num_unsuccessful_steps;
    }
    int SolverSummary::num_inner_iteration_steps() const {
        return inner.num_inner_iteration_steps;
    }
    int SolverSummary::num_line_search_steps() const {
        return inner.num_line_search_steps;
    }
    std::unique_ptr<SolverSummary> new_solver_summary() {
        return std::make_unique<SolverSummary>();
    }

    void solve(const SolverOptions& options, Problem& problem, SolverSummary& summary) {
        Solve(options.inner, &problem, &summary.inner);
    }
}
