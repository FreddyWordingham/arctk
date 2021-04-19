//! Optical surface structure.

use crate::{access, fmt_report, geom::Mesh};
use std::fmt::{Display, Error, Formatter};

/// Optical surface.
pub struct Surface<'a, T> {
    /// Mesh.
    mesh: Mesh,
    /// Attribute.
    attr: &'a T,
}

impl<'a, T> Surface<'a, T> {
    access!(mesh, Mesh);
    access!(attr, T);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mesh: Mesh, attr: &'a T) -> Self {
        Self { mesh, attr }
    }
}

impl<T: Display> Display for Surface<'_, T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.mesh, "mesh");
        fmt_report!(fmt, self.attr, "attribute");
        Ok(())
    }
}
