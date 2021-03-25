//! Loadable parameters.

use crate::{
    err::Error,
    fs::{File, Load, Redirect},
    geom::{GridBuilder, SurfaceLinkerLoader, TreeSettings},
    math::FormulaBuilder,
    ord::Set,
    phys::{LightLinkerBuilderLoader, MaterialBuilder},
    sim::mcrt::{AttributeLinkerLinkerLinker, EngineBuilder, ParametersBuilder, Settings},
};
use arctk_attr::file;
use ndarray::Array3;
use std::path::{Path, PathBuf};

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceLinkerLoader>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinkerLinkerLinker>>,
    /// Materials.
    mats: Redirect<Set<Redirect<MaterialBuilder>>>,
    /// Main light.
    light: Redirect<LightLinkerBuilderLoader>,
    /// Engine selection.
    engine: EngineBuilder,
    /// Optional fluorophore properties.
    shifts_conc_spec: Option<(PathBuf, FormulaBuilder)>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let tree = self.tree.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;
        let surfs = self.surfs.load(in_dir)?.load(in_dir)?;
        let attrs = self.attrs.load(in_dir)?;
        let mats = self.mats.load(in_dir)?.load(in_dir)?;
        let light = self.light.load(in_dir)?.load(in_dir)?;
        let engine = self.engine;
        let shifts_conc_spec = if let Some((concs, spec)) = self.shifts_conc_spec {
            Some((Array3::new_from_file(&in_dir.join(concs))?, spec))
        } else {
            None
        };

        Ok(Self::Inst::new(
            sett,
            tree,
            grid,
            surfs,
            attrs,
            mats,
            light,
            engine,
            shifts_conc_spec,
        ))
    }
}
