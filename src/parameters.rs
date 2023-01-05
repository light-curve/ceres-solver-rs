use std::os::raw::c_int;
use std::pin::Pin;

/// [ResidualBlock](crate::residual_block::ResidualBlock)'s internal representation of a problem
/// parameters.
pub struct Parameters {
    values: Pin<Vec<Vec<f64>>>,
    sizes: Vec<usize>,
    sizes_c_int: Vec<c_int>,
    pointers: Vec<*mut f64>,
}

#[allow(clippy::len_without_is_empty)]
impl Parameters {
    /// Creates a new `Parameters` from a vector of parameters, keeping in mind that each parameter
    /// is a [f64] vector itself
    pub fn new(values: impl Into<Vec<Vec<f64>>>) -> Self {
        let mut values = values.into();
        assert!(!values.is_empty());
        let sizes: Vec<_> = values.iter().map(|x| x.len()).collect();
        let sizes_c_int = sizes.iter().map(|&x| x as c_int).collect();
        let pointers = values.iter_mut().map(|p| (*p).as_mut_ptr()).collect();
        Self {
            values: Pin::new(values),
            sizes,
            sizes_c_int,
            pointers,
        }
    }

    /// Number of parameters.
    pub fn len(&self) -> usize {
        self.sizes.len()
    }

    /// Parameter sizes.
    pub fn sizes(&self) -> &[usize] {
        &self.sizes
    }

    pub(crate) fn sizes_c_int_mut(&mut self) -> &mut [c_int] {
        &mut self.sizes_c_int
    }

    pub(crate) fn pointers_mut(&mut self) -> &mut [*mut f64] {
        &mut self.pointers
    }

    /// Convert to vector of parameters, each parameter is vector of floats.
    pub fn to_values(self) -> Vec<Vec<f64>> {
        Pin::into_inner(self.values)
    }
}
