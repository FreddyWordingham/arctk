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
    /// Surface mesh.
    Surface(Mesh),
}

impl Emitter {
    /// Emit a new ray.
    #[inline]
    #[must_use]
    pub fn emit<R: Rng>(&self, rng: &mut R) -> Ray {
        match *self {
            Self::Beam(ref ray) => ray.clone(),
            Self::Points(ref ps) => {
                Ray::new(ps[rng.gen_range(0, ps.len())], rand_isotropic_dir(rng))
            }
            Self::Surface(ref mesh) => mesh.cast(rng),
        }
    }
}
