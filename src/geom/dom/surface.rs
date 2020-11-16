//! Optical surface structure.

use crate::{access, geom::Mesh};

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
