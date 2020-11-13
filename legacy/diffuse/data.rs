//! Output structure.

use crate::{err::Error, file::Save};
use ndarray::Array3;
use std::path::Path;

/// Diffusion output data.
pub struct Data {
    /// Current rate values.
    pub rates: Array3<f64>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(rates: Array3<f64>) -> Self {
        debug_assert!(!rates.is_empty());

        Self { rates }
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let p = out_dir.join(format!("{:06}ms.nc", (self.time * 1000.0) as u32));
        println!("Saving: {}", p.display());
        self.rates.save(&p)
    }
}
