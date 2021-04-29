//! Attribute second-stage spectrometer linker.

use crate::{
    err::Error,
    fmt_report,
    geom::Orient,
    ord::{Link, Name, Set},
    sim::mcrt::AttributeLinker,
    tools::{Binner, Range},
};
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
/// Handles detector linking.
pub enum AttributeLinkerLinker {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer id, range, resolution.
    Spectrometer(Name, [f64; 2], u64),
    /// Imager id, horizontal size, orientation.
    Imager(usize, f64, Orient),
    /// CCD detector id, width, orientation, binner.
    Ccd(usize, f64, Orient, Binner),
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
            Self::Imager(id, width, orient) => Self::Inst::Imager(id, width, orient),
            Self::Ccd(id, width, orient, binner) => Self::Inst::Ccd(id, width, orient, binner),
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
            Self::Imager(ref id, width, ref orient) => {
                writeln!(fmt, "Imager: ...")?;
                fmt_report!(fmt, id, "name");
                fmt_report!(fmt, width, "width (m)");
                fmt_report!(fmt, orient, "orientation");
                Ok(())
            }
            Self::Ccd(ref id, width, ref orient, ref binner) => {
                writeln!(fmt, "Ccd: ...")?;
                fmt_report!(fmt, id, "name");
                fmt_report!(fmt, width, "width (m)");
                fmt_report!(fmt, orient, "orientation");
                fmt_report!(fmt, binner, "binner");
                Ok(())
            }
        }
    }
}
