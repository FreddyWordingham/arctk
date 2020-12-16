//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::GridBuilder,
    sim::cartographer::{EngineBuilder, Parameters, Settings},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds paths to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Engine selection.
    engine: EngineBuilder,
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let grid = self.grid.build(in_dir)?.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let engine = self.engine.build(in_dir)?;

        Ok(Self::Inst::new(grid, sett, engine))
    }
}
