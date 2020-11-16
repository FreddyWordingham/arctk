//! Optical surface structure.

use crate::{err::Error, file::Build, geom::MeshBuilder, opt::SurfaceSetup};
use arctk_attr::load;
use std::path::Path;

/// Optical surface.
#[load]
pub struct SurfaceBuilder {
    /// Mesh.
    mesh: MeshBuilder,
    /// Attribute name.
    attr: String,
}

impl Build for SurfaceBuilder {
    type Inst = SurfaceSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mesh = self.mesh.build(in_dir)?;
        Ok(Self::Inst::new(mesh, self.attr))
    }
}
