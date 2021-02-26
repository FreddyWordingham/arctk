//! Rate structure.

use ndarray::ArrayView1;
use std::fmt::{Display, Formatter};

/// Rate of reaction.
#[derive(Debug, Clone)]
pub struct Rate {
    /// Reaction rate constant.
    k: f64,
    /// List of each index and its associated partial order of reaction.
    orders: Vec<(usize, f64)>,
}

impl Rate {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(k: f64, orders: Vec<(usize, f64)>) -> Self {
        debug_assert!(k > 0.0);
        debug_assert!(!orders.is_empty());

        Self { k, orders }
    }

    /// Get the total order of the reaction.
    #[inline]
    #[must_use]
    pub fn order(&self) -> f64 {
        let mut p = 0.0;
        for &(_, c) in &self.orders {
            p += c;
        }
        p
    }

    /// Calculate the current rate given the current concentrations.
    #[inline]
    #[must_use]
    pub fn rate(&self, concs: &ArrayView1<f64>) -> f64 {
        let mut r = self.k;

        for &(c, m) in &self.orders {
            r *= concs[c].powf(m);
        }

        r
    }

    /// Calculate the current rate given the current concentrations.
    #[inline]
    #[must_use]
    pub fn rate_m(&self, concs: &ArrayView1<f64>) -> f64 {
        let mut r = self.k;

        for &(c, m) in &self.orders {
            r *= concs[c].powf(m);
        }

        r
    }
}

impl Display for Rate {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.k)?;

        if !self.orders.is_empty() {
            write!(fmt, " *")?;
            for &(c, m) in &self.orders {
                write!(fmt, " {}^{}", c, m)?;
            }
        }

        Ok(())
    }
}
