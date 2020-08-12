//! Smooth triangle-mesh implementation.

use crate::{
    access, clone, display_field, display_field_ln, Aabb, Collide, Dir3, Emit, Error, Load, Pos3,
    Ray, Side, SmoothTriangle, Trace, Trans3, Transform, Vec3, ALPHA, X,
};
use rand::{rngs::ThreadRng, Rng};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result::Result,
};

/// Boundary padding.
const PADDING: f64 = 1e-6;

/// Mesh geometry.
pub struct Mesh {
    /// Bounding box.
    boundary: Aabb,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
    /// Total surface area.
    area: f64,
}

impl Mesh {
    access!(boundary, Aabb);
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
    fn init_boundary(tris: &[SmoothTriangle]) -> Aabb {
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

        Aabb::new(mins, maxs)
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
    fn overlap(&self, aabb: &Aabb) -> bool {
        if !self.boundary.overlap(aabb) {
            return false;
        }

        for tri in &self.tris {
            if tri.overlap(aabb) {
                return true;
            }
        }

        false
    }
}

impl Transform for Mesh {
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
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
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

impl Load for Mesh {
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        println!("loading: {}", path.display());
        let vertex_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("v "))
            .collect();

        let mut verts = Vec::with_capacity(vertex_lines.len());
        for line in vertex_lines {
            let mut words = line.split_whitespace();
            words.next();

            let px = words.next().ok_or("Missing vertex word.")?.parse::<f64>()?;
            let py = words.next().ok_or("Missing vertex word.")?.parse::<f64>()?;
            let pz = words.next().ok_or("Missing vertex word.")?.parse::<f64>()?;

            verts.push(Pos3::new(px, py, pz));
        }

        let normal_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("vn "))
            .collect();

        let mut norms = Vec::with_capacity(normal_lines.len());
        for line in normal_lines {
            let mut words = line.split_whitespace();
            words.next();

            let nx = words.next().ok_or("Missing normal word.")?.parse::<f64>()?;
            let ny = words.next().ok_or("Missing normal word.")?.parse::<f64>()?;
            let nz = words.next().ok_or("Missing normal word.")?.parse::<f64>()?;

            norms.push(Dir3::new_normalize(Vec3::new(nx, ny, nz)));
        }

        let face_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("f "))
            .collect();

        let mut faces = Vec::with_capacity(face_lines.len());
        for line in face_lines {
            let line = line.replace("//", " ");
            let mut words = line.split_whitespace();
            words.next();

            let fx = words.next().ok_or("Missing face word.")?.parse::<usize>()? - 1;
            let nx = words
                .next()
                .ok_or("Missing normal word.")?
                .parse::<usize>()?
                - 1;
            let fy = words.next().ok_or("Missing face word.")?.parse::<usize>()? - 1;
            let ny = words
                .next()
                .ok_or("Missing normal word.")?
                .parse::<usize>()?
                - 1;
            let fz = words.next().ok_or("Missing face word.")?.parse::<usize>()? - 1;
            let nz = words
                .next()
                .ok_or("Missing normal word.")?
                .parse::<usize>()?
                - 1;

            faces.push(((fx, fy, fz), (nx, ny, nz)));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(SmoothTriangle::new_from_verts(
                [verts[(face.0).0], verts[(face.0).1], verts[(face.0).2]],
                [norms[(face.1).0], norms[(face.1).1], norms[(face.1).2]],
            ));
        }

        Ok(Self::new(tris))
    }
}

impl Display for Mesh {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "number of triangles", self.tris.len())?;
        display_field_ln!(fmt, "area", self.area, "m^2")?;
        display_field!(fmt, "boundary", &self.boundary)
    }
}
