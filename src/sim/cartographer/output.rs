//! Output structure.

use crate::{ord::Group, Error, Save, X, Y, Z};
use ndarray::{Array3, Zip};
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Center material sample.
    pub mats: Array3<Option<Group>>,
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
            mats: Array3::default(res),
        }
    }

    /// Save the material maps.
    #[inline]
    #[must_use]
    pub fn save_mat_maps(&self, out_dir: &Path) -> Result<(), Error> {
        let base = self.mats.map(|m| m.as_ref().unwrap().clone());

        let mut mat_list = Vec::new();
        for m in base.iter() {
            if !mat_list.contains(m) {
                mat_list.push(m.clone());
            }
        }
        mat_list.sort();

        let mut composite = Array3::zeros([
            self.mats.shape()[X],
            self.mats.shape()[Y],
            self.mats.shape()[Z],
        ]);
        for (index, mat) in mat_list.iter().enumerate() {
            let map: Array3<f64> = base.map(|m| if m == mat { 1.0 } else { 0.0 });
            let p = out_dir.join(format!("{}.nc", mat));
            println!("Saving: {}", p.display());
            map.save(&p)?;

            composite += &(map * (index + 1) as f64);
        }

        let p = out_dir.join("composite.nc");
        println!("Saving: {}", p.display());
        composite.save(&p)
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        Zip::from(&mut self.mats)
            .and(&rhs.mats)
            .apply(|a, b| *a = if a.is_some() { a.clone() } else { b.clone() });
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

        self.save_mat_maps(&path)
    }
}
