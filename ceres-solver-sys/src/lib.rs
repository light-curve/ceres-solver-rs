#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Manual translation of c_api_test.cc from version 2.1.0
#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_abs_diff_eq;
    use std::mem::transmute;
    use std::os::raw::{c_int, c_void};
    use std::ptr::null_mut;
    use std::slice;

    const NUM_OBSERVATIONS: usize = 67;
    const NDIM: usize = 2;
    const NPARAM: usize = 2;
    const DATA: [f64; NUM_OBSERVATIONS * NDIM] = [
        0.000000e+00,
        1.133898e+00,
        7.500000e-02,
        1.334902e+00,
        1.500000e-01,
        1.213546e+00,
        2.250000e-01,
        1.252016e+00,
        3.000000e-01,
        1.392265e+00,
        3.750000e-01,
        1.314458e+00,
        4.500000e-01,
        1.472541e+00,
        5.250000e-01,
        1.536218e+00,
        6.000000e-01,
        1.355679e+00,
        6.750000e-01,
        1.463566e+00,
        7.500000e-01,
        1.490201e+00,
        8.250000e-01,
        1.658699e+00,
        9.000000e-01,
        1.067574e+00,
        9.750000e-01,
        1.464629e+00,
        1.050000e+00,
        1.402653e+00,
        1.125000e+00,
        1.713141e+00,
        1.200000e+00,
        1.527021e+00,
        1.275000e+00,
        1.702632e+00,
        1.350000e+00,
        1.423899e+00,
        1.425000e+00,
        1.543078e+00,
        1.500000e+00,
        1.664015e+00,
        1.575000e+00,
        1.732484e+00,
        1.650000e+00,
        1.543296e+00,
        1.725000e+00,
        1.959523e+00,
        1.800000e+00,
        1.685132e+00,
        1.875000e+00,
        1.951791e+00,
        1.950000e+00,
        2.095346e+00,
        2.025000e+00,
        2.361460e+00,
        2.100000e+00,
        2.169119e+00,
        2.175000e+00,
        2.061745e+00,
        2.250000e+00,
        2.178641e+00,
        2.325000e+00,
        2.104346e+00,
        2.400000e+00,
        2.584470e+00,
        2.475000e+00,
        1.914158e+00,
        2.550000e+00,
        2.368375e+00,
        2.625000e+00,
        2.686125e+00,
        2.700000e+00,
        2.712395e+00,
        2.775000e+00,
        2.499511e+00,
        2.850000e+00,
        2.558897e+00,
        2.925000e+00,
        2.309154e+00,
        3.000000e+00,
        2.869503e+00,
        3.075000e+00,
        3.116645e+00,
        3.150000e+00,
        3.094907e+00,
        3.225000e+00,
        2.471759e+00,
        3.300000e+00,
        3.017131e+00,
        3.375000e+00,
        3.232381e+00,
        3.450000e+00,
        2.944596e+00,
        3.525000e+00,
        3.385343e+00,
        3.600000e+00,
        3.199826e+00,
        3.675000e+00,
        3.423039e+00,
        3.750000e+00,
        3.621552e+00,
        3.825000e+00,
        3.559255e+00,
        3.900000e+00,
        3.530713e+00,
        3.975000e+00,
        3.561766e+00,
        4.050000e+00,
        3.544574e+00,
        4.125000e+00,
        3.867945e+00,
        4.200000e+00,
        4.049776e+00,
        4.275000e+00,
        3.885601e+00,
        4.350000e+00,
        4.110505e+00,
        4.425000e+00,
        4.345320e+00,
        4.500000e+00,
        4.161241e+00,
        4.575000e+00,
        4.363407e+00,
        4.650000e+00,
        4.161576e+00,
        4.725000e+00,
        4.619728e+00,
        4.800000e+00,
        4.737410e+00,
        4.875000e+00,
        4.727863e+00,
        4.950000e+00,
        4.669206e+00,
    ];

    extern "C" fn exponential_residual(
        user_data: *mut c_void,
        parameters: *mut *mut f64,
        residuals: *mut f64,
        jacobians: *mut *mut f64,
    ) -> c_int {
        unsafe {
            let measurement: &[f64] = slice::from_raw_parts(transmute(user_data), NDIM);
            let x = measurement[0];
            let y = measurement[1];

            let parameters = slice::from_raw_parts(parameters, NPARAM);
            let m = *parameters[0];
            let c = *parameters[1];

            *residuals = y - f64::exp(m * x + c);

            if jacobians.is_null() {
                return 1;
            }
            let jacobians = slice::from_raw_parts_mut(jacobians, NPARAM);
            if !jacobians[0].is_null() {
                *jacobians[0] = -x * f64::exp(m * x + c); // dr/dm
            }
            if !jacobians[1].is_null() {
                *jacobians[1] = -f64::exp(m * x + c); // dr/dc
            }
        }
        1
    }

    #[test]
    fn simple_end_to_end_test() {
        let mut data = DATA;

        let mut m = 0.0;
        let mut c = 0.0;
        let mut parameter_pointers = [&mut m as *mut _, &mut c as *mut _];
        let mut parameter_sizes = [1, 1];

        unsafe {
            let problem = ceres_create_problem();
            for i in 0..NUM_OBSERVATIONS {
                let _block_id = ceres_problem_add_residual_block(
                    problem,
                    Some(exponential_residual),
                    data[NDIM * i..NDIM * (i + 1)].as_mut_ptr() as *mut c_void,
                    None,
                    null_mut(),
                    1,
                    NPARAM as c_int,
                    parameter_sizes.as_mut_ptr(),
                    parameter_pointers.as_mut_ptr(),
                );
            }
            ceres_solve(problem);
            ceres_free_problem(problem);
        }

        assert_abs_diff_eq!(0.3, m, epsilon = 0.02);
        assert_abs_diff_eq!(0.1, c, epsilon = 0.04);
    }

    #[test]
    fn loss_functions() {
        let mut data = DATA;
        data[12] = 2.5;
        data[13] = 1.0e3;
        data[14] = 3.2;
        data[15] = 30e3;

        let mut m = 0.2;
        let mut c = 0.03;
        let mut parameter_pointers = [&mut m as *mut _, &mut c as *mut _];
        let mut parameter_sizes = [1, 1];

        unsafe {
            let cauchy_loss_data = ceres_create_cauchy_loss_function_data(5.0);
            let problem = ceres_create_problem();
            for i in 0..NUM_OBSERVATIONS {
                let _block_id = ceres_problem_add_residual_block(
                    problem,
                    Some(exponential_residual),
                    data[NDIM * i..NDIM * (i + 1)].as_mut_ptr() as *mut c_void,
                    Some(ceres_stock_loss_function),
                    cauchy_loss_data,
                    1,
                    NPARAM as c_int,
                    parameter_sizes.as_mut_ptr(),
                    parameter_pointers.as_mut_ptr(),
                );
            }
            ceres_solve(problem);
            ceres_free_problem(problem);
        }

        assert_abs_diff_eq!(0.3, m, epsilon = 0.02);
        assert_abs_diff_eq!(0.1, c, epsilon = 0.04);
    }
}
