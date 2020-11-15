//! Optical surface structure.

use crate::{
    geom::MeshBuilder,
    opt::{Attribute, Surface},
    ord::Set,
};
use arctk_attr::load;

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
        let mesh = self.mesh.build(in_dir);
        Ok(Self::Inst::new(mesh, self.attr))
    }
}
