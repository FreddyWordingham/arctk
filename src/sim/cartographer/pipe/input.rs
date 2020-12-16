//! Simulation input.

use crate::{geom::Grid, sim::cartographer::Settings};

/// Cartographer simulation resources conglomerate.
pub struct Input<'a> {
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(grid: &'a Grid, sett: &'a Settings) -> Self {
        Self { grid, sett }
    }
}
