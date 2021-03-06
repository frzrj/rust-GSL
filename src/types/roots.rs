//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/*!
# One dimensional Root-Finding

This chapter describes routines for finding roots of arbitrary one-dimensional functions.
The library provides low level components for a variety of iterative solvers and convergence
tests. These can be combined by the user to achieve the desired solution, with full access to
the intermediate steps of the iteration. Each class of methods uses the same framework, so
that you can switch between solvers at runtime without needing to recompile your program.
Each instance of a solver keeps track of its own state, allowing the solvers to be used in
multi-threaded programs.

## Overview

One-dimensional root finding algorithms can be divided into two classes, root bracketing and
root polishing. Algorithms which proceed by bracketing a root are guaranteed to converge.
Bracketing algorithms begin with a bounded region known to contain a root. The size of
this bounded region is reduced, iteratively, until it encloses the root to a desired tolerance.
This provides a rigorous error estimate for the location of the root.

The technique of root polishing attempts to improve an initial guess to the root. These
algorithms converge only if started “close enough” to a root, and sacrifice a rigorous error
bound for speed. By approximating the behavior of a function in the vicinity of a root they
attempt to find a higher order improvement of an initial guess. When the behavior of the
function is compatible with the algorithm and a good initial guess is available a polishing
algorithm can provide rapid convergence.

In GSL both types of algorithm are available in similar frameworks. The user provides
a high-level driver for the algorithms, and the library provides the individual functions
necessary for each of the steps. There are three main phases of the iteration. The steps are,
• initialize solver state, s, for algorithm T
• update s using the iteration T
• test s for convergence, and repeat iteration if necessary

The state for bracketing solvers is held in a gsl_root_fsolver struct. The updating
procedure uses only function evaluations (not derivatives). The state for root polishing
solvers is held in a gsl_root_fdfsolver struct. The updates require both the function and
its derivative (hence the name fdf) to be supplied by the user.
!*/

use ffi::FFI;
use std::os::raw::{c_double, c_void};

use std::boxed::Box;
use std::mem::transmute;

ffi_wrapper!(
    RootFSolverType,
    *const sys::gsl_root_fsolver_type,
    "The root bracketing algorithms described in this section require an initial interval which is
guaranteed to contain a root—if a and b are the endpoints of the interval then f (a) must
differ in sign from f (b). This ensures that the function crosses zero at least once in the
interval. If a valid initial interval is used then these algorithm cannot fail, provided the
function is well-behaved.

Note that a bracketing algorithm cannot find roots of even degree, since these do not
cross the x-axis."
);

impl RootFSolverType {
    /// The bisection algorithm is the simplest method of bracketing the roots of a function.
    /// It is the slowest algorithm provided by the library, with linear convergence.
    /// On each iteration, the interval is bisected and the value of the function at the midpoint
    /// is calculated. The sign of this value is used to determine which half of the interval does
    /// not contain a root. That half is discarded to give a new, smaller interval containing
    /// the root. This procedure can be continued indefinitely until the interval is sufficiently
    /// small.
    ///
    /// At any time the current estimate of the root is taken as the midpoint of the interval.
    pub fn bisection() -> RootFSolverType {
        ffi_wrap!(gsl_root_fsolver_bisection)
    }

    /// The false position algorithm is a method of finding roots based on linear interpolation.
    /// Its convergence is linear, but it is usually faster than bisection.
    ///
    /// On each iteration a line is drawn between the endpoints (a, f (a)) and (b, f (b)) and
    /// the point where this line crosses the x-axis taken as a “midpoint”. The value of the
    /// function at this point is calculated and its sign is used to determine which side of the
    /// interval does not contain a root. That side is discarded to give a new, smaller interval
    /// containing the root. This procedure can be continued indefinitely until the interval
    /// is sufficiently small.
    ///
    /// The best estimate of the root is taken from the linear interpolation of the interval on
    /// the current iteration.
    pub fn brent() -> RootFSolverType {
        ffi_wrap!(gsl_root_fsolver_brent)
    }

    /// The Brent-Dekker method (referred to here as Brent’s method) combines an interpo-
    /// lation strategy with the bisection algorithm. This produces a fast algorithm which is
    /// still robust.

    /// On each iteration Brent’s method approximates the function using an interpolating
    /// curve. On the first iteration this is a linear interpolation of the two endpoints. For
    /// subsequent iterations the algorithm uses an inverse quadratic fit to the last three
    /// points, for higher accuracy. The intercept of the interpolating curve with the x-axis
    /// is taken as a guess for the root. If it lies within the bounds of the current interval
    /// then the interpolating point is accepted, and used to generate a smaller interval. If
    /// the interpolating point is not accepted then the algorithm falls back to an ordinary
    /// bisection step.
    ///
    /// The best estimate of the root is taken from the most recent interpolation or bisection.
    pub fn falsepos() -> RootFSolverType {
        ffi_wrap!(gsl_root_fsolver_falsepos)
    }
}

ffi_wrapper!(
    RootFSolver,
    *mut sys::gsl_root_fsolver,
    gsl_root_fsolver_free
);

impl RootFSolver {
    /// This function returns a pointer to a newly allocated instance of a solver of type T.
    ///
    /// If there is insufficient memory to create the solver then the function returns a null
    /// pointer and the error handler is invoked with an error code of `Value::NoMemory`.
    #[doc(alias = "gsl_root_fsolver_alloc")]
    pub fn new(t: &RootFSolverType) -> Option<RootFSolver> {
        let tmp = unsafe { sys::gsl_root_fsolver_alloc(t.unwrap_shared()) };

        if tmp.is_null() {
            None
        } else {
            Some(RootFSolver::wrap(tmp))
        }
    }

    /// This function initializes, or reinitializes, an existing solver s to use the function f and
    /// the initial search interval [x lower, x upper].
    #[doc(alias = "gsl_root_fsolver_set")]
    pub fn set<F: Fn(f64) -> f64>(&mut self, f: F, x_lower: f64, x_upper: f64) -> ::Value {
        unsafe extern "C" fn inner<F: Fn(f64) -> f64>(
            x: c_double,
            params: *mut c_void,
        ) -> c_double {
            let params: &F = &*(params as *const F);
            params(x)
        }
        ::Value::from(unsafe {
            let f: Box<F> = Box::new(f);
            let params = Box::into_raw(f);

            let mut func = sys::gsl_function {
                function: Some(transmute::<
                    _,
                    unsafe extern "C" fn(c_double, *mut c_void) -> c_double,
                >(inner::<F> as *const ())),
                params: params as *mut _,
            };
            let r = sys::gsl_root_fsolver_set(self.unwrap_unique(), &mut func, x_lower, x_upper);
            // We free the closure now that we're done using it.
            Box::from_raw(params);
            r
        })
    }

    /// The following function drives the iteration of each algorithm. Each function performs one
    /// iteration to update the state of any solver of the corresponding type. The same func-
    /// tion works for all solvers so that different methods can be substituted at runtime without
    /// modifications to the code.
    ///
    /// This function performs a single iteration of the solver s. If the iteration encounters
    /// an unexpected problem then an error code will be returned.
    ///
    /// The solver maintains a current best estimate of the root at all times. The bracketing
    /// solvers also keep track of the current best interval bounding the root.
    #[doc(alias = "gsl_root_fsolver_iterate")]
    pub fn iterate(&mut self) -> ::Value {
        ::Value::from(unsafe { sys::gsl_root_fsolver_iterate(self.unwrap_unique()) })
    }

    /// Returns the solver type name.
    #[doc(alias = "gsl_root_fsolver_name")]
    pub fn name(&self) -> String {
        unsafe {
            let tmp = sys::gsl_root_fsolver_name(self.unwrap_shared());

            String::from_utf8_lossy(::std::ffi::CStr::from_ptr(tmp).to_bytes()).to_string()
        }
    }

    /// This function returns the current estimate of the root for the solver s.
    #[doc(alias = "gsl_root_fsolver_root")]
    pub fn root(&self) -> f64 {
        unsafe { sys::gsl_root_fsolver_root(self.unwrap_shared()) }
    }

    /// These functions return the current bracketing interval for the solver s.
    #[doc(alias = "gsl_root_fsolver_x_lower")]
    pub fn x_lower(&self) -> f64 {
        unsafe { sys::gsl_root_fsolver_x_lower(self.unwrap_shared()) }
    }

    /// These functions return the current bracketing interval for the solver s.
    #[doc(alias = "gsl_root_fsolver_x_upper")]
    pub fn x_upper(&self) -> f64 {
        unsafe { sys::gsl_root_fsolver_x_upper(self.unwrap_shared()) }
    }
}

ffi_wrapper!(
    RootFdfSolverType,
    *const sys::gsl_root_fdfsolver_type,
    "The root polishing algorithms described in this section require an initial guess for the
location of the root. There is no absolute guarantee of convergence—the function must be
suitable for this technique and the initial guess must be sufficiently close to the root
for it to work. When these conditions are satisfied then convergence is quadratic.

These algorithms make use of both the function and its derivative."
);

impl RootFdfSolverType {
    /// Newton’s Method is the standard root-polishing algorithm. The algorithm begins
    /// with an initial guess for the location of the root. On each iteration, a line tangent to
    /// the function f is drawn at that position. The point where this line crosses the x-axis
    /// becomes the new guess.
    pub fn newton() -> RootFdfSolverType {
        ffi_wrap!(gsl_root_fdfsolver_newton)
    }

    /// The secant method is a simplified version of Newton’s method which does not require
    /// the computation of the derivative on every step.
    pub fn secant() -> RootFdfSolverType {
        ffi_wrap!(gsl_root_fdfsolver_secant)
    }

    /// The Steffenson Method 1 provides the fastest convergence of all the routines. It com-
    /// bines the basic Newton algorithm with an Aitken “delta-squared” acceleration.
    pub fn steffenson() -> RootFdfSolverType {
        ffi_wrap!(gsl_root_fdfsolver_steffenson)
    }
}

ffi_wrapper!(
    RootFdfSolver,
    *mut sys::gsl_root_fdfsolver,
    gsl_root_fdfsolver_free
);

impl RootFdfSolver {
    /// This function returns a pointer to a newly allocated instance of a derivative-based
    /// solver of type T.
    ///
    /// If there is insufficient memory to create the solver then the function returns a null
    /// pointer and the error handler is invoked with an error code of `Value::NoMemory`.
    #[doc(alias = "gsl_root_fdfsolver_alloc")]
    pub fn new(t: &RootFdfSolverType) -> Option<RootFdfSolver> {
        let tmp = unsafe { sys::gsl_root_fdfsolver_alloc(t.unwrap_shared()) };

        if tmp.is_null() {
            None
        } else {
            Some(RootFdfSolver::wrap(tmp))
        }
    }

    /// This function initializes, or reinitializes, an existing solver s to use the function and
    /// derivative fdf and the initial guess root.
    #[doc(alias = "gsl_root_fdfsolver_set")]
    pub fn set<F: Fn(f64) -> f64, DF: Fn(f64) -> f64, FDF: Fn(f64, &mut f64, &mut f64)>(
        &mut self,
        f: F,
        df: DF,
        fdf: FDF,
        root: f64,
    ) -> ::Value {
        unsafe extern "C" fn inner_f<F: Fn(f64) -> f64>(
            x: c_double,
            params: *mut c_void,
        ) -> c_double {
            let params: &(*const F, *const (), *const ()) =
                &*(params as *const (*const F, *const (), *const ()));
            let f = &*params.0;
            f(x)
        }
        unsafe extern "C" fn inner_df<DF: Fn(f64) -> f64>(
            x: c_double,
            params: *mut c_void,
        ) -> c_double {
            let params: &(*const (), *const DF, *const ()) =
                &*(params as *const (*const (), *const DF, *const ()));
            let df = &*params.1;
            df(x)
        }
        unsafe extern "C" fn inner_fdf<FDF: Fn(f64, &mut f64, &mut f64)>(
            x: c_double,
            params: *mut c_void,
            y: *mut c_double,
            dy: *mut c_double,
        ) {
            let params: &(*const (), *const (), *const FDF) =
                &*(params as *const (*const (), *const (), *const FDF));
            let fdf = &*params.2;
            fdf(x, &mut *y, &mut *dy);
        }

        ::Value::from(unsafe {
            let f: Box<F> = Box::new(f);
            let f = Box::into_raw(f);
            let df: Box<DF> = Box::new(df);
            let df = Box::into_raw(df);
            let fdf: Box<FDF> = Box::new(fdf);
            let fdf = Box::into_raw(fdf);

            let params = Box::new((f, df, fdf));
            let params = Box::into_raw(params);

            let mut func = sys::gsl_function_fdf {
                f: Some(transmute::<
                    _,
                    unsafe extern "C" fn(c_double, *mut c_void) -> c_double,
                >(inner_f::<F> as *const ())),
                df: Some(transmute::<
                    _,
                    unsafe extern "C" fn(c_double, *mut c_void) -> c_double,
                >(inner_df::<DF> as *const ())),
                fdf: Some(transmute::<
                    _,
                    unsafe extern "C" fn(c_double, *mut c_void, *mut c_double, *mut c_double),
                >(inner_fdf::<FDF> as *const ())),
                params: params as *mut _,
            };
            let r = sys::gsl_root_fdfsolver_set(self.unwrap_unique(), &mut func, root);
            // We free the closure now that we're done using it.
            let tmp = Box::from_raw(params);
            Box::from_raw(tmp.0);
            Box::from_raw(tmp.1);
            Box::from_raw(tmp.2);
            r
        })
    }

    /// The following function drives the iteration of each algorithm. Each function performs one
    /// iteration to update the state of any solver of the corresponding type. The same func-
    /// tion works for all solvers so that different methods can be substituted at runtime without
    /// modifications to the code.
    ///
    /// This function performs a single iteration of the solver s. If the iteration encounters
    /// an unexpected problem then an error code will be returned.
    ///
    /// The solver maintains a current best estimate of the root at all times. The bracketing
    /// solvers also keep track of the current best interval bounding the root.
    #[doc(alias = "gsl_root_fdfsolver_iterate")]
    pub fn iterate(&mut self) -> ::Value {
        ::Value::from(unsafe { sys::gsl_root_fdfsolver_iterate(self.unwrap_unique()) })
    }

    /// Returns the solver type name.
    #[doc(alias = "gsl_root_fdfsolver_name")]
    pub fn name(&self) -> Option<&str> {
        unsafe {
            let ptr = sys::gsl_root_fdfsolver_name(self.unwrap_shared());

            if ptr.is_null() {
                return None;
            }

            let mut len = 0;
            while *ptr.add(len) != 0 {
                len += 1;
            }

            let slice = ::std::slice::from_raw_parts(ptr as *const _, len);
            ::std::str::from_utf8(slice).ok()
        }
    }

    /// This function returns the current estimate of the root for the solver s.
    #[doc(alias = "gsl_root_fdfsolver_root")]
    pub fn root(&self) -> f64 {
        unsafe { sys::gsl_root_fdfsolver_root(self.unwrap_shared()) }
    }
}
