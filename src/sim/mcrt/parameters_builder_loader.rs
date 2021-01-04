//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{CameraBuilder, SurfaceLinkerLoader, TreeSettings},
    img::GradientBuilder,
    ord::Set,
    sim::mcrt::{
        AttributeLinker, EngineBuilder, LightBuilder, MaterialBuilder, ParametersBuilder, Settings,
        Settings,
    },
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceLinkerLoader>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinker>>,
    /// Materials.
    mats: Redirect<Set<MaterialBuilder>>,
    /// Main light.
    light: Redirect<LightBuilder>,
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
        let mats = self.mats.load(in_dir)?;
        let light = self.light.load(in_dir)?;
        let engine = self.engine;

        Ok(Self::Inst::new(
            sett, tree, surfs, attrs, mats, light, engine,
        ))
    }
}
