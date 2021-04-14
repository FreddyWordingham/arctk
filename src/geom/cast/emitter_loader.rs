//! Optical material.

use crate::{
    data::Table,
    err::Error,
    fs::{File, Load, Redirect},
    geom::{Emitter, GridBuilder, MeshLoader, Ray},
    math::{Dir3, Pos3},
    ord::{X, Y, Z, Build},
};
use arctk_attr::file;
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

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
    /// Volume map.
    Volume(PathBuf, Redirect<GridBuilder>),
}

impl Load for EmitterLoader {
    type Inst = Emitter;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Beam(pos, dir) => Self::Inst::new_beam(Ray::new(pos, dir)),
            Self::Points(points_path) => {
                let table = Table::new_from_file(&in_dir.join(points_path))?;
                let points = table
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                Self::Inst::new_points(points)
            }
            Self::WeightedPoints(points_path, weight_path) => {
                let points_data = Table::new_from_file(&in_dir.join(points_path))?;
                let points = points_data
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                let weights_data = Table::new_from_file(&in_dir.join(weight_path))?;
                let weights: Vec<_> = weights_data.into_inner().iter().map(|row| row[X]).collect();

                Self::Inst::new_weighted_points(points, &weights)
            }
            Self::Surface(mesh) => Self::Inst::new_surface(mesh.load(in_dir)?),
            Self::Volume(map, grid) => {
                Self::Inst::new_volume(Array3::load(&in_dir.join(map))?, grid.load(in_dir)?.build())
            }
        })
    }
}

impl Display for EmitterLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        let kind = match *self {
            Self::Beam { .. } => "Beam",
            Self::Points { .. } => "Points",
            Self::WeightedPoints { .. } => "WeightedPoints",
            Self::Surface { .. } => "Surface",
            Self::Volume { .. } => "Volume",
        };
        write!(fmt, "{}", kind)
    }
}
