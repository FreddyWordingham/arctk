//! MCRT output structure.

use crate::{access, clone, display_field, display_field_ln, Aabb, Error, Save, X, Y, Z};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// MCRT output structure.
pub struct Data {
    /// Measured volume.
    boundary: Aabb,
    /// Cell volume [m^3].
    cell_vol: f64,
    /// Escaped weight.
    pub escaped_weight: f64,
    /// Emission power.
    pub emission_power: Array3<f64>,
    /// Local photo-energy [J].
    pub energy: Array3<f64>,
    /// Local photo-absorptions [J].
    pub absorptions: Array3<f64>,
    /// Local photo-shifts [J].
    pub shifts: Array3<f64>,
}

impl Data {
    access!(boundary, Aabb);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Aabb, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self {
            boundary,
            cell_vol,
            escaped_weight: 0.0,
            emission_power: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.escaped_weight += &rhs.escaped_weight;
        self.emission_power += &rhs.emission_power;
        self.energy += &rhs.energy;
        self.absorptions += &rhs.absorptions;
        self.shifts += &rhs.shifts;
    }
}

impl Display for Data {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "cell volume", self.cell_vol, "m^3")?;
        display_field_ln!(fmt, "escaped weight", self.escaped_weight)?;
        display_field_ln!(
            fmt,
            "total emission power",
            self.emission_power.sum(),
            "J/s"
        )?;
        display_field_ln!(fmt, "total photo-energy", self.energy.sum(), "J")?;
        display_field_ln!(fmt, "total photo-absorptions", self.absorptions.sum(), "J")?;
        display_field!(fmt, "total photo-shifts", self.shifts.sum(), "J")
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("emission_power_density.nc");
        println!("Saving: {}", path.display());
        (&self.emission_power / self.cell_vol).save(&path)?;

        let path = out_dir.join("energy_density.nc");
        println!("Saving: {}", path.display());
        (&self.energy / self.cell_vol).save(&path)?;

        let path = out_dir.join("absorption_density.nc");
        println!("Saving: {}", path.display());
        (&self.absorptions / self.cell_vol).save(&path)?;

        let path = out_dir.join("shift_density.nc");
        println!("Saving: {}", path.display());
        (&self.shifts / self.cell_vol).save(&path)
    }
}
