//! Optical surface structure.

use crate::{
    err::Error,
    geom::Mesh,
    opt::{Attribute, Surface},
    ord::{Set, Setup},
};

/// Optical surface.
pub struct SurfaceSetup {
    /// Mesh.
    mesh: Mesh,
    /// Attribute name.
    attr: String,
}

impl SurfaceSetup {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mesh: Mesh, attr: String) -> Self {
        Self { mesh, attr }
    }
}

impl<'a> Setup<'a, Attribute<'a>> for SurfaceSetup {
    type Inst = Surface<'a>;

    #[inline]
    fn setup(self, attrs: &'a Set<Attribute>) -> Result<Self::Inst, Error> {
        Ok(Surface::new(self.mesh, &attrs[&self.attr]))
    }
}
