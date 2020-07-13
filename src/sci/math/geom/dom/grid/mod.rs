//! Regular grid cell scheme.

pub mod settings;

pub use self::settings::*;

use crate::{access, display_field, display_field_ln, Aabb, Pos3, Vec3, X, Y, Z};
use std::fmt::{Display, Formatter, Result};

/// Regular grid structure.
pub struct Grid {
    /// Boundary.
    boundary: Aabb,
    /// Resolution.
    res: [usize; 3],
    /// Voxel size.
    voxel_size: Vec3,
}

impl Grid {
    access!(boundary, Aabb);
    access!(res, [usize; 3]);
    access!(voxel_size, Vec3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sett: &Settings) -> Self {
        let mut voxel_size = sett.boundary().widths();
        for (w, n) in voxel_size.iter_mut().zip(sett.res()) {
            *w /= *n as f64;
        }

        Self {
            boundary: sett.boundary().clone(),
            res: *sett.res(),
            voxel_size,
        }
    }

    /// Calculate the voxel volume.
    #[inline]
    #[must_use]
    pub fn voxel_vol(&self) -> f64 {
        self.voxel_size.x * self.voxel_size.y * self.voxel_size.z
    }

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }

    /// If the given position is contained within the grid,
    /// generate the index for the given position within the grid.
    #[inline]
    #[must_use]
    pub fn gen_index(&self, p: &Pos3) -> Option<[usize; 3]> {
        if self.boundary.contains(p) {
            let mins = self.boundary.mins();
            let maxs = self.boundary.maxs();

            Some([
                (((p.x - mins.x) / (maxs.x - mins.x)) * self.res[X] as f64).floor() as usize,
                (((p.y - mins.y) / (maxs.y - mins.y)) * self.res[Y] as f64).floor() as usize,
                (((p.z - mins.z) / (maxs.z - mins.z)) * self.res[Z] as f64).floor() as usize,
            ])
        } else {
            None
        }
    }

    /// If the given position is contained within the grid,
    /// generate the index and voxel for the given position within the grid.
    #[inline]
    #[must_use]
    pub fn gen_index_voxel(&self, p: &Pos3) -> Option<([usize; 3], Aabb)> {
        if let Some(index) = self.gen_index(p) {
            let mut min = *self.boundary.mins();
            min.x += self.voxel_size[X] * index[X] as f64;
            min.y += self.voxel_size[Y] * index[Y] as f64;
            min.z += self.voxel_size[Z] * index[Z] as f64;

            let boundary = Aabb::new(min, min + self.voxel_size);
            debug_assert!(boundary.contains(p));

            Some((index, boundary))
        } else {
            None
        }
    }
}

impl Display for Grid {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(
            fmt,
            "resolution",
            format!("{}x{}x{}", self.res[X], self.res[Y], self.res[Z])
        )?;
        display_field_ln!(fmt, "total cells", self.total_cells())?;
        display_field_ln!(
            fmt,
            "voxel size",
            &format!(
                "[{:.2}, {:.2}, {:.2}]",
                self.voxel_size.x, self.voxel_size.y, self.voxel_size.z
            ),
            "m"
        )?;
        display_field_ln!(fmt, "voxel volume", self.voxel_vol(), "m^3")?;
        display_field!(fmt, "boundary", &self.boundary)
    }
}
