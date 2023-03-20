//! Axis-aligned cuboid.

use core::cmp::Ordering;
use nalgebra::{Point3, Unit, Vector3};

use crate::rt::{Ray, Side};

/// Cuboid oriented along the Cartesian axes.
#[derive(Clone)]
pub struct Cube {
    /// Minimum bound.
    pub mins: Point3<f64>,
    /// Maximum bound.
    pub maxs: Point3<f64>,
}

impl Cube {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        debug_assert!(mins < maxs);

        Self { mins, maxs }
    }

    /// Calculate the centre position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the widths.
    #[inline]
    #[must_use]
    pub fn widths(&self) -> Vector3<f64> {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[inline]
    #[must_use]
    pub fn half_widths(&self) -> Vector3<f64> {
        (self.maxs - self.mins) * 0.5
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

    /// Determine if the given point if contained with the volume.
    /// Points lying on the surface are contained contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Point3<f64>) -> bool {
        p >= &self.mins && p <= &self.maxs
    }

    /// Shrink by a fraction of the side lengths.
    /// Central position is maintained.
    #[inline]
    pub fn shrink(&mut self, f: f64) {
        debug_assert!(f > 0.0);
        debug_assert!(f < 1.0);

        let delta = self.half_widths() * f;

        self.mins += delta;
        self.maxs -= delta;
    }

    /// Expand by a fraction of the side lengths.
    /// Central position is maintained.
    #[inline]
    pub fn expand(&mut self, f: f64) {
        debug_assert!(f > 0.0);

        let delta = self.half_widths() * f;

        self.mins -= delta;
        self.maxs += delta;
    }

    /// Check for an intersection with another Cube.
    #[inline]
    #[must_use]
    pub fn collides(&self, cube: &Self) -> bool {
        self.mins <= cube.maxs && self.maxs >= cube.mins
    }

    /// Determine the distances from a given Ray to the axial planes of a Cube's minimum and maximal bounds.
    #[inline]
    #[must_use]
    fn intersections(&self, ray: &Ray) -> (f64, f64) {
        let t_0: Vec<_> = self
            .mins
            .iter()
            .zip(ray.pos.iter().zip(ray.dir.iter()))
            .map(|(m, (p, d))| (m - p) / d)
            .collect();

        let t_1: Vec<_> = self
            .maxs
            .iter()
            .zip(ray.pos.iter().zip(ray.dir.iter()))
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
            .expect("Failed to perform Ray-Cube intersection.");

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
            .expect("Failed to perform Ray-Cube intersection.");

        (t_min, t_max)
    }

    /// Determine if a Ray-Cube intersection occurs.
    #[inline]
    #[must_use]
    pub fn hit(&self, ray: &Ray) -> bool {
        let (t_min, t_max) = self.intersections(ray);

        !(t_max <= 0.0 || t_min > t_max)
    }

    /// Determine the distance to a Ray-Cube intersection.
    #[inline]
    #[must_use]
    pub fn dist(&self, ray: &Ray) -> Option<f64> {
        let (t_min, t_max) = self.intersections(ray);

        if t_max <= 0.0 || t_min > t_max {
            return None;
        }

        if t_min > 0.0 {
            return Some(t_min);
        }

        Some(t_max)
    }

    /// Determine the distance and facing side of a Ray-Cube intersection.
    #[inline]
    #[must_use]
    pub fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if let Some(dist) = self.dist(ray) {
            let hit = ray.pos + (dist * ray.dir.as_ref());
            let relative = hit - self.centre();

            let xy = relative.y / relative.x;
            let zy = relative.z / relative.y;

            let norm = Unit::new_normalize(if (-1.0..=1.0).contains(&xy) {
                Vector3::new(1.0_f64.copysign(relative.x), 0.0, 0.0)
            } else if (-1.0..=1.0).contains(&zy) {
                Vector3::new(0.0, 1.0_f64.copysign(relative.y), 0.0)
            } else {
                Vector3::new(0.0, 0.0, 1.0_f64.copysign(relative.z))
            });

            return Some((dist, Side::new(&ray.dir, norm)));
        }

        None
    }
}
