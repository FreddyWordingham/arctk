//! Rate structure.

use crate::{fmt_report, fmt_reports};
use ndarray::Array1;
use std::fmt::{Display, Error, Formatter};

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

    /// Calculate the current rate given the current concentrations.
    #[inline]
    #[must_use]
    pub fn rate(&self, concs: &Array1<f64>) -> f64 {
        let mut r = self.k;

        for &(c, m) in &self.orders {
            r *= concs[c].powf(m);
        }

        r
    }
}

impl Display for Rate {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        let power = self.orders.len();
        fmt_report!(fmt, self.k, &format!("rate ([C]^{} s^-1)", -(power as i32)));

        let mut orders = Vec::with_capacity(power);
        for &(c, m) in &self.orders {
            orders.push(format!("[{}]^{}", c, m));
        }
        fmt_reports!(fmt, orders, "orders");

        Ok(())
    }
}
