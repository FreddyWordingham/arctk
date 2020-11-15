//! Optical surface structure.

use crate::{
    geom::Mesh,
    opt::{Attribute, Surface},
    ord::Set,
};

/// Optical surface.
pub struct SurfaceSetup {
    /// Mesh.
    mesh: Mesh,
    /// Attribute name.
    attr: String,
}

impl SurfaceSetup {
    /// Setup the attribute link.
    #[must_use]
    #[inline]
    fn setup<'a>(self, attrs: &'a Set<Attribute>) -> Surface<'a> {
        Surface::new(self.mesh, &attrs[&self.attr])
    }
}
