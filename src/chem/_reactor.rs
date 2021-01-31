//! Reactor structure.

use crate::{chem::Rate, fmt_report};
use ndarray::{Array1, Array2, Array4, Axis};
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

    /// Calculate the reaction rates.
    #[inline]
    fn rates_m(&self, values: &Array4<f64>, rates: &mut Array4<f64>) {
        let [rs, rx, ry, rz] = [
            values.shape()[0],
            values.shape()[1],
            values.shape()[2],
            values.shape()[3],
        ];

        for zi in 0..rz {
            for yi in 0..ry {
                for xi in 0..rx {
                    let arr = values.index_axis(Axis(0), xi);
                    let arr = arr.index_axis(Axis(1), yi);
                    let arr = arr.index_axis(Axis(2), zi);

                    for si in 0..rs {
                        rates[[si, xi, yi, zi]] = self.rates[si].rate_m(&arr);
                    }
                }
            }
        }
    }

    /// Calculate the overall rate of change given the current concentrations.
    #[inline]
    #[must_use]
    pub fn deltas_m(&self, mut values: Array4<f64>, rates: &mut Array4<f64>) -> Array4<f64> {
        self.rates_m(&values, rates);
        values += &(&coeffs * *rate);

        values
    }
}

impl Display for Reactor {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        for (i, r) in self.rates.iter().enumerate() {
            fmt_report!(fmt, r, format!("k{} ([C]^{} s^-1) ->", i, -r.order()));
        }
        for (i, k) in self.coeffs.outer_iter().enumerate() {
            fmt_report!(fmt, k, format!("r{} ->", i));
        }
        Ok(())
    }
}
