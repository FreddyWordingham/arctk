//! Chemical reaction implementation.

use crate::kinetics::{Chem, Rate};
use ndarray::Array1;

/// Chemical reaction.
pub struct Reaction {
    /// Stoichiometric coefficent map.
    coeffs: Array1<f64>,
    /// Reaction rate.
    rate: Rate,
}

impl Reaction {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        reactants: Vec<(i32, Chem)>,
        products: Vec<(i32, Chem)>,
        rate: Rate,
        num_chems: usize,
    ) -> Self {
        debug_assert!(!reactants.is_empty());
        debug_assert!(num_chems > 0);

        let mut coeffs = Array1::zeros(num_chems);
        for (s, c) in reactants {
            coeffs[c] -= s as f64;
        }

        for (s, c) in products {
            coeffs[c] += s as f64;
        }

        Self { coeffs, rate }
    }

    /// Determine the rate of change for each chemical within the system.
    #[inline]
    #[must_use]
    pub fn rate(&self, concs: &Array1<f64>) -> Array1<f64> {
        self.rate.rate(concs) * &self.coeffs
    }
}
