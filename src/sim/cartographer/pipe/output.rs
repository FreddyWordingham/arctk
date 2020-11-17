//! Output data structure.

use crate::{
    err::Error,
    file::Save,
    ord::{Register, X, Y, Z},
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Cartographer output data.
pub struct Output<'a> {
    /// Material name register.
    pub mat_reg: &'a Register,
    /// Occupying materials.
    pub mats: Vec<Array3<f64>>,
    /// Missing materials.
    pub void: Array3<f64>,
}

impl<'a> Output<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mat_reg: &'a Register, res: [usize; 3]) -> Self {
        debug_assert!(mat_reg.len() > 0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let num_mats = mat_reg.len();
        let mut mats = Vec::with_capacity(num_mats);
        for _ in 0..num_mats {
            mats.push(Array3::zeros(res));
        }

        Self {
            mat_reg,
            mats,
            void: Array3::zeros(res),
        }
    }
}

impl<'a> AddAssign<&Self> for Output<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!(self.mat_reg == rhs.mat_reg);

        self.void += &rhs.void;

        for (l, r) in self.mats.iter_mut().zip(&rhs.mats) {
            *l += r;
        }
    }
}

impl<'a> Save for Output<'a> {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}
