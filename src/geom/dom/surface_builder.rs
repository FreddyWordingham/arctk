//! Optical surface structure.

use crate::{
    err::Error,
    file::Build,
    geom::{MeshBuilder, SurfaceLinker},
    ord::Name,
};
use arctk_attr::load;
use std::path::Path;

/// Optical surface.
#[load]
pub struct SurfaceBuilder {
    /// Mesh.
    mesh: MeshBuilder,
    /// Attribute name.
    attr: Name,
}

impl Build for SurfaceBuilder {
    type Inst = SurfaceLinker;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mesh = self.mesh.build(in_dir)?;
        Ok(Self::Inst::new(mesh, self.attr))
    }
}
