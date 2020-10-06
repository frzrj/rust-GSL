//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use enums;
use ffi;

/// This function returns the index i of the array x_array such that x_array[i] <= x < x_array[i+1]. The index is searched for in the
/// range [index_lo,index_hi].
pub fn bsearch(x_array: &[f64], x: f64, index_lo: u64, index_hi: u64) -> u64 {
    unsafe { sys::gsl_interp_bsearch(x_array.as_ptr(), x, index_lo, index_hi) }
}

/// This function returns the interpolated value of y for a given point x, using the interpolation object interp, data arrays xa and ya and
/// the accelerator acc. When x is outside the range of xa, the error code ::Dom is returned with a value of rgsl::NAN for y.
pub fn eval(interp: &::Interp, xa: &[f64], ya: &[f64], x: f64, acc: &mut ::InterpAccel) -> f64 {
    unsafe {
        sys::gsl_interp_eval(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
        )
    }
}

/// This function returns the interpolated value of y for a given point x, using the interpolation
/// object interp, data arrays xa and ya and the accelerator acc. When x is outside the range of xa,
/// the error code ::Dom is returned with a value of rgsl::NAN for y.
///
/// Returns `y`.
pub fn eval_e(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    x: f64,
    acc: &mut ::InterpAccel,
) -> (enums::Value, f64) {
    let mut y = 0.;
    let ret = unsafe {
        sys::gsl_interp_eval_e(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
            &mut y,
        )
    };
    (::Value::from(ret), y)
}

/// This function returns the derivative d of an interpolated function for a given point x, using the interpolation object interp, data
/// arrays xa and ya and the accelerator acc.
pub fn eval_deriv(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    x: f64,
    acc: &mut ::InterpAccel,
) -> f64 {
    unsafe {
        sys::gsl_interp_eval_deriv(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
        )
    }
}

/// This function returns the derivative d of an interpolated function for a given point x, using the interpolation object interp, data
/// arrays xa and ya and the accelerator acc.
pub fn eval_deriv_e(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    x: f64,
    acc: &mut ::InterpAccel,
    d: &mut f64,
) -> enums::Value {
    enums::Value::from(unsafe {
        sys::gsl_interp_eval_deriv_e(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
            d,
        )
    })
}

/// This function returns the second derivative d2 of an interpolated function for a given point x, using the interpolation object interp,
/// data arrays xa and ya and the accelerator acc.
pub fn eval_deriv2(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    x: f64,
    acc: &mut ::InterpAccel,
) -> f64 {
    unsafe {
        sys::gsl_interp_eval_deriv2(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
        )
    }
}

/// This function returns the second derivative d2 of an interpolated function for a given point x, using the interpolation object interp,
/// data arrays xa and ya and the accelerator acc.
pub fn eval_deriv2_e(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    x: f64,
    acc: &mut ::InterpAccel,
    d2: &mut f64,
) -> enums::Value {
    enums::Value::from(unsafe {
        sys::gsl_interp_eval_deriv2_e(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            x,
            &mut acc.0,
            d2,
        )
    })
}

/// This function returns the numerical integral result of an interpolated function over the range [a, b], using the interpolation object
/// interp, data arrays xa and ya and the accelerator acc.
pub fn eval_integ(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    a: f64,
    b: f64,
    acc: &mut ::InterpAccel,
) -> f64 {
    unsafe {
        sys::gsl_interp_eval_integ(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            a,
            b,
            &mut acc.0,
        )
    }
}

/// This function returns the numerical integral result of an interpolated function over the range [a, b], using the interpolation object
/// interp, data arrays xa and ya and the accelerator acc.
pub fn eval_integ_e(
    interp: &::Interp,
    xa: &[f64],
    ya: &[f64],
    a: f64,
    b: f64,
    acc: &mut ::InterpAccel,
    result: &mut f64,
) -> enums::Value {
    enums::Value::from(unsafe {
        sys::gsl_interp_eval_integ_e(
            ffi::FFI::unwrap_shared(interp),
            xa.as_ptr(),
            ya.as_ptr(),
            a,
            b,
            &mut acc.0,
            result,
        )
    })
}
