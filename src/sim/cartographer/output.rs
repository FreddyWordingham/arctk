//! Output data structure.

use crate::{
    err::Error,
    fmt_report,
    fs::Save,
    ord::{Register, X, Y, Z},
    util::datacube::display_datacube,
};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Cartographer output data.
pub struct Output<'a> {
    /// Material name register.
    pub mat_reg: &'a Register,
    /// Missing materials.
    pub void: Array3<f64>,
    /// Occupying materials.
    pub mats: Vec<Array3<f64>>,
}

impl<'a> Output<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mat_reg: &'a Register, res: [usize; 3]) -> Self {
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

impl AddAssign<&Self> for Output<'_> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.void += &rhs.void;
        for (l, r) in self.mats.iter_mut().zip(&rhs.mats) {
            *l += r;
        }
    }
}

impl Save for Output<'_> {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        for (name, index) in self.mat_reg.set().map().iter() {
            self.mats[*index].save(&out_dir.join(&format!("map_{}.nc", name)))?;
        }
        self.void.save(&out_dir.join("map_void.nc"))
    }
}

impl Display for Output<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        let max = self.void.len() as f64;
        for (name, index) in self.mat_reg.set().map().iter() {
            let map = &self.mats[*index];
            fmt_report!(fmt, index, &format!("{} data", name));
            fmt_report!(fmt, map.sum() / map.len() as f64 * 100.0, "volume (%)");
            fmt_report!(fmt, name, display_datacube(fmt, map)?);
        }
        writeln!(fmt, "*VOID* data...")?;
        // fmt_report!(fmt, "void\t{}%", self.void.sum() / max * 100.0);
        display_datacube(fmt, &self.void)?;
        Ok(())
    }
}
