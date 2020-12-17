//! Output data structure.

use crate::{
    err::Error,
    file::Save,
    ord::{X, Y, Z},
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Cartographer output data.
pub struct Output {
    /// Missing materials.
    pub void: Array3<f64>,
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
            void: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.void += &rhs.void;
    }
}

impl Save for Output {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        let max = self.void.len() as f64;
        println!("void\t{}%", self.void.sum() / max * 100.0);
        self.void.save(&out_dir.join("map_void.nc"))
    }
}
