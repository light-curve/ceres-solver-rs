//! Residual-block related structures.

use ceres_solver_sys::cxx::SharedPtr;
use ceres_solver_sys::ffi;
use std::pin::Pin;

pub type ResidualBlockId = SharedPtr<ffi::ResidualBlockId>;

#[allow(dead_code)] // we use this struct to pin the parameter pointers array in memory
pub(crate) struct ResidualBlock {
    pub(crate) id: ResidualBlockId,
    pub(crate) parameter_pointers: Pin<Vec<*mut f64>>,
}
