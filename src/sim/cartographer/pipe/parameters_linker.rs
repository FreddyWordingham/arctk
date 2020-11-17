//! Startup parameters file.

use crate::{
    geom::Grid,
    geom::{GridBuilder, SurfaceLinker, TreeSettings},
    ord::Set,
    sim::cartographer::{Attribute, Engine, Settings},
};

/// Parameter linker structure.
/// Holds references to data loaded in memory.
pub struct ParametersLinker {
    /// Attributes.
    pub attrs: Set<Attribute>,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid.
    pub grid: Grid,
    /// Simulation specific settings.
    pub sett: Settings,
    /// Engine function.
    pub engine: Engine,
}

impl ParametersLinker {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        attrs: Set<Attribute>,
        surfs: Set<SurfaceLinker>,
        tree: TreeSettings,
        grid: Grid,
        sett: Settings,
        engine: Engine,
    ) -> Self {
        Self {
            attrs,
            surfs,
            tree,
            grid,
            sett,
            engine,
        }
    }
}
