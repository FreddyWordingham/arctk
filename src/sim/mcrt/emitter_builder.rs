//! Optical material.

use super::Emitter;
use crate::{
    data::Table,
    err::Error,
    file::{Build, Load},
    geom::{MeshBuilder, Ray},
    math::{Dir3, Pos3},
    ord::{X, Y, Z},
};
use arctk_attr::load;
use std::path::{Path, PathBuf};

/// Ray emission structure.
#[load]
pub enum EmitterBuilder {
    /// Single beam.
    Beam(Pos3, Dir3),
    /// Point list.
    Points(PathBuf),
    // /// Weighted point list.
    // WeightedPoints(PathBuf, PathBuf),
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
                let table = Table::load(&in_dir.join(path))?;
                let points = table
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                Self::Inst::Points(points)
            }
            Self::Surface(mesh) => Self::Inst::Surface(mesh.build(in_dir)?),
        })
    }
}
