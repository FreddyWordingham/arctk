//! Flat triangle implementation.

use crate::{
    access,
    geom::{Collide, Cube, Emit, Ray, Side, Trace, Transformable},
    math::{Dir3, Pos3, Trans3, Vec3},
    ord::{ALPHA, BETA, GAMMA},
};
use rand::Rng;

/// Triangle.
#[derive(Clone)]
pub struct Triangle {
    /// Vertex points.
    verts: [Pos3; 3],
    /// Surface plane normal.
    plane_norm: Dir3,
}

impl Triangle {
    access!(verts, [Pos3; 3]);
    access!(plane_norm, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Pos3; 3]) -> Self {
        let plane_norm = Self::init_plane_norm(&verts);

        Self { verts, plane_norm }
    }

    /// Initialise the plane normal.
    #[inline]
    #[must_use]
    fn init_plane_norm(verts: &[Pos3; 3]) -> Dir3 {
        Dir3::new_normalize((verts[ALPHA] - verts[GAMMA]).cross(&(verts[BETA] - verts[ALPHA])))
    }

    /// Calculate the side lengths.
    #[inline]
    #[must_use]
    pub fn side_lengths(&self) -> [f64; 3] {
        let ab = nalgebra::distance(&self.verts[ALPHA], &self.verts[BETA]);
        let bc = nalgebra::distance(&self.verts[BETA], &self.verts[GAMMA]);
        let ca = nalgebra::distance(&self.verts[GAMMA], &self.verts[ALPHA]);

        [ab, bc, ca]
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        ab + bc + ca
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        let s = (ab + bc + ca) * 0.5;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Centre point.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Pos3 {
        Pos3::from(
            ((self.verts[ALPHA].to_homogeneous()
                + self.verts[BETA].to_homogeneous()
                + self.verts[GAMMA].to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }

    /// Determine the intersection distance along a `Ray`'s direction.
    /// Also return the barycentric intersection coordinates.
    #[inline]
    #[must_use]
    pub fn intersection_coors(&self, ray: &Ray) -> Option<(f64, [f64; 3])> {
        let verts = self.verts;

        let e1 = verts[BETA] - verts[ALPHA];
        let e2 = verts[GAMMA] - verts[ALPHA];

        let d_cross_e2 = ray.dir().cross(&e2);
        let e1_dot_d_cross_e2 = e1.dot(&d_cross_e2);

        if e1_dot_d_cross_e2.abs() <= 0.0 {
            return None;
        }

        let inv_e1_dot_d_cross_e2 = 1.0 / e1_dot_d_cross_e2;
        let rel_pos = ray.pos() - verts[ALPHA];
        let u = inv_e1_dot_d_cross_e2 * rel_pos.dot(&d_cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = rel_pos.cross(&e1);
        let v = inv_e1_dot_d_cross_e2 * ray.dir().dot(&q);

        if (v < 0.0) || ((u + v) > 1.0) {
            return None;
        }

        let dist = inv_e1_dot_d_cross_e2 * e2.dot(&q);

        if dist <= 0.0 {
            return None;
        }

        let w = 1.0 - (u + v);

        Some((dist, [u, v, w]))
    }
}

impl Collide for Triangle {
    #[inline]
    #[must_use]
    fn overlap(&self, cube: &Cube) -> bool {
        let c = cube.centre();
        let e = cube.half_widths();

        let v0 = self.verts[ALPHA] - c;
        let v1 = self.verts[BETA] - c;
        let v2 = self.verts[GAMMA] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vec3::x_axis();
        let u1 = Vec3::y_axis();
        let u2 = Vec3::z_axis();

        let axis_test = |axis: &Vec3| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);

            let r = e.z.mul_add(
                u2.dot(axis).abs(),
                e.x.mul_add(u0.dot(axis).abs(), e.y * u1.dot(axis).abs()),
            );

            if (-(p0.max(p1).max(p2))).max(p0.min(p1).min(p2)) > r {
                return false;
            }

            true
        };

        if !axis_test(&u0) {
            return false;
        }
        if !axis_test(&u1) {
            return false;
        }
        if !axis_test(&u2) {
            return false;
        }

        let axis_u0_f0 = u0.cross(&f0);
        let axis_u0_f1 = u0.cross(&f1);
        let axis_u0_f2 = u0.cross(&f2);

        let axis_u1_f0 = u1.cross(&f0);
        let axis_u1_f1 = u1.cross(&f1);
        let axis_u1_f2 = u1.cross(&f2);

        let axis_u2_f0 = u2.cross(&f0);
        let axis_u2_f1 = u2.cross(&f1);
        let axis_u2_f2 = u2.cross(&f2);

        if !axis_test(&axis_u0_f0) {
            return false;
        }
        if !axis_test(&axis_u0_f1) {
            return false;
        }
        if !axis_test(&axis_u0_f2) {
            return false;
        }

        if !axis_test(&axis_u1_f0) {
            return false;
        }
        if !axis_test(&axis_u1_f1) {
            return false;
        }
        if !axis_test(&axis_u1_f2) {
            return false;
        }

        if !axis_test(&axis_u2_f0) {
            return false;
        }
        if !axis_test(&axis_u2_f1) {
            return false;
        }
        if !axis_test(&axis_u2_f2) {
            return false;
        }

        if !axis_test(&self.plane_norm) {
            return false;
        }

        true
    }
}

impl Trace for Triangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        self.dist(ray).map(|dist| {
            let side = Side::new(ray.dir(), self.plane_norm);
            (dist, side)
        })
    }
}

impl Transformable for Triangle {
    #[inline]
    fn transform(&mut self, trans: &Trans3) {
        for v in &mut self.verts {
            *v = trans.transform_point(v);
        }

        self.plane_norm = Dir3::new_normalize(trans.transform_vector(&self.plane_norm));
    }
}

impl Emit for Triangle {
    #[inline]
    #[must_use]
    fn cast<R: Rng>(&self, rng: &mut R) -> Ray {
        let mut u = rng.gen::<f64>();
        let mut v = rng.gen::<f64>();

        if (u + v) > 1.0 {
            u = 1.0 - u;
            v = 1.0 - v;
        }

        let edge_a_b = self.verts[BETA] - self.verts[ALPHA];
        let edge_a_c = self.verts[GAMMA] - self.verts[ALPHA];

        let pos = self.verts[ALPHA] + (edge_a_b * u) + (edge_a_c * v);

        Ray::new(pos, self.plane_norm)
    }
}
