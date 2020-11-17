//! Simulation input.

use crate::{
    geom::{Grid, Tree},
    ord::{Register, Set},
    sim::cartographer::{Attribute, Settings},
};

/// Cartographer simulation resources conglomerate.
pub struct Input<'a> {
    /// Material register.
    pub mat_reg: &'a Register,
    /// Attributes.
    pub attrs: &'a Set<Attribute>,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, Attribute>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        mat_reg: &'a Register,
        attrs: &'a Set<Attribute>,
        tree: &'a Tree<Attribute>,
        grid: &'a Grid,
        sett: &'a Settings,
    ) -> Self {
        Self {
            mat_reg,
            attrs,
            tree,
            grid,
            sett,
        }
    }
}
