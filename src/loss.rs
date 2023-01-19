use ceres_solver_sys as sys;
use std::os::raw::c_void;
use std::pin::Pin;
use std::slice;

pub type LossFunctionType = Box<dyn Fn(f64, &mut [f64; 3])>;

/// Loss function for [ResidualBlock](crate::residual_block::ResidualBlock) and
/// [CurveFitProblem1D](crate::curve_fit::CurveFitProblem1D), it is a transformation of the squared
/// residuals which is generally used to make the solver less sensitive to outliers. This enum has
/// two flavours: user specified function and Ceres stock function.
pub enum LossFunction {
    /// User-specified loss function.
    Custom(CustomLossFunction),
    /// One of the loss functions specified by Ceres, see
    /// [Ceres Solver docs for details](http://ceres-solver.org/nnls_modeling.html#instances).
    Stock(StockLossFunction),
}

impl LossFunction {
    /// Create a [LossFunction] to handle a custom loss function.
    ///
    /// # Arguments
    /// - func - a boxed function which accepts two arguments: non-negative squared residual, and
    ///  an array of 0) loss function value, 1) its first, and 2) its second derivatives. See
    /// details at
    /// <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12LossFunctionE>.
    pub fn custom(func: impl Into<LossFunctionType>) -> Self {
        let func: LossFunctionType = func.into();
        Self::Custom(CustomLossFunction {
            func: Box::pin(func),
        })
    }

    /// Huber loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres9HuberLossE>.
    pub fn huber(a: f64) -> Self {
        let stock = StockLossFunction {
            inner: unsafe { sys::ceres_create_huber_loss_function_data(a) },
            name: "Huber",
        };
        Self::Stock(stock)
    }

    /// Soft L1 loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12SoftLOneLossE>.
    pub fn soft_l1(a: f64) -> Self {
        let stock = StockLossFunction {
            inner: unsafe { sys::ceres_create_softl1_loss_function_data(a) },
            name: "SoftLOne",
        };
        Self::Stock(stock)
    }

    /// log(1+s) loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres10CauchyLossE>.
    pub fn cauchy(a: f64) -> Self {
        let stock = StockLossFunction {
            inner: unsafe { sys::ceres_create_cauchy_loss_function_data(a) },
            name: "Cauchy",
        };
        Self::Stock(stock)
    }

    /// Arctangent loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres10ArctanLossE>.
    pub fn arctan(a: f64) -> Self {
        let stock = StockLossFunction {
            inner: unsafe { sys::ceres_create_arctan_loss_function_data(a) },
            name: "Arctan",
        };
        Self::Stock(stock)
    }

    /// Tolerant loss function, see details at <http://ceres-solver.org/nnls_modeling.html#_CPPv4N5ceres12TolerantLossE>.
    pub fn tolerant_loss(a: f64, b: f64) -> Self {
        let stock = StockLossFunction {
            inner: unsafe { sys::ceres_create_tolerant_loss_function_data(a, b) },
            name: "TolerantLoss",
        };
        Self::Stock(stock)
    }

    /// Calls the underlying loss function.
    #[inline]
    pub fn loss(&self, squared_norm: f64, out: &mut [f64; 3]) {
        match self {
            Self::Custom(custom) => (custom.func)(squared_norm, out),
            Self::Stock(stock) => stock.loss(squared_norm, out),
        }
    }

    pub(crate) fn ffi_function(&self) -> unsafe extern "C" fn(*mut c_void, f64, *mut f64) {
        match self {
            Self::Stock(_) => sys::ceres_stock_loss_function,
            Self::Custom(_) => ffi_custom_loss_function,
        }
    }

    pub(crate) fn ffi_user_data(&mut self) -> *mut c_void {
        match self {
            Self::Custom(custom) => {
                Pin::into_inner(custom.func.as_mut()) as *mut LossFunctionType as *mut c_void
            }
            Self::Stock(stock) => stock.inner,
        }
    }
}

/// Custom loss function. Create it with [LossFunction::custom]
pub struct CustomLossFunction {
    pub func: Pin<Box<LossFunctionType>>,
}

/// Stock loss function. Create it with one of the [LossFunction]'s constructors.
pub struct StockLossFunction {
    inner: *mut c_void,
    pub name: &'static str,
}

impl StockLossFunction {
    #[inline]
    fn loss(&self, squared_norm: f64, out: &mut [f64; 3]) {
        unsafe { sys::ceres_stock_loss_function(self.inner, squared_norm, out.as_mut_ptr()) }
    }
}

impl Drop for StockLossFunction {
    fn drop(&mut self) {
        unsafe { sys::ceres_free_stock_loss_function_data(self.inner) }
    }
}

#[no_mangle]
unsafe extern "C" fn ffi_custom_loss_function(
    user_data: *mut c_void,
    squared_norm: f64,
    out: *mut f64,
) {
    let func = (user_data as *mut LossFunctionType).as_ref().unwrap();
    let out = slice::from_raw_parts_mut(out, 3).try_into().unwrap();
    (func)(squared_norm, out)
}
