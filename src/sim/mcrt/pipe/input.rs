//! Simulation input.

use crate::{
    geom::{Grid, Tree},
    opt::{Attribute, Light, Material},
    ord::Set,
    sim::mcrt::Settings,
};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Hit-scan tree.
    pub tree: &'a Tree<'a>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings<'a>,
    /// Materials.
    pub mats: &'a Set<Material>,
    /// Attributes.
    pub attrs: &'a Set<Attribute<'a>>,
    /// Emission light.
    pub light: &'a Light,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Tree,
        grid: &'a Grid,
        sett: &'a Settings,
        mats: &'a Set<Material>,
        attrs: &'a Set<Attribute>,
        light: &'a Light,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            mats,
            attrs,
            light,
        }
    }
}
