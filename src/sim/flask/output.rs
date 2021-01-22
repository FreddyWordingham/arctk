//! Output data structure.

use crate::{
    // data::Table,
    err::Error,
    fmt_report,
    fmt_reports,
    fs::Save,
    ord::{Name, Register},
};
use ndarray::Array2;
use std::fmt::{Display, Formatter};
use std::{ops::AddAssign, path::Path};

/// Flask output data.
pub struct Output {
    /// Total integration time.
    total_time: f64,
    /// Species names.
    names: Vec<Name>,
    /// Recorded values.
    values: Array2<f64>,
}

impl Output {
    /// Get the data time-step.
    #[inline]
    #[must_use]
    pub fn dt(&self) -> f64 {
        self.total_time / self.values.nrows().max(1) as f64
    }
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(total_time: f64, reg: Register, num_steps: usize) -> Self {
        debug_assert!(total_time > 0.0);
        debug_assert!(num_steps > 0);

        Self {
            total_time,
            names: reg.names_list(),
            values: Array2::zeros([reg.len(), num_steps]),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.values += &rhs.values;
    }
}

impl Save for Output {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        let _path = out_dir.join("values_over_time.nc");
        println!("TODO! SAVE VALUES TO CSV");

        Ok(())
    }
}

impl Display for Output {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.total_time, "total integration time (s)");
        fmt_reports!(fmt, &self.names, "species");

        let mut t = 0.0;
        let dt = self.dt();
        for _row in self.values.outer_iter() {
            println!("t: {}", t);
            t += dt;
        }

        Ok(())
    }
}
