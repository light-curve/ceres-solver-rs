//
// Created by Konstantin Malanchev on 2023-01-24.
//

#ifndef CERES_SOLVER_RS_LIB_H
#define CERES_SOLVER_RS_LIB_H

#include <ceres/ceres.h>
#include <rust/cxx.h>

namespace ceres {
    struct RustCostFunction;
    struct CallbackCostFunction final : public CostFunction {
        rust::Box<RustCostFunction> inner;
        CallbackCostFunction(rust::Box<RustCostFunction> inner,
                             int num_residuals,
                             rust::Slice<const int32_t> parameter_block_sizes);
        // CostFunction impl
        virtual bool Evaluate(double const* const* parameters,
                              double* residuals,
                              double** jacobians) const override;
    };
    std::unique_ptr<CallbackCostFunction> new_callback_cost_function(rust::Box<RustCostFunction> inner,
                                                                     int num_residuals,
                                                                     rust::Slice<const int32_t> parameter_block_sizes);

    struct RustLossFunction;
    struct CallbackLossFunction final : public LossFunction {
        rust::Box<RustLossFunction> inner;
        CallbackLossFunction(rust::Box<RustLossFunction> inner);
        // LossFunction impl
        virtual void Evaluate(double sq_norm, double out[3]) const override;
    };
    std::unique_ptr<LossFunction> new_callback_loss_function(rust::Box<RustLossFunction> inner);
    std::unique_ptr<LossFunction> new_trivial_loss();
    std::unique_ptr<LossFunction> new_huber_loss(double a);
    std::unique_ptr<LossFunction> new_soft_l_one_loss(double a);
    std::unique_ptr<LossFunction> new_cauchy_loss(double a);
    std::unique_ptr<LossFunction> new_arctan_loss(double a);
    std::unique_ptr<LossFunction> new_tolerant_loss(double a, double b);
    std::unique_ptr<LossFunction> new_tukey_loss(double a);

    std::unique_ptr<Problem> new_problem();
    std::shared_ptr<ResidualBlockId> add_residual_block(Problem& problem,
                                                        std::unique_ptr<CallbackCostFunction> cost_function,
                                                        std::unique_ptr<LossFunction> loss_function,
                                                        double* const* const parameter_blocks,
                                                        int num_parameter_blocks);

    struct SolverOptions {
        Solver::Options inner;
        SolverOptions();
        bool is_valid(std::unique_ptr<std::string>& error) const;
        void set_minimizer_type(MinimizerType minimizer_type);
        void set_line_search_direction_type(LineSearchDirectionType line_search_direction_type);
        void set_line_search_type(LineSearchType line_search_type);
        void set_nonlinear_conjugate_gradient_type(NonlinearConjugateGradientType nonlinear_conjugate_gradient_type);
        void set_max_lbfgs_rank(int max_rank);
        void set_use_approximate_eigenvalue_bfgs_scaling(bool yes);
        void set_line_search_interpolation_type(LineSearchInterpolationType line_search_interpolation_type);
        void set_min_line_search_step_size(double step_size);
        void set_line_search_sufficient_function_decrease(double sufficient_decrease);
        void set_max_line_search_step_contraction(double max_step_contraction);
        void set_min_line_search_step_contraction(double min_step_contraction);
        void set_max_num_line_search_direction_restarts(int max_num_line_search_direction_restarts);
        void set_line_search_sufficient_curvature_decrease(double sufficient_curvature_decrease);
        void set_max_line_search_step_expansion(double max_line_search_step_expansion);
        void set_trust_region_strategy_type(TrustRegionStrategyType trust_region_strategy_type);
        void set_dogleg_type(DoglegType dogleg_type);
        void set_use_nonmonotonic_steps(bool yes);
        void set_max_consecutive_nonmonotonic_steps(int max_consecutive_nonmonotonic_steps);
        void set_max_num_iterations(int max_num_iterations);
        void set_max_solver_time_in_seconds(double max_solver_time_in_seconds);
        void set_num_threads(int num_threads);
        void set_initial_trust_region_radius(double initial_trust_region_radius);
        void set_max_trust_region_radius(double max_trust_region_radius);
        void set_min_trust_region_radius(double min_trust_region_radius);
        void set_min_relative_decrease(double min_relative_decrease);
        void set_min_lm_diagonal(double min_lm_diagonal);
        void set_max_lm_diagonal(double max_lm_diagonal);
        void set_max_num_consecutive_invalid_steps(int max_num_consecutive_invalid_steps);
        void set_function_tolerance(double function_tolerance);
        void set_gradient_tolerance(double gradient_tolerance);
        void set_parameter_tolerance(double parameter_tolerance);
        void set_linear_solver_type(LinearSolverType linear_solver_type);
        void set_preconditioner_type(PreconditionerType preconditioner_type);
        void set_visibility_clustering_type(VisibilityClusteringType visibility_clustering_type);
        void set_residual_blocks_for_subset_preconditioner(rust::Slice<const std::shared_ptr<ResidualBlockId>> residual_blocks);
        void set_dense_linear_algebra_library_type(DenseLinearAlgebraLibraryType dense_linear_algebra_library_type);
        void set_sparse_linear_algebra_library_type(SparseLinearAlgebraLibraryType sparse_linear_algebra_library_type);
        // We skip bundle adjustment specific options.
        void set_logging_type(LoggingType logging_type);
        void set_minimizer_progress_to_stdout(bool yes);
        void set_trust_region_minimizer_iterations_to_dump(rust::Slice<const int32_t> iterations);
        void set_trust_region_problem_dump_directory(std::unique_ptr<std::string> directory);
        void set_trust_region_problem_dump_format_type(DumpFormatType trust_region_problem_dump_format_type);
        void set_check_gradients(bool yes);
        void set_gradient_check_relative_precision(double relative_precision);
        void set_gradient_check_numeric_derivative_relative_step_size(double relative_step_size);
        void set_update_state_every_iteration(bool yes);
        // Callbacks are skipped for now.
    };
    std::unique_ptr<SolverOptions> new_solver_options();

    struct SolverSummary {
        Solver::Summary inner;
        SolverSummary();
        std::unique_ptr<std::string> brief_report() const;
        std::unique_ptr<std::string> full_report() const;
        bool is_solution_usable() const;
        double initial_cost() const;
        double final_cost() const;
        double fixed_cost() const;
        int num_successful_steps() const;
        int num_unsuccessful_steps() const;
        int num_inner_iteration_steps() const;
        int num_line_search_steps() const;
        // No times nor sovler/problem options for now.
    };
    std::unique_ptr<SolverSummary> new_solver_summary();

    void solve(const SolverOptions& options, Problem& problem, SolverSummary& summary);
}

#endif //CERES_SOLVER_RS_LIB_H
