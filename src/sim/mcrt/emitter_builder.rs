//! Optical material.

use super::Emitter;
use crate::{
    err::Error,
    file::Build,
    geom::{MeshBuilder, Ray},
    math::{Dir3, Pos3},
};
use arctk_attr::load;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

/// Ray emission structure.
#[load]
pub enum EmitterBuilder {
    /// Single beam.
    Beam(Pos3, Dir3),
    /// Point list.
    Points(PathBuf),
    /// Surface mesh.
    Surface(MeshBuilder),
}

impl Build for EmitterBuilder {
    type Inst = Emitter;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Beam(pos, dir) => Self::Inst::Beam(Ray::new(pos, dir)),
            Self::Points(path) => {
                let path = in_dir.join(path);
                println!("loading: {}", path.display());

                let lines: Vec<_> = BufReader::new(File::open(path)?)
                    .lines()
                    .map(Result::unwrap)
                    .filter(|line| !line.starts_with("//"))
                    .collect();

                let mut points = Vec::with_capacity(lines.len());
                for mut line in lines {
                    line.retain(|c| !c.is_whitespace());
                    let mut words = line.split(',');

                    let x = words
                        .next()
                        .ok_or("Missing position word x.")?
                        .parse::<f64>()?;
                    let y = words
                        .next()
                        .ok_or("Missing position word y.")?
                        .parse::<f64>()?;
                    let z = words
                        .next()
                        .ok_or("Missing position word z.")?
                        .parse::<f64>()?;

                    points.push(Pos3::new(x, y, z));
                }

                Self::Inst::Points(points)
            }
            Self::Surface(mesh) => Self::Inst::Surface(mesh.build(in_dir)?),
        })
    }
}
