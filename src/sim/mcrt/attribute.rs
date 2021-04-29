//! Optical attributes.

use crate::{fmt_report, geom::Orient, phys::Material, tools::Binner};
use std::fmt::{Display, Error, Formatter};

/// Surface attributes.
pub enum Attribute<'a> {
    /// Material interface, inside material reference, outside material reference.
    Interface(&'a Material, &'a Material),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer detector.
    Spectrometer(usize),
    /// Imager detector id, width, orientation.
    Imager(usize, f64, Orient),
    /// CCD detector id, width, orientation, binner.
    Ccd(usize, f64, Orient, Binner),
}

impl Display for Attribute<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Interface(in_mat, out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Spectrometer(id) => {
                write!(fmt, "Spectrometer: {}", id)
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
                fmt_report!(fmt, binner, "binner (m)");
                Ok(())
            }
        }
    }
}
