//! Formula implementation.

use crate::math::is_ascending;
use ndarray::Array1;
use std::fmt::{Display, Error, Formatter};

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug, Clone)]
pub enum Formula {
    /// Constant value. = c
    Constant {
        /// Constant.
        c: f64,
    },
    /// Line formula. = mx + c
    Line {
        /// Offset.
        c: f64,
        /// Gradient.
        m: f64,
    },
    /// Bifurcation formula. = if x < t { under } else { over }.
    Bifurcation {
        /// Threshold value.
        t: f64,
        /// Under value.
        under: f64,
        /// Over value.
        over: f64,
    },
    /// Constant value spline.
    ConstantSpline {
        /// X change points.
        xs: Array1<f64>,
        /// Y values.
        ys: Array1<f64>,
    },
    /// Linear spline.
    LinearSpline {
        /// X change points.
        xs: Array1<f64>,
        /// Y values.
        ys: Array1<f64>,
        /// Gradient between points.
        grads: Array1<f64>,
    },
    /// Quadratic spline.
    QuadraticSpline {
        /// X change points.
        xs: Array1<f64>,
        /// Y values.
        ys: Array1<f64>,
        /// Gradient between points.
        grads: Array1<f64>,
        /// Second order term between points.
        quads: Array1<f64>,
    },
}

impl Formula {
    /// Construct a constant spline instance.
    #[inline]
    #[must_use]
    pub fn new_constant_spline(xs: Array1<f64>, ys: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(is_ascending(xs.as_slice().unwrap()));
        debug_assert!(ys.len() == xs.len());

        Self::ConstantSpline { xs, ys }
    }

    /// Construct a linear spline instance.
    #[inline]
    #[must_use]
    pub fn new_linear_spline(xs: Array1<f64>, ys: Array1<f64>, grads: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(is_ascending(xs.as_slice().unwrap()));
        debug_assert!(ys.len() == xs.len());
        debug_assert!((grads.len() + 1) == xs.len());

        Self::LinearSpline { xs, ys, grads }
    }

    /// Construct a linear spline instance.
    #[inline]
    #[must_use]
    pub fn new_linear_spline_auto(xs: Array1<f64>, ys: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(is_ascending(xs.as_slice().unwrap()));
        debug_assert!(ys.len() == xs.len());

        let mut grads = Vec::with_capacity(xs.len() - 1);
        for ((x_curr, x_next), (y_curr, y_next)) in xs
            .iter()
            .zip(xs.iter().skip(1))
            .zip(ys.iter().zip(ys.iter().skip(1)))
        {
            grads.push((y_next - y_curr) / (x_next - x_curr));
        }

        Self::new_linear_spline(xs, ys, Array1::from(grads))
    }

    /// Construct a quadratic spline instance.
    #[inline]
    #[must_use]
    pub fn new_quadratic_spline(
        xs: Array1<f64>,
        ys: Array1<f64>,
        grads: Array1<f64>,
        quads: Array1<f64>,
    ) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(is_ascending(xs.as_slice().unwrap()));
        debug_assert!(ys.len() == xs.len());
        debug_assert!((grads.len() + 1) == xs.len());
        debug_assert!((quads.len() + 1) == xs.len());

        Self::QuadraticSpline {
            xs,
            ys,
            grads,
            quads,
        }
    }

    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, x: f64) -> f64 {
        match *self {
            Self::Constant { ref c } => *c,
            Self::Line { c, m } => x.mul_add(m, c),
            Self::Bifurcation {
                ref t,
                ref under,
                ref over,
            } => {
                if x < *t {
                    *under
                } else {
                    *over
                }
            }
            Self::ConstantSpline { ref xs, ref ys } => {
                debug_assert!(x >= xs[0]);
                debug_assert!(x <= xs[xs.len() - 1]);

                for (index, xn) in xs.iter().enumerate() {
                    if *xn > x {
                        return ys[index - 1];
                    }
                }
                ys[ys.len() - 1]
            }
            Self::LinearSpline {
                ref xs,
                ref ys,
                ref grads,
            } => {
                debug_assert!(x >= xs[0]);
                debug_assert!(x <= xs[xs.len() - 1]);

                for (index, xn) in xs.iter().enumerate() {
                    if *xn > x {
                        let dx = x - xs[index - 1];
                        return grads[index - 1].mul_add(dx, ys[index - 1]);
                    }
                }
                ys[ys.len() - 1]
            }
            Self::QuadraticSpline {
                ref xs,
                ref ys,
                ref grads,
                ref quads,
            } => {
                debug_assert!(x >= xs[0]);
                debug_assert!(x <= xs[xs.len() - 1]);

                for (index, xn) in xs.iter().enumerate() {
                    if *xn > x {
                        let dx = x - xs[index - 1];
                        let y = ys[index - 1];
                        let g = grads[index - 1];
                        let q = quads[index - 1];
                        return q.mul_add(dx * dx, g.mul_add(dx, y));
                    }
                }
                ys[ys.len() - 1]
            }
        }
    }
}

impl Display for Formula {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let kind = match *self {
            Self::Constant { .. } => "Constant",
            Self::Line { .. } => "Line",
            Self::Bifurcation { .. } => "Bifurcation",
            Self::ConstantSpline { .. } => "Constant Spline",
            Self::LinearSpline { .. } => "Linear Spline",
            Self::QuadraticSpline { .. } => "Quadrati Spline",
        };
        write!(fmt, "{}", kind)
    }
}
