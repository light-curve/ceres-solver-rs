use crate::types::JacobianType;

use ceres_solver_sys::cxx;
use ceres_solver_sys::ffi;
use std::slice;

pub type CostFunctionType<'a> = Box<dyn Fn(&[&[f64]], &mut [f64], JacobianType<'_>) -> bool + 'a>;

/// A cost function for [NllsProblem](crate::nlls_problem::NllsProblem).
pub struct CostFunction<'cost>(cxx::UniquePtr<ffi::CallbackCostFunction<'cost>>);

impl<'cost> CostFunction<'cost> {
    /// Create a new cost function from a Rust function.
    ///
    /// # Arguments
    /// - func - function to find residuals and Jacobian for the problem block. The function itself
    /// must return [false] if it cannot compute Jacobian, [true] otherwise, and accept following
    /// arguments:
    ///   - parameters - slice of [f64] slices representing the current values of the parameters.
    ///   Each parameter is represented as a slice, the slice sizes are specified by
    ///   `parameter_sizes`.
    ///   - residuals - mutable slice of [f64] for residuals outputs, the size is specified by
    ///   `num_residuals`.
    ///   - jacobians: [JacobianType](crate::types::JacobianType) - represents a mutable
    ///   structure to output the Jacobian. Sometimes the solver doesn't need the Jacobian or
    ///   some of its components, in this case the corresponding value is [None]. For the required
    ///   components it has a 3-D shape: top index is for the parameter index, middle index is for
    ///   the residual index, and the most inner dimension is for the given parameter component
    ///   index. So the size of top-level [Some] is defined by `parameter_sizes.len()`, second-level
    ///   [Some]'s slice length is `num_residuals`, and the bottom-level slice has length of
    ///   `parameter_sizes[i]`, where `i` is the top-level index.
    /// - parameter_sizes - sizes of the parameter vectors.
    /// - num_residuals - length of the residual vector, usually corresponds to the number of
    /// data points.
    pub fn new(
        func: impl Into<CostFunctionType<'cost>>,
        parameter_sizes: impl Into<Vec<usize>>,
        num_residuals: usize,
    ) -> Self {
        let parameter_sizes = parameter_sizes.into();
        let parameter_block_sizes: Vec<_> =
            parameter_sizes.iter().map(|&size| size as i32).collect();

        let safe_func = func.into();
        let rust_func: Box<dyn Fn(*const *const f64, *mut f64, *mut *mut f64) -> bool + 'cost> =
            Box::new(move |parameters_ptr, residuals_ptr, jacobians_ptr| {
                let parameter_pointers =
                    unsafe { slice::from_raw_parts(parameters_ptr, parameter_sizes.len()) };
                let parameters = parameter_pointers
                    .iter()
                    .zip(parameter_sizes.iter())
                    .map(|(&p, &size)| unsafe { slice::from_raw_parts(p, size) })
                    .collect::<Vec<_>>();
                let residuals = unsafe { slice::from_raw_parts_mut(residuals_ptr, num_residuals) };
                let mut jacobians_owned =
                    OwnedJacobian::from_pointer(jacobians_ptr, &parameter_sizes, num_residuals);
                let mut jacobian_references = jacobians_owned.references();
                safe_func(
                    &parameters,
                    residuals,
                    jacobian_references.as_mut().map(|v| &mut v[..]),
                )
            });
        let inner = ffi::new_callback_cost_function(
            Box::new(rust_func.into()),
            num_residuals as i32,
            &parameter_block_sizes,
        );
        Self(inner)
    }

    pub fn into_inner(self) -> cxx::UniquePtr<ffi::CallbackCostFunction<'cost>> {
        self.0
    }
}

struct OwnedJacobian<'a>(Option<Vec<Option<Vec<&'a mut [f64]>>>>);

impl<'a> OwnedJacobian<'a> {
    fn from_pointer(
        pointer: *mut *mut f64,
        parameter_sizes: &[usize],
        num_residuals: usize,
    ) -> Self {
        if pointer.is_null() {
            return Self(None);
        }
        let per_parameter = unsafe { slice::from_raw_parts_mut(pointer, parameter_sizes.len()) };
        let vec = per_parameter
            .iter()
            .zip(parameter_sizes)
            .map(|(&p, &size)| OwnedDerivative::from_pointer(p, size, num_residuals).0)
            .collect();
        Self(Some(vec))
    }

    fn references(&'a mut self) -> Option<Vec<Option<&'a mut [&'a mut [f64]]>>> {
        let v = self
            .0
            .as_mut()?
            .iter_mut()
            .map(|der| der.as_mut().map(|v| &mut v[..]))
            .collect();
        Some(v)
    }
}

struct OwnedDerivative<'a>(Option<Vec<&'a mut [f64]>>);

impl<'a> OwnedDerivative<'a> {
    fn from_pointer(pointer: *mut f64, parameter_size: usize, num_residuals: usize) -> Self {
        if pointer.is_null() {
            return Self(None);
        }
        let per_residual_per_param_component =
            { unsafe { slice::from_raw_parts_mut(pointer, parameter_size * num_residuals) } };
        let v = per_residual_per_param_component
            .chunks_exact_mut(parameter_size)
            .collect();
        Self(Some(v))
    }
}
