//! Startup parameters file.

use crate::{
    geom::Grid,
    sim::cartographer::{Engine, Settings},
};

/// Parameter builder structure.
pub struct Parameters {
    /// Measurement grid.
    pub grid: Grid,
    /// Simulation specific settings.
    pub sett: Settings,
    /// Engine function.
    pub engine: Engine,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grid: Grid, sett: Settings, engine: Engine) -> Self {
        Self { grid, sett, engine }
    }
}
