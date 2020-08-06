//! Output structure.

use crate::{Error, Save, X, Y, Z};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Distance travelled .
    dist_travelled: Array3<f64>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self {
            dist_travelled: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.dist_travelled += &rhs.dist_travelled;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // Get current time string.
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let path = out_dir.join(time);
        std::fs::create_dir(&path)?;

        let p = out_dir.join("dist_travelled.nc");
        println!("Saving: {}", p.display());
        self.dist_travelled.save(&p)
    }
}
