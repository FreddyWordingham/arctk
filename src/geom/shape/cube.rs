//! Axis-aligned-bounding-box implementation.

use crate::{
    access,
    geom::{Collide, Mesh, Ray, Side, Trace},
    math::{Pos3, Vec3},
};
use arctk_attr::load;
use std::cmp::Ordering;

/// Axis-aligned bounding box geometry.
/// Used for spatial partitioning.
#[load]
#[derive(Clone)]
pub struct Cube {
    /// Minimum bound.
    mins: Pos3,
    /// Maximum bound.
    maxs: Pos3,
}

impl Cube {
    access!(mins, Pos3);
    access!(maxs, Pos3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mins: Pos3, maxs: Pos3) -> Self {
        debug_assert!(mins < maxs);

        Self { mins, maxs }
    }

    /// Construct a new axis-aligned bounding box centred on a given point with given half widths.
    #[inline]
    #[must_use]
    pub fn new_centred(centre: &Pos3, hws: &Vec3) -> Self {
        debug_assert!(hws.iter().all(|x| *x > 0.0));

        Self::new(centre - hws, centre + hws)
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    pub fn new_shrink(surfs: &[Mesh]) -> Self {
        let mut mins = None;
        let mut maxs = None;

        for mesh in surfs {
            let (mesh_mins, mesh_maxs) = mesh.boundary().mins_maxs();

            if mins.is_none() {
                mins = Some(mesh_mins);
            } else {
                for (grid_min, mesh_min) in mins.as_mut().unwrap().iter_mut().zip(mesh_mins.iter())
                {
                    if mesh_min < grid_min {
                        *grid_min = *mesh_min;
                    }
                }
            }

            if maxs.is_none() {
                maxs = Some(mesh_maxs);
            } else {
                for (grid_max, mesh_max) in maxs.as_mut().unwrap().iter_mut().zip(mesh_maxs.iter())
                {
                    if mesh_max > grid_max {
                        *grid_max = *mesh_max;
                    }
                }
            }
        }

        Self::new(mins.unwrap(), maxs.unwrap())
    }

    /// Get mins and maxs together.
    #[inline]
    #[must_use]
    pub const fn mins_maxs(&self) -> (Pos3, Pos3) {
        (self.mins, self.maxs)
    }

    /// Calculate the widths.
    #[inline]
    #[must_use]
    pub fn widths(&self) -> Vec3 {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[inline]
    #[must_use]
    pub fn half_widths(&self) -> Vec3 {
        self.widths() * 0.5
    }

    /// Calculate the centre position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Pos3 {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ws = self.widths();
        2.0 * ws.z.mul_add(ws.x, ws.x.mul_add(ws.y, ws.y * ws.z))
    }

    /// Calculate the volume.
    #[inline]
    #[must_use]
    pub fn vol(&self) -> f64 {
        let ws = self.widths();
        ws.x * ws.y * ws.z
    }

    /// Determine if the given point if contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Pos3) -> bool {
        p >= &self.mins && p <= &self.maxs
    }

    /// Shrink the aabb by a fraction of its lengths, maintaining the central position.
    #[inline]
    pub fn shrink(&mut self, f: f64) {
        debug_assert!(f > 0.0);
        debug_assert!(f < 1.0);

        let delta = self.half_widths() * f;

        self.mins += delta;
        self.maxs -= delta;
    }

    /// Expand the aabb by a fraction of its lengths, maintaining the central position.
    #[inline]
    pub fn expand(&mut self, f: f64) {
        debug_assert!(f > 0.0);

        let delta = self.half_widths() * f;

        self.mins -= delta;
        self.maxs += delta;
    }

    /// Determine the intersection distances along a ray's direction.
    #[inline]
    #[must_use]
    fn intersections(&self, ray: &Ray) -> (f64, f64) {
        let t_0: Vec<_> = self
            .mins
            .iter()
            .zip(ray.pos().iter().zip(ray.dir().iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_1: Vec<_> = self
            .maxs
            .iter()
            .zip(ray.pos().iter().zip(ray.dir().iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_min = t_0
            .iter()
            .zip(t_1.iter())
            .map(|(a, b)| a.min(*b))
            .max_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap();

        let t_max = t_0
            .iter()
            .zip(t_1.iter())
            .map(|(a, b)| a.max(*b))
            .min_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap();

        (t_min, t_max)
    }
}

impl Collide for Cube {
    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Cube) -> bool {
        self.mins <= aabb.maxs && self.maxs >= aabb.mins
    }
}

impl Trace for Cube {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        let (t_min, t_max) = self.intersections(ray);

        !(t_max <= 0.0 || t_min > t_max)
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        let (t_min, t_max) = self.intersections(ray);

        if t_max <= 0.0 || t_min > t_max {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    #[inline]
    #[must_use]
    fn dist_side(&self, _ray: &Ray) -> Option<(f64, Side)> {
        unimplemented!("Tell me (Freddy) if you need this.");
    }
}
