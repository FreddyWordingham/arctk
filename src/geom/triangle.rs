//! Triangle.

use nalgebra::{Point3, Unit, Vector3};

use crate::{
    geom::Cube,
    rt::{Ray, Side},
};

/// Two-dimensional triangle embedded in three-dimensional space.
/// Vertex positions and normals are stored in the same counter-clockwise order.
/// The plane-normal follows the right-hand rule.
#[derive(Clone)]
pub struct Triangle {
    /// Vertex positions.
    pub verts: [Point3<f64>; 3],
    /// vertex normals.
    pub norms: [Unit<Vector3<f64>>; 3],
    /// Plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Triangle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        let plane_norm = Unit::new_normalize((verts[0] - verts[2]).cross(&(verts[1] - verts[0])));

        Self {
            verts,
            norms,
            plane_norm,
        }
    }

    /// Calculate the central position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        Point3::from(
            ((self.verts[0].to_homogeneous()
                + self.verts[1].to_homogeneous()
                + self.verts[2].to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }

    /// Calculate the side lengths.
    #[inline]
    #[must_use]
    pub fn side_lengths(&self) -> [f64; 3] {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        [ab, bc, ca]
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        self.side_lengths().iter().sum()
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        let s = (ab + bc + ca) * 0.5;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Check for an intersection with a given bounding box.
    #[inline]
    #[must_use]
    pub fn collides(&self, cube: &Cube) -> bool {
        let c = cube.centre();
        let e = cube.half_widths();

        let v0 = self.verts[0] - c;
        let v1 = self.verts[1] - c;
        let v2 = self.verts[2] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vector3::x_axis();
        let u1 = Vector3::y_axis();
        let u2 = Vector3::z_axis();

        let axis_test = |axis: &Vector3<f64>| {
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

    /// Determine the intersection distance along a Ray's direction.
    /// Also return the barycentric intersection coordinates.
    #[inline]
    #[must_use]
    fn intersection_coors(&self, ray: &Ray) -> Option<(f64, [f64; 3])> {
        let verts = self.verts;

        let e1 = verts[1] - verts[0];
        let e2 = verts[2] - verts[0];

        let d_cross_e2 = ray.dir.cross(&e2);
        let e1_dot_d_cross_e2 = e1.dot(&d_cross_e2);

        if e1_dot_d_cross_e2.abs() <= 0.0 {
            return None;
        }

        let inv_e1_dot_d_cross_e2 = 1.0 / e1_dot_d_cross_e2;
        let rel_pos = ray.pos - verts[0];
        let u = inv_e1_dot_d_cross_e2 * rel_pos.dot(&d_cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = rel_pos.cross(&e1);
        let v = inv_e1_dot_d_cross_e2 * ray.dir.dot(&q);

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

    /// Determine if a Ray-Triangle intersection occurs.
    #[inline]
    #[must_use]
    pub fn hit(&self, ray: &Ray) -> bool {
        self.intersection_coors(ray).is_some()
    }

    /// Determine the distance to a Ray-Triangle intersection.
    #[inline]
    #[must_use]
    pub fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    /// Determine the distance and facing side of a Ray-Triangle intersection.
    #[inline]
    #[must_use]
    pub fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if let Some((dist, [u, v, w])) = self.intersection_coors(ray) {
            Some((
                dist,
                Side::new(
                    &ray.dir,
                    Unit::new_normalize(
                        (self.norms[1].into_inner() * u)
                            + (self.norms[2].into_inner() * v)
                            + (self.norms[0].into_inner() * w),
                    ),
                ),
            ))
        } else {
            None
        }
    }
}
