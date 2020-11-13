//! Output structure.

use super::Interface;
use crate::{
    err::Error,
    file::Save,
    ord::{Key, Set, X, Y, Z},
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Cartographer output data.
pub struct Data {
    /// Material sample.
    pub maps: Set<Key, Array3<f64>>,
    /// Problematic cells.
    pub undetermined: Array3<f64>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(inters: &Set<Key, Interface>, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let mut keys = Vec::new();
        for &(ref inside, ref outside) in inters.map().values() {
            keys.push(inside);
            keys.push(outside);
        }
        keys.sort();
        keys.dedup();

        let maps = Set::from_vec(
            keys.into_iter()
                .map(|key| (key.clone(), Array3::zeros(res)))
                .collect(),
        );

        Self {
            maps,
            undetermined: Array3::zeros(res),
        }
    }

    /// Save the maps to the given directory.
    /// # Errors
    /// if a map can not be saved.
    #[inline]
    pub fn save_maps(&self, out_dir: &Path) -> Result<(), Error> {
        for (key, map) in self.maps.map() {
            let p = out_dir.join(format!("{}.nc", key));
            println!("Saving: {}", p.display());
            map.save(&p)?;
        }

        Ok(())
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        for ((l_key, l_map), (r_key, r_map)) in
            self.maps.mut_map().iter_mut().zip(rhs.maps.map().iter())
        {
            debug_assert!(l_key == r_key);
            *l_map += r_map;
        }

        self.undetermined += &rhs.undetermined;
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        self.save_maps(out_dir)?;

        let p = out_dir.join("undetermined.nc");
        println!("Saving: {}", p.display());
        self.undetermined.save(&p)
    }
}
