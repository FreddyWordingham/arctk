//! Optical surface linking structure.

use crate::{
    err::Error,
    geom::{Mesh, Surface},
    ord::{Link, Name, Set},
};

/// Optical surface.
pub struct SurfaceLinker {
    /// Mesh.
    mesh: Mesh,
    /// Attribute name.
    attr: Name,
}

impl SurfaceLinker {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mesh: Mesh, attr: Name) -> Self {
        Self { mesh, attr }
    }
}

impl<'a, T: 'a> Link<'a, T> for SurfaceLinker {
    type Inst = Surface<'a, T>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        vec![self.attr.clone()]
    }

    #[inline]
    fn link(self, attrs: &'a Set<T>) -> Result<Self::Inst, Error> {
        let attr = self.attr;
        Ok(Surface::new(
            self.mesh,
            attrs
                .get(&attr)
                .unwrap_or_else(|| panic!("Failed to link surface-attribute key: {}", attr)),
        ))
    }
}
