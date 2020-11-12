//! Smooth triangle-mesh implementation.

use crate::{
    access, clone,
    geom::{Collide, Cube, Emit, Ray, Side, SmoothTriangle, Trace, Transformable},
    math::Trans3,
    ord::{ALPHA, X},
};
use rand::Rng;

/// Boundary padding.
const PADDING: f64 = 1e-6;

/// Mesh geometry.
pub struct Mesh {
    /// Bounding box.
    boundary: Cube,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
    /// Total surface area.
    area: f64,
}

impl Mesh {
    access!(boundary, Cube);
    access!(tris, Vec<SmoothTriangle>);
    clone!(area, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tris: Vec<SmoothTriangle>) -> Self {
        let area = tris.iter().map(|tri| tri.tri().area()).sum();

        Self {
            boundary: Self::init_boundary(&tris),
            tris,
            area,
        }
    }

    /// Initialise the bounding box for the mesh.
    #[inline]
    #[must_use]
    fn init_boundary(tris: &[SmoothTriangle]) -> Cube {
        let mut mins = tris[X].tri().verts()[ALPHA];
        let mut maxs = mins;

        for tri in tris {
            for v in tri.tri().verts().iter() {
                for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                    if *min > *v {
                        *min = *v;
                    } else if *max < *v {
                        *max = *v;
                    }
                }
            }
        }

        for (max, min) in maxs.iter_mut().zip(mins.iter_mut()) {
            *min -= PADDING;
            *max += PADDING;
        }

        Cube::new(mins, maxs)
    }

    /// Destruct the instance and retrieve the list of triangles.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use]
    pub fn into_tris(self) -> Vec<SmoothTriangle> {
        self.tris
    }
}

impl Collide for Mesh {
    #[inline]
    #[must_use]
    fn overlap(&self, cube: &Cube) -> bool {
        if !self.boundary.overlap(cube) {
            return false;
        }

        for tri in &self.tris {
            if tri.overlap(cube) {
                return true;
            }
        }

        false
    }
}

impl Transformable for Mesh {
    #[inline]
    fn transform(&mut self, trans: &Trans3) {
        for tri in &mut self.tris {
            tri.transform(trans);
        }

        self.boundary = Self::init_boundary(&self.tris);
    }
}

impl Emit for Mesh {
    #[inline]
    #[must_use]
    fn cast<R: Rng>(&self, rng: &mut R) -> Ray {
        let r = rng.gen_range(0.0, self.area);
        let mut total_area = 0.0;
        for tri in &self.tris {
            total_area += tri.tri().area();
            if total_area >= r {
                return tri.cast(rng);
            }
        }

        unreachable!()
    }
}

impl Trace for Mesh {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        if !self.boundary.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.boundary.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    #[inline]
    #[must_use]
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if !self.boundary.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_side(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
