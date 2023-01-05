use crate::cost::CostFunction;
use crate::loss::LossFunction;
use crate::parameters::Parameters;

/// A block of [NllsProblem](crate::nlls_problem::NllsProblem) consists of problem parameters, cost
/// and loss function. Unlike origin Ceres Colver implementation, different blocks cannot share
/// parameters due to Rust safety rules.
pub struct ResidualBlock<'cost> {
    /// Problem parameters.
    pub parameters: Parameters,
    /// Cost function.
    pub cost_function: CostFunction<'cost>,
    /// Optional loss function, the trivial one is used if [None].
    pub loss_function: Option<LossFunction>,
}

impl<'cost> ResidualBlock<'cost> {
    /// Creates a new instance of [ResidualBlock].
    ///
    /// # Arguments
    /// - parameters - A vector of initial parameters, each parameter is a vector of [f64] values
    /// itself. The sizes of the vectors must be consistent with [CostFunction::parameter_sizes],
    /// otherwise this method panics.
    /// - cost_function - An instance of [CostFunction].
    ///
    /// Constructed [ResidualBlock] has no [LossFunction] assigned, use [ResidualBlock::set_loss]
    /// to assign it.
    ///
    /// # Panics
    /// Panics if the shape of `parameters` differs from `cost_function.parameter_sizes()`.
    pub fn new(parameters: impl Into<Vec<Vec<f64>>>, cost_function: CostFunction<'cost>) -> Self {
        let parameters = Parameters::new(parameters);
        assert_eq!(
            parameters.len(),
            cost_function.num_parameters(),
            "parameters shape must be consistent with cost_function.num_parameters()"
        );
        for (&given_size, &cost_size) in parameters
            .sizes()
            .iter()
            .zip(cost_function.parameter_sizes())
        {
            assert_eq!(
                given_size, cost_size,
                "parameters shape must be consistent with cost_function.num_parameters()"
            )
        }
        Self {
            parameters,
            cost_function,
            loss_function: None,
        }
    }

    /// Assigns a loss function applied to squared residuals. If no loss function
    /// assigned, identity function is assumed.
    pub fn set_loss(mut self, loss: LossFunction) -> Self {
        self.loss_function = Some(loss);
        self
    }

    /// Change the loss function.
    pub fn change_loss(mut self, loss: Option<LossFunction>) -> Self {
        self.loss_function = loss;
        self
    }
}
