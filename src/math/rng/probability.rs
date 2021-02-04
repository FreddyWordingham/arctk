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
    /// Linear function.
    Linear {
        // Gradient.
        m: f64,
        // Offset.
        c: f64,
        // Integration constant.
        d: f64,
        // Area normalisation.
        a: f64,
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
        /// Base values.
        xs: Array1<f64>,
        grads: Array1<f64>,
        intercepts: Array1<f64>,
        ds: Array1<f64>,
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
    pub fn new_linear(min: f64, max: f64, grad: f64, offset: f64) -> Self {
        debug_assert!(grad != 0.0);

        let m = grad;
        let c = offset;
        let d = -((0.5 * m * min * min) + (c * min));

        let min_y = (m * min) + c;
        let max_y = (m * max) + c;
        let a = 0.5 * (max - min) * (max_y + min_y);

        Self::Linear { m, c, d, a }
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

        let mut cdf = Vec::with_capacity(xs.len());
        let mut grads = Vec::with_capacity(xs.len() - 1);
        let mut intercepts = Vec::with_capacity(xs.len() - 1);
        let mut ds = Vec::with_capacity(xs.len() - 1);
        let mut total = 0.0;
        cdf.push(total);
        for ((x_curr, x_next), (p_curr, p_next)) in xs
            .iter()
            .zip(xs.iter().skip(1))
            .zip(ps.iter().zip(ps.iter().skip(1)))
        {
            let area = 0.5 * (x_next - x_curr) * (p_next + p_curr);
            total += area;
            cdf.push(total);

            let grad = (p_next - p_curr) / (x_next - x_curr);
            grads.push(grad);

            let intercept = -grad * x_curr;
            intercepts.push(intercept);

            let d = (0.5 * grad * x_curr) + (intercept * x_curr);
            ds.push(d);
        }
        let mut cdf = Array1::from(cdf);
        cdf /= total;

        let grads = Array1::from(grads);
        let intercepts = Array1::from(intercepts);
        let ds = Array1::from(ds);

        crate::report!(xs);
        crate::report!(grads);
        crate::report!(intercepts);
        crate::report!(ds);
        crate::report!(cdf);

        Self::LinearSpline {
            xs,
            grads,
            intercepts,
            ds,
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
            Self::Linear { m, c, d, a } => {
                let r = rng.gen_range(0.0..1.0);
                let ans = (((2.0 * a * m * r) + (c * c)).sqrt() - c) / m;
                println!("{}\t{}\t{}\t{}\t{}\t->\t{}", m, c, d, a, r, ans);
                ans
            }
            Self::Gaussian { ref mu, ref sigma } => distribution::sample_gaussian(rng, *mu, *sigma),
            Self::ConstantSpline { ref cdf } => cdf.y(rng.gen()),
            Self::LinearSpline {
                ref xs,
                ref grads,
                ref intercepts,
                ref ds,
                ref cdf,
            } => {
                let r = rng.gen_range(0.0..1.0);
                for (index, c) in cdf.iter().enumerate() {
                    if *c > r {
                        let x = xs[index - 1];
                        let m = grads[index - 1];
                        let d = ds[index - 1];

                        let ans = (((d * d) + (2.0 * m * (x - r))).sqrt() - d) / m;
                        println!("{}\t->\t{}", r, ans);

                        return ans;
                    }
                }
                xs[xs.len() - 1]
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
