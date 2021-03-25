//! Runtime parameters.

use crate::{
    fmt_report,
    geom::{Grid, SurfaceLinker, TreeSettings},
    math::Formula,
    ord::Set,
    phys::{LightLinker, Material},
    sim::mcrt::{AttributeLinkerLinkerLinker, Engine, Settings},
    util::Analyze,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid settings.
    pub grid: Grid,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Attributes.
    pub attrs: Set<AttributeLinkerLinkerLinker>,
    /// Materials.
    pub mats: Set<Material>,
    /// Main light.
    pub light: LightLinker,
    /// Engine selection.
    pub engine: Engine,
    /// Optional fluorophore properties.
    pub shifts_conc_spec: Option<(Array3<f64>, Formula)>,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: Grid,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinkerLinkerLinker>,
        mats: Set<Material>,
        light: LightLinker,
        engine: Engine,
        shifts_conc_spec: Option<(Array3<f64>, Formula)>,
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
        }
    }
}

impl Display for Parameters {
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
        fmt_report!(fmt, "{* POINTER LOADED *}", "engine");
        if let Some(shifts_conc_spec) = &self.shifts_conc_spec {
            fmt_report!(fmt, shifts_conc_spec.0.display(), "Fluorophore map");
            fmt_report!(fmt, shifts_conc_spec.1, "Fluorophore absorption spectra");
        }
        Ok(())
    }
}
