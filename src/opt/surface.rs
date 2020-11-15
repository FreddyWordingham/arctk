//! Optical surface structure.

use crate::{access, geom::Mesh, opt::Attribute};

/// Optical surface.
pub struct Surface<'a> {
    /// Mesh.
    mesh: Mesh,
    /// Attribute.
    attr: &'a Attribute<'a>,
}

impl<'a> Surface<'a> {
    access!(mesh, Mesh);
    access!(attr, Attribute);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(mesh: Mesh, attr: &'a Attribute) -> Self {
        Self { mesh, attr }
    }
}
