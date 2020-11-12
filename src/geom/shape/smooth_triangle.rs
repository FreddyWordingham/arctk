//! Smooth triangle implementation.

use crate::{
    access,
    geom::{Collide, Cube, Emit, Ray, Side, Trace, Transformable, Triangle},
    math::{Dir3, Pos3, Trans3},
    ord::{ALPHA, BETA, GAMMA},
};
use rand::Rng;

/// Triangle geometry with normal interpolation.
pub struct SmoothTriangle {
    /// Base triangle.
    tri: Triangle,
    /// Normal vectors.
    norms: [Dir3; 3],
}

impl SmoothTriangle {
    access!(tri, Triangle);
    access!(norms, [Dir3; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tri: Triangle, norms: [Dir3; 3]) -> Self {
        if !norms.iter().all(|&n| n.dot(tri.plane_norm()) > 0.0) {
            println!("[WARN] Reverse triangle.");
        }

        Self { tri, norms }
    }

    /// Construct a new instance from vertices.
    #[inline]
    #[must_use]
    pub fn new_from_verts(verts: [Pos3; 3], norms: [Dir3; 3]) -> Self {
        Self::new(Triangle::new(verts), norms)
    }
}

impl Collide for SmoothTriangle {
    #[inline]
    #[must_use]
    fn overlap(&self, cube: &Cube) -> bool {
        self.tri.overlap(cube)
    }
}

impl Trace for SmoothTriangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.tri.hit(ray)
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        self.tri.dist(ray)
    }

    #[inline]
    #[must_use]
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if let Some((dist, [u, v, w])) = self.tri.intersection_coors(ray) {
            Some((
                dist,
                Side::new(
                    ray.dir(),
                    Dir3::new_normalize(
                        (self.norms[BETA].into_inner() * u)
                            + (self.norms[GAMMA].into_inner() * v)
                            + (self.norms[ALPHA].into_inner() * w),
                    ),
                ),
            ))
        } else {
            None
        }
    }
}

impl Transformable for SmoothTriangle {
    #[inline]
    fn transform(&mut self, trans: &Trans3) {
        self.tri.transform(trans);

        for n in &mut self.norms {
            *n = Dir3::new_normalize(trans.transform_vector(n.as_ref()));
        }
    }
}

impl Emit for SmoothTriangle {
    #[inline]
    #[must_use]
    fn cast<R: Rng>(&self, rng: &mut R) -> Ray {
        let mut u = rng.gen::<f64>();
        let mut v = rng.gen::<f64>();

        if (u + v) > 1.0 {
            u = 1.0 - u;
            v = 1.0 - v;
        }
        let w = 1.0 - u - v;

        let edge_a_b = self.tri.verts()[BETA] - self.tri.verts()[ALPHA];
        let edge_a_c = self.tri.verts()[GAMMA] - self.tri.verts()[ALPHA];

        let pos = self.tri.verts()[ALPHA] + (edge_a_b * u) + (edge_a_c * v);
        let dir = Dir3::new_normalize(
            (self.norms[BETA].as_ref() * u)
                + (self.norms[GAMMA].as_ref() * v)
                + (self.norms[ALPHA].as_ref() * w),
        );

        Ray::new(pos, dir)
    }
}
