//! Buildable parameters.

use crate::{
    fmt_report,
    geom::{GridBuilder, SurfaceLinker, TreeSettings},
    math::{FormulaBuilder, Pos3},
    ord::{Build, Set},
    phys::{LightLinkerBuilder, MaterialBuilder},
    sim::mcrt::{AttributeLinkerLinkerLinker, EngineBuilder, Parameters, Settings},
    util::Analyze,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Buildable runtime parameters.
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Tree settings.
    tree: TreeSettings,
    /// Measurement grid settings.
    grid: GridBuilder,
    /// Surfaces.
    surfs: Set<SurfaceLinker>,
    /// Attributes.
    attrs: Set<AttributeLinkerLinkerLinker>,
    /// Materials.
    mats: Set<MaterialBuilder>,
    /// Main light.
    light: LightLinkerBuilder,
    /// Engine selection.
    engine: EngineBuilder,
    /// Optional fluorophore properties.
    shifts_conc_spec: Option<(Array3<f64>, FormulaBuilder)>,
    /// Optional camera position.
    cam_pos: Option<Pos3>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: GridBuilder,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinkerLinkerLinker>,
        mats: Set<MaterialBuilder>,
        light: LightLinkerBuilder,
        engine: EngineBuilder,
        shifts_conc_spec: Option<(Array3<f64>, FormulaBuilder)>,
        cam_pos: Option<Pos3>,
    ) -> Self {
        Self {
            sett,
            tree,
            grid,
            surfs,
            attrs,
            mats,
            light,
            engine,
            shifts_conc_spec,
            cam_pos,
        }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let tree = self.tree;
        let grid = self.grid.build();
        let surfs = self.surfs;
        let attrs = self.attrs;
        let mats = self.mats.build();
        let light = self.light.build();
        let engine = self.engine.build();
        let shifts_conc_spec = if let Some((concs, spec)) = self.shifts_conc_spec {
            Some((concs, spec.build()))
        } else {
            None
        };
        let cam_pos = self.cam_pos;

        Self::Inst::new(
            sett,
            tree,
            grid,
            surfs,
            attrs,
            mats,
            light,
            engine,
            shifts_conc_spec,
            cam_pos,
        )
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.grid, "grid settings");
        fmt_report!(fmt, self.surfs, "surfaces");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.mats, "materials");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, self.engine, "engine");
        if let Some((concs, spec)) = &self.shifts_conc_spec {
            fmt_report!(fmt, concs.display(), "Fluorophore map");
            fmt_report!(fmt, spec, "Fluorophore absorption spectra");
        }
        if let Some(cam_pos) = &self.cam_pos {
            fmt_report!(fmt, cam_pos, "camera position (m)");
        }
        Ok(())
    }
}
