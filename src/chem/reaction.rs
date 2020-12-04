//! Reaction structure.

use crate::{access, chem::Rate};
use ndarray::Array1;

/// Reaction.
#[derive(Clone)]
pub struct Reaction {
    // /// Reaction rate.
    // rate: Rate,
    /// Stoichiometric coefficient map.
    coeffs: Array1<f64>,
}

impl Reaction {
    access!(coeffs, Array1<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        // rate: Rate,
        coeffs: Array1<f64>,
    ) -> Self {
        debug_assert!(!coeffs.is_empty());

        Self {
            //  rate,
            coeffs,
        }
    }

    // /// Determine the rate of change for each chemical within the system.
    // #[inline]
    // #[must_use]
    // pub fn rate(&self, concs: &Array1<f64>) -> Array1<f64> {
    //     self.rate.rate(concs) * &self.coeffs
    // }

    // /// Separate into components.
    // #[allow(clippy::missing_const_for_fn)]
    // #[inline]
    // #[must_use]
    // pub fn components(self) -> (Rate, Array1<f64>) {
    //     (self.rate, self.coeffs)
    // }
}
