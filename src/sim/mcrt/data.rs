//! Output data structure.

use crate::{
    access, clone,
    data::Histogram,
    err::Error,
    file::Save,
    geom::Cube,
    ord::{X, Y, Z},
    tools::Range,
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path, fs::File, fs::OpenOptions, io::Write};

/// MCRT output data.
pub struct Data {
    /// Measured volume.
    boundary: Cube,
    /// Cell volume [m^3].
    cell_vol: f64,
    /// Spectrometer.
    pub spec: Histogram,
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

    /// Total Raman weight
    pub total_raman_weight: f64,
}

impl Data {
    access!(boundary, Cube);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, res: [usize; 3], range: Range, hist_bins: u64) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self {
            boundary,
            cell_vol,
            spec: Histogram::new_range(range, hist_bins),
            escaped_weight: 0.0,
            emission_power: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),

            total_raman_weight: 0.0,
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

        self.total_raman_weight += rhs.total_raman_weight;
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("spectrometer.dat");
        println!("Saving: {}", path.display());
        self.spec.save(&path)?;

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
        (&self.shifts / self.cell_vol).save(&path)?;

        let path = out_dir.join("Raman_weight.txt");
        //self.total_raman_weight.save(&path)?;
        //let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
        //println!("Saving: {}", path.display());
        //writeln!(file, "{}", self.total_raman_weight)?;
        println!("Total Raman weight: {}", self.total_raman_weight);

        Ok(())
    }
}
