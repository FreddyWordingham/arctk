//! Output data structure.

use crate::ord::{X, Y, Z};
use ndarray::Array3;

/// Cartographer output data.
pub struct Output<'a> {
    /// Occupying material.
    pub mat: Array3<&'a str>,
}

impl<'a> Output<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self {
            mat: Array3::default(res),
        }
    }
}

// impl<'a> AddAssign<&Self> for Output<'a> {
//     #[inline]
//     fn add_assign(&mut self, rhs: &Self) {
//         self.mat += &rhs.mat;
//     }
// }

// impl<'a> Save for Output<'a> {
//     #[inline]
//     fn save(&self, out_dir: &Path) -> Result<(), Error> {
//         let path = out_dir.join("emission_density.nc");
//         println!("Saving: {}", path.display());
//         (&self.emission / self.cell_vol).save(&path)?;

//         let path = out_dir.join("energy_density.nc");
//         println!("Saving: {}", path.display());
//         (&self.energy / self.cell_vol).save(&path)?;

//         let path = out_dir.join("absorption_density.nc");
//         println!("Saving: {}", path.display());
//         (&self.absorptions / self.cell_vol).save(&path)?;

//         let path = out_dir.join("shift_density.nc");
//         println!("Saving: {}", path.display());
//         (&self.shifts / self.cell_vol).save(&path)?;

//         Ok(())
//     }
// }
