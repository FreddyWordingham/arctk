//! Axis-aligned uniform grid.

use nalgebra::{Point3, Vector3};

use crate::geom::Cube;

/// Three-dimensional uniform partitioning.
pub struct Grid {
    /// Boundary of the grid.
    pub boundary: Cube,
    /// Number of voxels in each direction.
    pub num_voxels: [usize; 3],
    /// Size of a voxel.
    voxel_size: Vector3<f64>,
    /// Voxel inverse size.
    voxel_inv_size: Vector3<f64>,
}

impl Grid {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, num_voxels: [usize; 3]) -> Self {
        let widths = boundary.widths();
        let voxel_size = Vector3::new(
            widths.x / num_voxels[0] as f64,
            widths.y / num_voxels[1] as f64,
            widths.z / num_voxels[2] as f64,
        );
        Self {
            boundary,
            num_voxels,
            voxel_size,
            voxel_inv_size: Vector3::new(
                1.0 / voxel_size.x,
                1.0 / voxel_size.y,
                1.0 / voxel_size.z,
            ),
        }
    }

    /// Determine the voxel index of a given point.
    #[inline]
    #[must_use]
    pub fn voxel_index(&self, point: &Point3<f64>) -> Option<[usize; 3]> {
        if !self.boundary.contains(point) {
            return None;
        }

        let x = ((point.x - self.boundary.mins.x) * self.voxel_inv_size.x) as usize;
        let y = ((point.y - self.boundary.mins.y) * self.voxel_inv_size.y) as usize;
        let z = ((point.z - self.boundary.mins.z) * self.voxel_inv_size.z) as usize;

        Some([x, y, z])
    }

    /// Generate voxel boundary of a given voxel index.
    #[inline]
    #[must_use]
    pub fn generate_voxel(&self, index: [usize; 3]) -> Cube {
        let mins = Point3::new(
            self.boundary.mins.x + (self.voxel_size.x * index[0] as f64),
            self.boundary.mins.y + (self.voxel_size.y * index[1] as f64),
            self.boundary.mins.z + (self.voxel_size.z * index[2] as f64),
        );
        let maxs = mins + self.voxel_size;

        Cube::new(mins, maxs)
    }
}
