//! Optical surface linking structure.

use crate::{
    err::Error,
    geom::{Mesh, Surface},
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

impl<'a, T: 'a> Link<'a, T> for SurfaceLinker {
    type Inst = Surface<'a, T>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        vec![self.attr.clone()]
    }

    #[inline]
    fn link(self, attrs: &'a Set<T>) -> Result<Self::Inst, Error> {
        Ok(Surface::new(
            self.mesh,
            &attrs.get(&self.attr).expect(&format!(
                "Failed to link surface-attribute key: {}",
                self.attr
            )),
        ))
    }
}
