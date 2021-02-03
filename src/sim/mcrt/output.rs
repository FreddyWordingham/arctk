//! Output data structure.

use crate::{
    access, clone,
    data::Histogram,
    err::Error,
    fmt_report,
    fs::Save,
    geom::Cube,
    ord::{Register, X, Y, Z},
    util::fmt::DataCube,
};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// MCRT output data.
pub struct Output<'a> {
    /// Spectrometer name register.
    spec_reg: &'a Register,
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
    /// Spectrometer data.
    pub specs: Vec<Histogram>,
}

impl<'a> Output<'a> {
    access!(boundary, Cube);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        spec_reg: &'a Register,
        boundary: Cube,
        res: [usize; 3],
        specs: Vec<Histogram>,
    ) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self {
            spec_reg,
            boundary,
            cell_vol,
            emission: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),
            specs,
        }
    }
}

impl AddAssign<&Self> for Output<'_> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emission += &rhs.emission;
        self.energy += &rhs.energy;
        self.absorptions += &rhs.absorptions;
        self.shifts += &rhs.shifts;

        for (a, b) in self.specs.iter_mut().zip(&rhs.specs) {
            *a += b;
        }
    }
}

impl Save for Output<'_> {
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

        // let path = out_dir.join("histogram.csv");
        // self.hist.save(&path)?;

        Ok(())
    }
}

impl Display for Output<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.spec_reg, "spectrometer register");
        fmt_report!(fmt, self.boundary, "boundary");
        fmt_report!(fmt, self.cell_vol, "cell volume (m^3)");
        fmt_report!(fmt, DataCube::new(&self.emission), "emission data");
        fmt_report!(fmt, DataCube::new(&self.energy), "energy data");
        fmt_report!(
            fmt,
            DataCube::new(&self.absorptions),
            "absorbed energy data"
        );
        fmt_report!(fmt, DataCube::new(&self.shifts), "shifted energy data");
        Ok(())
    }
}
