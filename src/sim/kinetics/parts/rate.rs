//! Chemical reaction rate implementation.

use crate::kinetics::Chem;

/// Chemical reaction rate.
pub struct Rate {
    /// Reaction rate constant.
    k: f64,
    /// Partial orders of reaction.
    orders: Vec<(Chem, f64)>,
}

impl Rate {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(k: f64, orders: Vec<(Chem, f64)>) -> Self {
        debug_assert!(k > 0.0);

        Self { k, orders }
    }

    /// Calculate the current rate given the current concentrations.
    #[inline]
    #[must_use]
    pub fn rate(&self, concs: &[f64]) -> f64 {
        let mut r = self.k;

        for (c, m) in &self.orders {
            r *= concs[*c].powf(*m)
        }

        r
    }
}
