//! Surface linker loading structure.

use crate::{
    err::Error,
    fs::Load,
    geom::{MeshLoader, SurfaceLinker},
    ord::Name,
};
use arctk_attr::file;
use std::path::Path;

/// Optical surface.
#[file]
pub struct SurfaceLinkerLoader {
    /// Mesh.
    mesh: MeshLoader,
    /// Attribute name.
    attr: Name,
}

impl Load for SurfaceLinkerLoader {
    type Inst = SurfaceLinker;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mesh = self.mesh.load(in_dir)?;
        Ok(Self::Inst::new(mesh, self.attr))
    }
}
