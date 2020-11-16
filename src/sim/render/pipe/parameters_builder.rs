//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{SurfaceBuilder, TreeSettings},
    img::GradientBuilder,
    ord::Set,
    sim::render::{AttributeLinker, CameraBuilder, EngineBuilder, ParametersSetup, SettingsLinker},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds references to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Colour gradients.
    grads: Redirect<Set<GradientBuilder>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinker>>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceBuilder>>,
    /// Main camera.
    cam: Redirect<CameraBuilder>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Rendering specific settings.
    sett: Redirect<SettingsLinker>,
    /// Engine selection.
    engine: EngineBuilder,
}

impl Build for ParametersBuilder {
    type Inst = ParametersSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<ParametersSetup, Error> {
        let grads = self.grads.build(in_dir)?.build(in_dir)?;
        let attrs = self.attrs.build(in_dir)?;
        let surfs = self.surfs.build(in_dir)?.build(in_dir)?;
        let cam = self.cam.build(in_dir)?.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let engine = self.engine.build(in_dir)?;

        Ok(ParametersSetup::new(
            grads, attrs, surfs, cam, tree, sett, engine,
        ))
    }
}
