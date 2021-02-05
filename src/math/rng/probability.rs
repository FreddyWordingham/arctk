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
    /// Linear.
    Linear {
        /// Gradient.
        grad: f64,
        /// Y-intercept.
        intercept: f64,
        /// Integration constant offset.
        offset: f64,
        /// Area beneath line in range.
        area: f64,
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
    /// Linear spline.
    LinearSpline {
        /// Gradients.
        grads: Array1<f64>,
        /// Y-intercepts.
        intercepts: Array1<f64>,
        /// Integration constant offsets.
        offsets: Array1<f64>,
        /// Area beneath line in each range.
        areas: Array1<f64>,
        /// Cumulative distribution function.
        cdf: Array1<f64>,
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

    /// Construct a new linear instance.
    #[inline]
    #[must_use]
    pub fn new_linear([x0, x1]: [f64; 2], [p0, p1]: [f64; 2]) -> Self {
        let dx = x1 - x0;
        let dp = p1 - p0;

        let grad = dp / dx;
        let intercept = p0 - (grad * x0);
        let offset = if x0 < x1 {
            (0.5 * grad * x0).mul_add(x0, intercept * x0)
        } else {
            (0.5 * grad * x1).mul_add(x1, intercept * x1)
        };

        let area = 0.5 * (p0 + p1) * (x1 - x0);

        Self::Linear {
            grad,
            intercept,
            offset,
            area,
        }
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

    /// Construct a new linear spline instance.
    #[inline]
    #[must_use]
    pub fn new_linear_spline(xs: Array1<f64>, ps: &Array1<f64>) -> Self {
        debug_assert!(xs.len() > 1);
        debug_assert!(xs.len() == ps.len());
        debug_assert!(ps.iter().all(|p| *p >= 0.0));

        let mut grads = Vec::with_capacity(xs.len() - 1);
        let mut intercepts = Vec::with_capacity(xs.len() - 1);
        let mut offsets = Vec::with_capacity(xs.len() - 1);
        let mut areas = Vec::with_capacity(xs.len() - 1);
        let mut cdf = Vec::with_capacity(xs.len());
        let mut total = 0.0;
        for ((x_curr, x_next), (p_curr, p_next)) in xs
            .iter()
            .zip(xs.iter().skip(1))
            .zip(ps.iter().zip(ps.iter().skip(1)))
        {
            let dx = *x_next - *x_curr;
            let dp = *p_next - *p_curr;

            let grad = dp / dx;
            let intercept = *p_curr - (grad * *x_curr);
            let offset = if *x_curr < *x_next {
                (0.5 * grad * *x_curr).mul_add(*x_curr, intercept * *x_curr)
            } else {
                (0.5 * grad * *x_next).mul_add(*x_next, intercept * *x_next)
            };
            let area = 0.5 * (*p_curr + *p_next) * (*x_next - *x_curr);

            grads.push(grad);
            intercepts.push(intercept);
            offsets.push(offset);
            areas.push(area);

            total += area;
            cdf.push(total);
        }
        let mut cdf = Array1::from(cdf);
        cdf /= total;

        Self::LinearSpline {
            grads: Array1::from(grads),
            intercepts: Array1::from(intercepts),
            offsets: Array1::from(offsets),
            areas: Array1::from(areas),
            cdf,
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
            Self::Linear {
                grad,
                intercept,
                offset,
                area,
            } => {
                let r = rng.gen_range(0.0..1.0_f64);
                ((2.0 * grad)
                    .mul_add(r.mul_add(area, offset), intercept * intercept)
                    .sqrt()
                    - intercept)
                    / grad
            }
            Self::Gaussian { ref mu, ref sigma } => distribution::sample_gaussian(rng, *mu, *sigma),
            Self::ConstantSpline { ref cdf } => cdf.y(rng.gen()),
            Self::LinearSpline {
                ref grads,
                ref intercepts,
                ref offsets,
                ref areas,
                ref cdf,
            } => {
                let a = rng.gen_range(0.0..1.0);
                for (index, c) in cdf.iter().enumerate() {
                    if a < *c {
                        let grad = grads[index];
                        let intercept = intercepts[index];
                        let offset = offsets[index];
                        let area = areas[index];

                        let r = rng.gen_range(0.0..1.0_f64);
                        return ((2.0 * grad)
                            .mul_add(r.mul_add(area, offset), intercept * intercept)
                            .sqrt()
                            - intercept)
                            / grad;
                    }
                }
                0.0
            }
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
            Self::Linear { .. } => "Linear",
            Self::Gaussian { .. } => "Gaussian",
            Self::ConstantSpline { .. } => "Constant Spline",
            Self::LinearSpline { .. } => "Linear Spline",
        };
        write!(fmt, "{}", kind)
    }
}
