//! Wavefront file handling.

use crate::{
    err::Error,
    file::Load,
    geom::{Mesh, SmoothTriangle},
    math::{Dir3, Pos3, Vec3},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
