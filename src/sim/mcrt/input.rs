//! Simulation input.

use crate::{geom::Grid, opt::Light, sim::mcrt::Settings};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// General settings.
    pub sett: &'a Settings,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// Emission light.
    pub light: &'a Light,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: &'a Settings, grid: &'a Grid, light: &'a Light) -> Self {
        Self { sett, grid, light }
    }
}
