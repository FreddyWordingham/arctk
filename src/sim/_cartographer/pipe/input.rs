//! Simulation input.

use crate::{
    geom::{Grid, Tree},
    ord::{Register, Set},
    sim::{cartographer::Settings, mcrt::AttributeLinker},
};

/// Cartographer simulation resources conglomerate.
pub struct Input<'a> {
    /// Material register.
    pub mat_reg: &'a Register,
    /// Attributes.
    pub attrs: &'a Set<AttributeLinker>,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, AttributeLinker>,
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
        attrs: &'a Set<AttributeLinker>,
        tree: &'a Tree<AttributeLinker>,
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
