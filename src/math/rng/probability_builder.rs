//! Probability builder.

use crate::{math::Probability, ord::Build};
use arctk_attr::file;
use ndarray::Array1;
use std::fmt::{Display, Error, Formatter};

/// Probability distribution builders.
#[file]
pub enum ProbabilityBuilder {
    /// Point.
    Point(f64),
    /// Points.
    Points(Array1<f64>),
    /// Uniform range.
    Uniform(f64, f64),
    /// Linear function.
    Linear(f64, f64, f64, f64),
    /// Gaussian distribution.
    Gaussian(f64, f64),
    /// Constant spline.
    ConstantSpline(Vec<f64>, Vec<f64>),
}

impl Build for ProbabilityBuilder {
    type Inst = Probability;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Point(p) => Self::Inst::new_point(p),
            Self::Points(ps) => Self::Inst::new_points(ps),
            Self::Uniform(min, max) => Self::Inst::new_uniform(min, max),
            Self::Linear(min, max, m, c) => Self::Inst::new_linear(min, max, m, c),
            Self::Gaussian(mu, sigma) => Self::Inst::new_gaussian(mu, sigma),
            Self::ConstantSpline(xs, ps) => {
                Self::Inst::new_constant_spline(Array1::from(xs), &Array1::from(ps))
            }
        }
    }
}

impl Display for ProbabilityBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let kind = match *self {
            Self::Point { .. } => "Constant",
            Self::Points { .. } => "Line",
            Self::Uniform { .. } => "Bifurcation",
            Self::Linear { .. } => "Linear",
            Self::Gaussian { .. } => "Gaussian",
            Self::ConstantSpline { .. } => "Constant Spline",
        };
        write!(fmt, "{}", kind)
    }
}
