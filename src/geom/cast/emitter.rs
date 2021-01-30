//! Optical material.

use crate::{
    geom::{Emit, Mesh, Ray},
    math::{rand_isotropic_dir, Pos3},
};
use rand::Rng;
use std::fmt::{Display, Error, Formatter};

/// Ray emission structure.
#[derive(Clone)]
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
    pub fn new_weighted_points(points: Vec<Pos3>, weights: &[f64]) -> Self {
        debug_assert!(!points.is_empty());
        debug_assert!(points.len() == weights.len());

        let sum: f64 = weights.iter().sum();
        let mut cumulative_weight = Vec::with_capacity(weights.len());
        let mut total = 0.0;
        for w in weights {
            total += w;
            cumulative_weight.push(total / sum);
        }

        Self::WeightedPoints(points, cumulative_weight)
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
                Ray::new(ps[rng.gen_range(0..ps.len())], rand_isotropic_dir(rng))
            }
            Self::WeightedPoints(ref ps, ref ws) => {
                let r: f64 = rng.gen();
                for (p, w) in ps.iter().zip(ws) {
                    if r <= *w {
                        return Ray::new(*p, rand_isotropic_dir(rng));
                    }
                }
                unreachable!("Failed to determine weighted point to emit from.");
            }
            Self::Surface(ref mesh) => mesh.cast(rng),
        }
    }
}

impl Display for Emitter {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let kind = match *self {
            Self::Beam { .. } => "Beam",
            Self::Points { .. } => "Points",
            Self::WeightedPoints { .. } => "WeightedPoints",
            Self::Surface { .. } => "Surface",
        };
        write!(fmt, "{}", kind)
    }
}
