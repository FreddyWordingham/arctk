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
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.escaped_weight += &rhs.escaped_weight;
        self.emission_power += &rhs.emission_power;
    }
}

impl Display for Data {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "cell volume", self.cell_vol, "m^3")?;
        display_field_ln!(fmt, "escaped weight", self.escaped_weight)?;
        display_field!(fmt, "total emission power", self.emission_power.sum())
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // Get current time string.
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let path = out_dir.join(time);
        std::fs::create_dir(&path)?;

        let p = out_dir.join("emission_power_density.nc");
        println!("Saving: {}", p.display());
        (&self.emission_power / self.cell_vol).save(&p)
    }
}
