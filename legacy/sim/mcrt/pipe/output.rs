//! Output data structure.

use crate::{
    access, clone,
    err::Error,
    file::Save,
    geom::Cube,
    ord::{X, Y, Z},
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// MCRT output data.
pub struct Output {
    /// Measured volume.
    boundary: Cube,
    /// Cell volume [m^3].
    cell_vol: f64,
    /// Emission power.
    pub emission: Array3<f64>,
    /// Photo-energy.
    pub energy: Array3<f64>,
    /// Absorptions.
    pub absorptions: Array3<f64>,
    /// Wavelength shifts.
    pub shifts: Array3<f64>,
}

impl Output {
    access!(boundary, Cube);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self {
            boundary,
            cell_vol,
            emission: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emission += &rhs.emission;
        self.energy += &rhs.energy;
        self.absorptions += &rhs.absorptions;
        self.shifts += &rhs.shifts;
    }
}

impl Save for Output {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("emission_density.nc");
        (&self.emission / self.cell_vol).save(&path)?;

        let path = out_dir.join("energy_density.nc");
        (&self.energy / self.cell_vol).save(&path)?;

        let path = out_dir.join("absorption_density.nc");
        (&self.absorptions / self.cell_vol).save(&path)?;

        let path = out_dir.join("shift_density.nc");
        (&self.shifts / self.cell_vol).save(&path)?;

        Ok(())
    }
}
