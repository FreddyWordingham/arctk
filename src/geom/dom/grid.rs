//! Regular Cartesian-grid cell scheme.

use crate::{
    access,
    geom::Cube,
    math::{Pos3, Vec3},
    ord::{X, Y, Z},
};

/// Regular Cartesian-grid structure.
pub struct Grid {
    /// Boundary.
    boundary: Cube,
    /// Resolution.
    res: [usize; 3],
    /// Voxel size.
    voxel_size: Vec3,
}

impl Grid {
    access!(boundary, Cube);
    access!(res, [usize; 3]);
    access!(voxel_size, Vec3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let mut voxel_size = boundary.widths();
        for (w, n) in voxel_size.iter_mut().zip(&res) {
            *w /= *n as f64;
        }

        Self {
            boundary,
            res,
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

    /// Generate the voxel for the given index.
    #[inline]
    #[must_use]
    pub fn gen_voxel(&self, index: &[usize; 3]) -> Cube {
        debug_assert!(index[X] < self.res[X]);
        debug_assert!(index[Y] < self.res[Y]);
        debug_assert!(index[Z] < self.res[Z]);

        let x = self
            .voxel_size
            .x
            .mul_add(index[X] as f64, self.boundary.mins().x);
        let y = self
            .voxel_size
            .y
            .mul_add(index[Y] as f64, self.boundary.mins().y);
        let z = self
            .voxel_size
            .z
            .mul_add(index[Z] as f64, self.boundary.mins().z);

        let mins = Pos3::new(x, y, z);

        Cube::new(mins, mins + self.voxel_size)
    }

    /// If the given position is contained within the grid,
    /// generate the index and voxel for the given position within the grid.
    #[inline]
    #[must_use]
    pub fn gen_index_voxel(&self, p: &Pos3) -> Option<([usize; 3], Cube)> {
        if let Some(index) = self.gen_index(p) {
            let mut min = *self.boundary.mins();
            min.x += self.voxel_size[X] * index[X] as f64;
            min.y += self.voxel_size[Y] * index[Y] as f64;
            min.z += self.voxel_size[Z] * index[Z] as f64;

            let boundary = Cube::new(min, min + self.voxel_size);
            debug_assert!(boundary.contains(p));

            Some((index, boundary))
        } else {
            None
        }
    }
}
