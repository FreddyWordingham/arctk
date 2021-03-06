//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{CameraBuilder, SurfaceLinkerLoader, TreeSettings},
    img::GradientBuilder,
    ord::Set,
    sim::render::{AttributeLinker, EngineBuilder, ParametersBuilder, Settings, ShaderLinker},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Rendering specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceLinkerLoader>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinker>>,
    /// Colour gradients.
    grads: Redirect<Set<GradientBuilder>>,
    /// Main camera.
    cam: Redirect<CameraBuilder>,
    /// Shader settings.
    shader: Redirect<ShaderLinker>,
    /// Engine selection.
    engine: EngineBuilder,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let tree = self.tree.load(in_dir)?;
        let surfs = self.surfs.load(in_dir)?.load(in_dir)?;
        let attrs = self.attrs.load(in_dir)?;
        let grads = self.grads.load(in_dir)?;
        let cam = self.cam.load(in_dir)?;
        let shader = self.shader.load(in_dir)?;
        let engine = self.engine;

        Ok(Self::Inst::new(
            sett, tree, surfs, attrs, grads, cam, shader, engine,
        ))
    }
}
