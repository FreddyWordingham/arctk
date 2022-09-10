//! Triangle-mesh.

use itertools::izip;
use ndarray::parallel::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    geom::{Cube, Triangle},
    rt::{Ray, Side},
};

/// Mesh of two-dimensional triangles embedded in space as a three-dimensional mesh.
#[derive(Clone)]
pub struct Mesh {
    /// Bounding box.
    pub boundary: Cube,
    /// List of component triangles.
    pub tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tris: Vec<Triangle>) -> Self {
        debug_assert!(!tris.is_empty());

        // Calculate bounding box.
        let mut mins = tris[0].centre();
        let mut maxs = mins;
        for tri in &tris {
            for vert in tri.verts {
                for (a, (min, max)) in izip!(vert.iter(), izip!(mins.iter_mut(), maxs.iter_mut())) {
                    if *min > *a {
                        *min = *a;
                    } else if *max < *a {
                        *max = *a;
                    }
                }
            }
        }
        let mut boundary = Cube::new(mins, maxs);
        boundary.expand(0.01); // TODO: Consider what value is best here.

        Self { boundary, tris }
    }

    /// Check for an intersection with a given bounding box.
    #[inline]
    #[must_use]
    pub fn collides(&self, cube: &Cube) -> bool {
        if !self.boundary.collides(cube) {
            return false;
        }

        self.tris.par_iter().any(|tri| tri.collides(cube))
    }

    /// Determine if a Ray-Mesh intersection occurs.
    #[inline]
    #[must_use]
    pub fn hit(&self, ray: &Ray) -> bool {
        if !self.boundary.hit(ray) {
            return false;
        }

        self.tris.par_iter().any(|t| t.hit(ray))
    }

    /// Determine the distance to a Ray-Mesh intersection.
    #[inline]
    #[must_use]
    pub fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.boundary.hit(ray) {
            return None;
        }

        self.tris
            .par_iter()
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| {
                a.partial_cmp(b)
                    .expect("Failed to perform Ray-Mesh intersection")
            })
    }

    /// Determine the distance and facing side of a Ray-Mesh intersection.
    #[inline]
    #[must_use]
    pub fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if !self.boundary.hit(ray) {
            return None;
        }

        self.tris
            .par_iter()
            .filter_map(|tri| tri.dist_side(ray))
            .min_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .expect("Failed to perform Ray-Mesh intersection")
            })
    }
}
