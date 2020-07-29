//! Material map.

use crate::{access, Error, Group, Grp, Load};
use ndarray::Array3;
use std::path::Path;

/// Material grid map.
pub struct MatMap {
    /// Material groups.
    mats: Vec<Group>,
    /// Material map.
    map: Array3<i32>,
}

impl MatMap {
    access!(mats, Vec<Group>);
    access!(map, Array3<i32>);

    /// Construct a new instance.
    /// # Errors
    /// if the material map can not be loaded.
    #[allow(clippy::option_expect_used)]
    #[inline]
    pub fn load(path: &Path, mats: Vec<Group>) -> Result<Self, Error> {
        let map = Array3::load(path)?;
        let kinds = crate::order::kinds(
            map.as_slice()
                .expect("Could not create slice from loaded material map."),
        );
        if kinds != mats.len() {
            panic!("Number of materials in the map does not match number in the given list.");
        }

        Ok(Self { mats, map })
    }

    /// Get the material group at a given index of the map.
    #[inline]
    #[must_use]
    pub fn get_mat(&self, index: [usize; 3]) -> &Grp {
        &self.mats[(self.map[index] - 1) as usize]
    }
}
