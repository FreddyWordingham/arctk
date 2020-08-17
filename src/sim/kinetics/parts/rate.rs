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
        Self { k, orders }
    }
}
