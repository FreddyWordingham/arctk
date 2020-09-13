//! Formula form implementation.

use crate::{err::Error, file::Build};
use arctk_attr::load;
use ndarray::Array1;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Mathematical formulae accepting a single scalar argument.
#[load]
pub enum FormulaBuilder {
    /// Constant value. = c
    Constant(f64),
    /// Line formula. = (x * m) + c
    Line(f64, f64),
    /// Bifurcation formula. = x < y ? a : b.
    Bifurcation(f64, f64, f64),
    /// Constant value spline.
    ConstantSpline(Vec<f64>, Vec<f64>),
    /// Linear spline between points.
    LinearSpline(Vec<f64>, Vec<f64>, Vec<f64>),
    /// Connected linear spline between points.
    LinearSplineAuto(Vec<f64>, Vec<f64>),
    /// Quadratic spline between points.
    QuadraticSpline(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>),
}

impl Build for FormulaBuilder {
    type Inst = crate::math::Formula;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Constant(c) => Self::Inst::Constant { c },
            Self::Line(c, m) => Self::Inst::Line { c, m },
            Self::Bifurcation(t, under, over) => Self::Inst::Bifurcation { t, under, over },
            Self::ConstantSpline(xs, ys) => {
                Self::Inst::new_constant_spline(Array1::from(xs), Array1::from(ys))
            }
            Self::LinearSpline(xs, ys, grads) => Self::Inst::new_linear_spline(
                Array1::from(xs),
                Array1::from(ys),
                Array1::from(grads),
            ),
            Self::LinearSplineAuto(xs, ys) => {
                Self::Inst::new_linear_spline_auto(Array1::from(xs), Array1::from(ys))
            }
            Self::QuadraticSpline(xs, ys, grads, quads) => Self::Inst::new_quadratic_spline(
                Array1::from(xs),
                Array1::from(ys),
                Array1::from(grads),
                Array1::from(quads),
            ),
        })
    }
}

impl Display for FormulaBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let kind = match self {
            Self::Constant { .. } => "Constant",
            Self::Line { .. } => "Line",
            Self::Bifurcation { .. } => "Bifurcation",
            Self::ConstantSpline { .. } => "Constant Spline",
            Self::LinearSpline { .. } => "Linear Spline",
            Self::LinearSplineAuto { .. } => "Linear Spline [auto]",
            Self::QuadraticSpline { .. } => "Quadratic Spline",
        };
        write!(fmt, "{}", kind)
    }
}
