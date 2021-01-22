//! Reactor structure.

use crate::{chem::Rate, fmt_report};
use ndarray::{Array1, Array2};
use std::fmt::{Display, Error, Formatter};

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
    pub fn new(rates: Array1<Rate>, coeffs: Array2<f64>) -> Self {
        debug_assert!(!rates.is_empty());
        debug_assert!(rates.len() == coeffs.nrows());

        Self { rates, coeffs }
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

impl Display for Reactor {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        for (i, r) in self.rates.iter().enumerate() {
            fmt_report!(fmt, r, format!("r{} ([C]^{} s^-1) ->", i, -r.order()));
        }
        for (i, k) in self.coeffs.outer_iter().enumerate() {
            fmt_report!(fmt, k, format!("k{} ->", i));
        }
        Ok(())
    }
}
