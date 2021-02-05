//! Probability distribution implementation.

use crate::math::{distribution, Formula};
use ndarray::Array1;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

/// Probability distribution formulae.
#[derive(Clone)]
pub enum Probability {
    /// Point.
    Point {
        /// Constant value.
        c: f64,
    },
    /// Points.
    Points {
        /// Possible values.
        cs: Array1<f64>,
    },
    /// Uniform range.
    Uniform {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
    },
    /// Gaussian distribution.
    Gaussian {
        /// Average value.
        mu: f64,
        /// Variance.
        sigma: f64,
    },
    /// Constant spline.
    ConstantSpline {
        /// Cumulative distribution function.
        cdf: Formula,
    },
}

impl Probability {
    /// Construct a new point instance.
    #[inline]
    #[must_use]
    pub const fn new_point(c: f64) -> Self {
        Self::Point { c }
    }

    /// Construct a new points instance.
    #[inline]
    #[must_use]
    pub fn new_points(cs: Array1<f64>) -> Self {
        debug_assert!(cs.len() > 1);
        Self::Points { cs }
    }

    /// Construct a new uniform instance.
    #[inline]
    #[must_use]
    pub fn new_uniform(min: f64, max: f64) -> Self {
        debug_assert!(min < max);
        Self::Uniform { min, max }
    }

    /// Construct a new gaussian instance.
    #[inline]
    #[must_use]
    pub fn new_gaussian(mu: f64, sigma: f64) -> Self {
        debug_assert!(sigma > 0.0);
        Self::Gaussian { mu, sigma }
    }

    /// Construct a new constant spline instance.
    #[inline]
    #[must_use]
    pub fn new_constant_spline(xs: Array1<f64>, ps: &Array1<f64>) -> Self {
        debug_assert!(xs.len() > 1);
        debug_assert!(xs.len() == (ps.len() + 1));
        debug_assert!(ps.iter().all(|p| *p >= 0.0));

        let mut cdf = Vec::with_capacity(xs.len());
        let mut total = 0.0;
        cdf.push(total);
        for ((x_curr, x_next), prob) in xs.iter().zip(xs.iter().skip(1)).zip(ps.iter()) {
            let area = (x_next - x_curr) * prob;
            total += area;
            cdf.push(total);
        }
        let mut cdf = Array1::from(cdf);
        cdf /= total;

        Self::ConstantSpline {
            cdf: Formula::new_linear_spline_auto(cdf, xs),
        }
    }

    /// Sample a number from the described distribution.
    #[inline]
    #[must_use]
    pub fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        match *self {
            Self::Point { ref c } => *c,
            Self::Points { ref cs } => cs[rng.gen_range(0..cs.len())],
            Self::Uniform { ref min, ref max } => rng.gen_range(*min..*max),
            Self::Gaussian { ref mu, ref sigma } => distribution::sample_gaussian(rng, *mu, *sigma),
            Self::ConstantSpline { ref cdf } => cdf.y(rng.gen()),
        }
    }
}

impl Display for Probability {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match *self {
            Self::Point { .. } => "Point",
            Self::Points { .. } => "Points",
            Self::Uniform { .. } => "Uniform",
            Self::Gaussian { .. } => "Gaussian",
            Self::ConstantSpline { .. } => "Constant Spline",
        };
        write!(fmt, "{}", kind)
    }
}
