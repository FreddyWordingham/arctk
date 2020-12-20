//! Simulation input.

use crate::sim::cartographer::Settings;

/// Cartographer simulation resources conglomerate.
pub struct Input<'a> {
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: &'a Settings) -> Self {
        Self { sett }
    }
}
