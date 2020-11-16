//! Optical surface linking structure.

use crate::{
    err::Error,
    geom::Mesh,
    opt::{Attribute, Surface},
    ord::{Link, Set},
};

/// Optical surface.
pub struct SurfaceLinker {
    /// Mesh.
    mesh: Mesh,
    /// Attribute name.
    attr: String,
}

impl SurfaceLinker {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mesh: Mesh, attr: String) -> Self {
        Self { mesh, attr }
    }
}

impl<'a> Link<'a, Attribute<'a>> for SurfaceLinker {
    type Inst = Surface<'a>;

    #[inline]
    fn link(self, attrs: &'a Set<Attribute>) -> Result<Self::Inst, Error> {
        Ok(Surface::new(self.mesh, &attrs[&self.attr]))
    }
}
