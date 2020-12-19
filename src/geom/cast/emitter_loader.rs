//! Optical material.

use crate::{
    data::Table,
    err::Error,
    fs::{File, Load},
    geom::{Emitter, MeshLoader, Ray},
    math::{Dir3, Pos3},
    ord::{X, Y, Z},
};
use arctk_attr::file;
use std::path::{Path, PathBuf};

/// Ray emission structure.
#[file]
pub enum EmitterLoader {
    /// Single beam.
    Beam(Pos3, Dir3),
    /// Point list.
    Points(PathBuf),
    /// Weighted point list.
    WeightedPoints(PathBuf, PathBuf),
    /// Surface mesh.
    Surface(MeshLoader),
}

impl Load for EmitterLoader {
    type Inst = Emitter;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Beam(pos, dir) => Self::Inst::new_beam(Ray::new(pos, dir)),
            Self::Points(points_path) => {
                let table = Table::load(&in_dir.join(points_path))?;
                let points = table
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                Self::Inst::new_points(points)
            }
            Self::WeightedPoints(points_path, weight_path) => {
                let points_data = Table::load(&in_dir.join(points_path))?;
                let points = points_data
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                let weights_data = Table::load(&in_dir.join(weight_path))?;
                let weights: Vec<_> = weights_data.into_inner().iter().map(|row| row[X]).collect();

                Self::Inst::new_weighted_points(points, &weights)
            }
            Self::Surface(mesh) => Self::Inst::new_surface(mesh.load(in_dir)?),
        })
    }
}
