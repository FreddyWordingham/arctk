//! Wavefront.

use nalgebra::{Point3, Unit, Vector3};
use std::{fs, path::Path};

use crate::geom::{Mesh, Triangle};

/// Load a mesh from a wavefront file.
#[inline]
#[must_use]
pub fn load(path: &Path) -> Mesh {
    read(
        &fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}.", path.display())),
    )
}

/// Read a Mesh from a wavefront string.
#[inline]
#[must_use]
pub fn read(s: &str) -> Mesh {
    let verts = read_vertices(s);
    let norms = read_normals(s);
    let faces = read_faces(s);

    let mut tris = Vec::with_capacity(faces.len());
    for face in faces {
        tris.push(Triangle::new(
            [verts[(face.0).0], verts[(face.0).1], verts[(face.0).2]],
            [norms[(face.1).0], norms[(face.1).1], norms[(face.1).2]],
        ));
    }
    Mesh::new(tris)
}

/// Read the vertex list from wavefront string.
#[inline]
#[must_use]
fn read_vertices(s: &str) -> Vec<Point3<f64>> {
    let vert_lines: Vec<_> = s
        .split('\n')
        .filter(|line| line.starts_with("v "))
        .collect();

    let mut verts = Vec::with_capacity(vert_lines.len());
    for line in vert_lines {
        let mut words = line.split_whitespace();
        words.next();

        let px = words
            .next()
            .expect("Missing vertex x value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");
        let py = words
            .next()
            .expect("Missing vertex y value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");
        let pz = words
            .next()
            .expect("Missing vertex z value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");

        verts.push(Point3::new(px, py, pz));
    }

    verts
}

/// Read the normal list from wavefront string.
#[inline]
#[must_use]
fn read_normals(s: &str) -> Vec<Unit<Vector3<f64>>> {
    let norm_lines: Vec<_> = s
        .split('\n')
        .filter(|line| line.starts_with("vn "))
        .collect();

    let mut norms = Vec::with_capacity(norm_lines.len());
    for line in norm_lines {
        let mut words = line.split_whitespace();
        words.next();

        let nx = words
            .next()
            .expect("Missing normal x value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");
        let ny = words
            .next()
            .expect("Missing normal y value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");
        let nz = words
            .next()
            .expect("Missing normal z value.")
            .parse::<f64>()
            .expect("Unable to parse f64 from string.");

        norms.push(Unit::new_normalize(Vector3::new(nx, ny, nz)));
    }

    norms
}

/// Read the face list from wavefront string.
#[allow(clippy::type_complexity)]
#[inline]
#[must_use]
fn read_faces(s: &str) -> Vec<((usize, usize, usize), (usize, usize, usize))> {
    let face_lines: Vec<_> = s
        .split('\n')
        .filter(|line| line.starts_with("f "))
        .collect();

    let mut faces = Vec::with_capacity(face_lines.len());
    for line in face_lines {
        let split_line = line.replace("//", " ");
        let mut words = split_line.split_whitespace();
        words.next();

        let fx = words
            .next()
            .expect("Missing face vertex x value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;
        let nx = words
            .next()
            .expect("Missing face normal x value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;
        let fy = words
            .next()
            .expect("Missing face vertex y value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;
        let ny = words
            .next()
            .expect("Missing face normal y value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;
        let fz = words
            .next()
            .expect("Missing face vertex z value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;
        let nz = words
            .next()
            .expect("Missing face normal z value.")
            .parse::<usize>()
            .expect("Unable to parse f64 from string.")
            - 1;

        faces.push(((fx, fy, fz), (nx, ny, nz)));
    }

    faces
}
