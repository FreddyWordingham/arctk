//! Attribute first-stage linker.

use crate::{
    err::Error,
    fmt_report,
    math::Vec3,
    ord::{Link, Name, Set, X, Y},
    sim::mcrt::AttributeLinker,
    tools::Range,
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
/// Handles detector linking.
#[file]
pub enum AttributeLinkerLinker {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer id, range, resolution.
    Spectrometer(Name, [f64; 2], u64),
    /// Imager id, horizontal size, resolution, forward direction.
    Imager(Name, f64, [u64; 2], Vec3),
}

impl AttributeLinkerLinker {
    /// Get a list of spectrometer names used.
    #[must_use]
    #[inline]
    fn requires_spectrometers(&self) -> Vec<Name> {
        match *self {
            Self::Spectrometer(ref id, ..) => vec![id.clone()],
            _ => vec![],
        }
    }

    /// Get a list of imager names used.
    #[must_use]
    #[inline]
    fn requires_imagers(&self) -> Vec<Name> {
        match *self {
            Self::Imager(ref id, ..) => vec![id.clone()],
            _ => vec![],
        }
    }
}

impl<'a> Link<'a, usize> for AttributeLinkerLinker {
    type Inst = AttributeLinker;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        vec![]
    }

    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(inside, outside) => Self::Inst::Interface(inside, outside),
            Self::Mirror(r) => Self::Inst::Mirror(r),
            Self::Spectrometer(id, ..) => Self::Inst::Spectrometer(
                *reg.get(&id)
                    .unwrap_or_else(|| panic!("Failed to link attribute-spectrometer key: {}", id)),
            ),
            Self::Imager(id, ..) => Self::Inst::Spectrometer(
                *reg.get(&id)
                    .unwrap_or_else(|| panic!("Failed to link attribute-imager key: {}", id)),
            ),
        })
    }
}

impl Display for AttributeLinkerLinker {
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
            Self::Imager(ref id, width, res, dir) => {
                writeln!(fmt, "Imager: ...")?;
                fmt_report!(fmt, id, "name");
                fmt_report!(fmt, width, "width (m)");
                fmt_report!(fmt, &format!("[{} x {}]", res[X], res[Y]), "resolution");
                fmt_report!(fmt, dir, "direction");
                Ok(())
            }
        }
    }
}
