//! Optical material.

use crate::{
    geom::{Emit, Mesh, Ray},
    math::{rand_isotropic_dir, Pos3},
};
use rand::Rng;

/// Ray emission structure.
pub enum Emitter {
    /// Single beam.
    Beam(Ray),
    /// Points.
    Points(Vec<Pos3>),
    /// Weighted points.
    WeightedPoints(Vec<Pos3>, Vec<f64>),
    /// Surface mesh.
    Surface(Mesh),
}

impl Emitter {
    /// Construct a new beam instance.
    #[inline]
    #[must_use]
    pub const fn new_beam(ray: Ray) -> Self {
        Self::Beam(ray)
    }

    /// Construct a new points instance.
    #[inline]
    #[must_use]
    pub fn new_points(points: Vec<Pos3>) -> Self {
        debug_assert!(!points.is_empty());

        Self::Points(points)
    }

    /// Construct a new points instance.
    #[inline]
    #[must_use]
    pub fn new_weighted_points(points: Vec<Pos3>, weights: Vec<f64>) -> Self {
        debug_assert!(!points.is_empty());
        debug_assert!(points.len() == weights.len());

        Self::WeightedPoints(points, weights)
    }

    /// Construct a new surface instance.
    #[inline]
    #[must_use]
    pub const fn new_surface(mesh: Mesh) -> Self {
        Self::Surface(mesh)
    }

    /// Emit a new ray.
    #[inline]
    #[must_use]
    pub fn emit<R: Rng>(&self, rng: &mut R) -> Ray {
        match *self {
            Self::Beam(ref ray) => ray.clone(),
            Self::Points(ref ps) => {
                Ray::new(ps[rng.gen_range(0, ps.len())], rand_isotropic_dir(rng))
            }
            Self::WeightedPoints(ref ps, ref ws) => {
                Ray::new(ps[rng.gen_range(0, ps.len())], rand_isotropic_dir(rng))
            }
            Self::Surface(ref mesh) => mesh.cast(rng),
        }
    }
}
