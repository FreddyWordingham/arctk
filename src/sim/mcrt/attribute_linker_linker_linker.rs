//! Attribute first-stage imager linker.

use crate::{
    err::Error,
    fmt_report,
    geom::{Orient, Ray},
    math::{Dir3, Pos3, Vec3},
    ord::{Link, Name, Set, X, Y},
    sim::mcrt::AttributeLinkerLinker,
    tools::Range,
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
/// Handles detector linking.
#[file]
pub enum AttributeLinkerLinkerLinker {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer id, range, resolution.
    Spectrometer(Name, [f64; 2], u64),
    /// Imager id, resolution, horizontal width (m), center, forward direction.
    Imager(Name, [usize; 2], f64, Pos3, Vec3),
}

impl<'a> Link<'a, usize> for AttributeLinkerLinkerLinker {
    type Inst = AttributeLinkerLinker;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        vec![]
    }

    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(inside, outside) => Self::Inst::Interface(inside, outside),
            Self::Mirror(r) => Self::Inst::Mirror(r),
            Self::Spectrometer(name, range, resolution) => {
                Self::Inst::Spectrometer(name, range, resolution)
            }
            Self::Imager(id, _resolution, width, center, forward) => Self::Inst::Imager(
                *reg.get(&id)
                    .unwrap_or_else(|| panic!("Failed to link attribute-imager key: {}", id)),
                width,
                Orient::new(Ray::new(center, Dir3::new_normalize(forward))),
            ),
        })
    }
}

impl Display for AttributeLinkerLinkerLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Interface(ref in_mat, ref out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Spectrometer(ref id, [min, max], bins) => {
                write!(
                    fmt,
                    "Spectrometer: {} {} ({})",
                    id,
                    Range::new(min, max),
                    bins
                )
            }
            Self::Imager(ref id, res, width, center, forward) => {
                writeln!(fmt, "Imager: ...")?;
                fmt_report!(fmt, id, "name");
                fmt_report!(fmt, &format!("[{} x {}]", res[X], res[Y]), "resolution");
                fmt_report!(fmt, width, "width (m)");
                fmt_report!(fmt, center, "center (m)");
                fmt_report!(fmt, forward, "forward");
                Ok(())
            }
        }
    }
}
