//! Surface.

use crate::geom::Mesh;

/// Three-dimension triangular mesh with attribute data.
pub struct Surface<'a, T> {
    /// Mesh.
    pub mesh: Mesh,
    /// Attribute data object.
    pub attr: &'a T,
}

impl<'a, T> Surface<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mesh: Mesh, attr: &'a T) -> Self {
        Self { mesh, attr }
    }
}
