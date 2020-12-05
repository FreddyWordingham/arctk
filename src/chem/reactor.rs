//! Reactor structure.

use crate::{
    chem::{Rate, Reaction},
    ord::Register,
};
use ndarray::{Array1, Array2};

/// Complete reactor structure.
#[derive(Debug)]
pub struct Reactor {
    /// Rates.
    rates: Array1<Rate>,
    /// Stoichiometric coefficents.
    coeffs: Array2<f64>,
}

impl Reactor {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reg: &Register, reacts: Vec<Reaction>) -> Self {
        debug_assert!(!reacts.is_empty());

        let mut rates = Vec::with_capacity(reacts.len());
        let mut coeffs = Array2::zeros([reacts.len(), reg.len()]);
        for (mut coeff_set, react) in coeffs.outer_iter_mut().zip(reacts) {
            let (r, cs) = react.components();
            rates.push(r);
            coeff_set += &cs;
        }

        Self {
            rates: Array1::from(rates),
            coeffs,
        }
    }

    /// Calculate the reaction rates.
    #[inline]
    #[must_use]
    fn rates(&self, concs: &Array1<f64>) -> Array1<f64> {
        debug_assert!(concs.len() == self.coeffs.ncols());

        self.rates.iter().map(|r| r.rate(concs)).collect()
    }

    /// Calculate the overall rate of change given the current concentrations.
    #[inline]
    #[must_use]
    pub fn deltas(&self, concs: &Array1<f64>) -> Array1<f64> {
        debug_assert!(concs.len() == self.coeffs.ncols());

        let rates = self.rates(concs);

        let mut deltas = Array1::zeros(concs.len());
        for (coeffs, rate) in self.coeffs.outer_iter().zip(&rates) {
            deltas += &(&coeffs * *rate);
        }

        deltas
    }
}
